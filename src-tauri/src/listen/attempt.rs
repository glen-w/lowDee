//! Attempt engine: hop frames → evidence at end. Target frozen for the attempt.

use serde::{Deserialize, Serialize};

use super::evidence::Evidence;
use super::fingering::{self, nearest_scale_note};
use super::ornaments::{self, ContourPoint, GestureKind};
use super::pitch::{self, PitchEstimate};
use super::rms;
use super::types::{
    target_hz, NoteName, WhistleProfile, CENTS_TOLERANCE_POST_CAL, CENTS_TOLERANCE_PRE_CAL,
    DEFAULT_LOW_D_HZ, SETTLE_SECONDS,
};

const PHRASE_DWELL_SECS: f32 = 0.12;
/// Quiet after the line or the hold is already done. Longer than a breath gap.
const STOP_AFTER_GOAL_SECS: f32 = 0.55;
/// Quiet when they stop before the goal. Longer than a breath in the long low-D hold.
const STOP_ABANDON_SECS: f32 = 1.8;
/// Quiet after an ornament. Longer than the gap inside a cut or a roll.
const STOP_ORNAMENT_SECS: f32 = 0.7;
const STOP_HEARD_FRAMES: usize = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub t_ms: f32,
    pub hz: Option<f32>,
    pub rms: f32,
    pub confidence: f32,
    pub near_target: bool,
    pub early_break: bool,
    pub expected_note: Option<String>,
    pub phrase_index: usize,
    pub hold_ratio: f32,
    /// Set only when the heard scale note differs from the expected one by a single hole.
    pub leak_hole: Option<u8>,
    /// The player has stopped. Latched once, after the attempt has been heard.
    #[serde(default)]
    pub stopped: bool,
}

#[derive(Debug, Clone)]
pub struct PhraseMark {
    pub note_index: usize,
    pub gesture: GestureKind,
}

#[derive(Debug, Clone)]
pub enum AttemptMode {
    /// Hold low D until settle (~10s). Writes profile inputs on success.
    FirstSound,
    /// Stay on low note; early break → EarlyBreak.
    BreathOctave { want_octave: bool },
    /// Single expected note.
    SingleNote { note: NoteName },
    /// Phrase of expected notes (order).
    /// `breaths` are note indexes after which a gap belongs. Empty: gaps are ignored.
    /// `marks` are graded when the phrase arrives. A messy contour abstains.
    Phrase {
        notes: Vec<NoteName>,
        breaths: Vec<usize>,
        marks: Vec<PhraseMark>,
    },
    /// Joined notes; detect chronic restarts (many onsets).
    OnTheBreath { notes: Vec<NoteName> },
    /// Ornament on one long note.
    Ornament {
        note: NoteName,
        gesture: GestureKind,
    },
}

#[derive(Debug, Clone)]
pub struct AttemptConfig {
    pub sample_rate: u32,
    pub hop_ms: f32,
    pub mode: AttemptMode,
    /// Frozen for this attempt — never updated mid-phrase.
    pub break_hz: Option<f32>,
    pub rms_floor: Option<f32>,
    /// A new low-D hold may sit sharp of the stored break. The target stays frozen.
    pub recalibrate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostPoint {
    pub t_ms: f32,
    pub cents: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostSpan {
    pub t0_ms: f32,
    pub t1_ms: f32,
    pub note: String,
    /// Expected interval, in cents, from the frozen target.
    pub cents: f32,
    /// cut, tap, roll, and the later gestures. Empty when the note is bare.
    #[serde(default)]
    pub mark: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostTrace {
    pub points: Vec<GhostPoint>,
    pub spans: Vec<GhostSpan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptResult {
    pub evidence: Evidence,
    pub settled: bool,
    pub break_hz: Option<f32>,
    pub rms_floor: Option<f32>,
    pub frames: Vec<Frame>,
    pub target_hz: Option<f32>,
    pub remark_note: Option<String>,
    /// A later hold sat outside the post-calibration window. Not a tuner.
    pub warm: bool,
    /// C natural was asked, the pitch was not that interval, and no single hole explains it.
    pub cnat_disagree: bool,
    /// Downsampled contour. Absent when the attempt could not be heard, or the ornament abstains.
    pub ghost: Option<GhostTrace>,
    /// The pitch a phrase fault names, so the screen can open that note alone.
    pub isolate_note: Option<String>,
    /// Heard audio for this attempt. Absent when the attempt could not be heard, or the ornament abstains.
    #[serde(skip)]
    pub take_wav: Option<TakeWav>,
}

/// In-memory wav of one attempt. Debug prints the length, not the samples.
#[derive(Clone)]
pub struct TakeWav(pub Vec<u8>);

impl std::fmt::Debug for TakeWav {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "take({} bytes)", self.0.len())
    }
}

/// The last take the screen may play. Dropped with the attempt. Not stored.
#[derive(Debug, Default)]
pub struct TakeSlot {
    wav: Option<Vec<u8>>,
}

impl TakeSlot {
    pub fn store(&mut self, wav: Option<TakeWav>) {
        self.wav = wav.map(|taken| taken.0);
    }

    pub fn drop_take(&mut self) {
        self.wav = None;
    }

    pub fn bytes(&self) -> Option<&[u8]> {
        self.wav.as_deref()
    }
}

const TAKE_SECONDS: u32 = 30;

fn append_take(take: &mut Vec<f32>, incoming: &[f32], cap: usize) {
    if take.len() >= cap || incoming.is_empty() {
        return;
    }
    let room = cap - take.len();
    let n = incoming.len().min(room);
    take.extend_from_slice(&incoming[..n]);
}

pub struct AttemptEngine {
    cfg: AttemptConfig,
    frames: Vec<Frame>,
    contour: Vec<ContourPoint>,
    hold_secs: f32,
    hold_hz_acc: f32,
    hold_rms_acc: f32,
    hold_n: u32,
    early_break_latched: bool,
    onset_count: u32,
    was_quiet: bool,
    phrase_idx: usize,
    /// How many phrase notes have been held, in order, long enough to count.
    notes_confirmed: usize,
    /// Contour index where each phrase note began.
    slice_starts: Vec<usize>,
    /// Note indexes where a quiet gap was heard.
    breaths_heard: Vec<usize>,
    gap_secs: f32,
    /// Raw-pitch frames that sat on the lower octave of a high target.
    crack_low: u32,
    phrase_dwell: f32,
    off_secs: f32,
    /// Trailing quiet after the attempt has been heard.
    quiet_secs: f32,
    /// Stop has latched. Later frames keep it.
    stopped: bool,
    samples_buf: Vec<f32>,
    /// Mono samples for Hear that. Capped. Not the sliding pitch window.
    take: Vec<f32>,
    hop_samples: usize,
    t_ms: f32,
    /// Snapshot of target at start — frozen.
    frozen_target: Option<f32>,
}

impl AttemptEngine {
    pub fn new(cfg: AttemptConfig) -> Self {
        let hop_samples = ((cfg.sample_rate as f32) * cfg.hop_ms / 1000.0).round() as usize;
        let frozen_target = match &cfg.mode {
            AttemptMode::FirstSound => Some(cfg.break_hz.unwrap_or(DEFAULT_LOW_D_HZ)),
            AttemptMode::BreathOctave { want_octave } => {
                let base = cfg.break_hz.unwrap_or(DEFAULT_LOW_D_HZ);
                Some(if *want_octave { base * 2.0 } else { base })
            }
            AttemptMode::SingleNote { note } | AttemptMode::Ornament { note, .. } => {
                let base = cfg.break_hz.unwrap_or(DEFAULT_LOW_D_HZ);
                Some(target_hz(base, *note))
            }
            AttemptMode::Phrase { notes, .. } | AttemptMode::OnTheBreath { notes } => notes
                .first()
                .map(|n| target_hz(cfg.break_hz.unwrap_or(DEFAULT_LOW_D_HZ), *n)),
        };
        Self {
            cfg,
            frames: Vec::new(),
            contour: Vec::new(),
            hold_secs: 0.0,
            hold_hz_acc: 0.0,
            hold_rms_acc: 0.0,
            hold_n: 0,
            early_break_latched: false,
            onset_count: 0,
            was_quiet: true,
            phrase_idx: 0,
            notes_confirmed: 0,
            slice_starts: Vec::new(),
            breaths_heard: Vec::new(),
            gap_secs: 0.0,
            crack_low: 0,
            phrase_dwell: 0.0,
            off_secs: 0.0,
            quiet_secs: 0.0,
            stopped: false,
            samples_buf: Vec::new(),
            take: Vec::new(),
            hop_samples: hop_samples.max(256),
            t_ms: 0.0,
            frozen_target,
        }
    }

    pub fn frozen_target_hz(&self) -> Option<f32> {
        self.frozen_target
    }

    pub fn push_samples(&mut self, samples: &[f32]) -> Option<Frame> {
        let cap = (self.cfg.sample_rate as usize).saturating_mul(TAKE_SECONDS as usize);
        append_take(&mut self.take, samples, cap);
        self.samples_buf.extend_from_slice(samples);
        let mut last = None;
        while self.samples_buf.len() >= self.hop_samples * 2 {
            // Use 2 hops of context for YIN stability
            let window: Vec<f32> = self.samples_buf[..self.hop_samples * 2].to_vec();
            let frame = self.process_window(&window);
            self.samples_buf.drain(..self.hop_samples);
            self.t_ms += self.cfg.hop_ms;
            last = Some(frame);
        }
        last
    }

    fn process_window(&mut self, window: &[f32]) -> Frame {
        let energy = rms::rms(window);
        let floor = self.cfg.rms_floor.unwrap_or(0.01);
        let sounding = rms::is_sounding(energy, floor);

        let expected = self.current_expected_hz();
        let tol = if self.cfg.break_hz.is_some() {
            CENTS_TOLERANCE_POST_CAL
        } else {
            CENTS_TOLERANCE_PRE_CAL
        };

        let est: PitchEstimate = if sounding {
            if let Some(exp) = expected {
                pitch::search_near_expected(window, self.cfg.sample_rate, exp, tol)
            } else {
                pitch::estimate_pitch(window, self.cfg.sample_rate)
            }
        } else {
            PitchEstimate {
                hz: None,
                confidence: 0.0,
            }
        };

        // Raw pitch (no octave fold) — needed to hear an early break that
        // score-guided tracking would otherwise map back to the low target.
        let raw = if sounding {
            pitch::estimate_pitch(window, self.cfg.sample_rate)
        } else {
            PitchEstimate {
                hz: None,
                confidence: 0.0,
            }
        };

        let mut early_break = false;
        if let Some(raw_hz) = raw.hz {
            let base = self.cfg.break_hz.unwrap_or(DEFAULT_LOW_D_HZ);
            if matches!(
                self.cfg.mode,
                AttemptMode::FirstSound | AttemptMode::BreathOctave { want_octave: false }
            ) {
                let oct = base * 2.0;
                let cents_oct = 1200.0 * (raw_hz / oct).log2().abs();
                if cents_oct < 80.0 && energy > floor {
                    early_break = true;
                    self.early_break_latched = true;
                }
            }
            if let Some(note) = self.current_expected_note() {
                if let Some(down) = note.octave_down() {
                    let low = target_hz(base, down);
                    if (1200.0 * (raw_hz / low).log2()).abs() < 80.0 {
                        self.crack_low += 1;
                    }
                }
            }
        }

        let mut near_target = match (est.hz, expected) {
            (Some(hz), Some(exp)) => 1200.0 * (hz / exp).log2().abs() <= tol,
            _ => false,
        };
        if self.cfg.recalibrate
            && matches!(self.cfg.mode, AttemptMode::FirstSound)
            && sounding
            && !early_break
        {
            if let Some(hz) = est.hz {
                if (250.0..=1600.0).contains(&hz) {
                    near_target = true;
                }
            }
        }

        // Onset counting for on_the_breath
        if sounding && self.was_quiet {
            self.onset_count += 1;
        }
        self.was_quiet = !sounding;

        self.accumulate_hold(near_target, sounding, est.hz, energy);
        self.contour.push(ContourPoint {
            t_ms: self.t_ms,
            hz: est.hz,
            rms: energy,
        });
        self.track_phrase(est.hz, sounding, tol);
        self.note_stop(sounding);

        let frame = Frame {
            t_ms: self.t_ms,
            hz: est.hz,
            rms: energy,
            confidence: est.confidence,
            near_target,
            early_break,
            expected_note: self.current_expected_note().map(|n| n.as_str().to_string()),
            phrase_index: self.phrase_idx,
            hold_ratio: self.hold_ratio(),
            leak_hole: self.leak_for(est.hz, tol),
            stopped: self.stopped,
        };
        self.frames.push(frame.clone());
        frame
    }

    fn current_expected_note(&self) -> Option<NoteName> {
        match &self.cfg.mode {
            AttemptMode::FirstSound => Some(NoteName::D4),
            AttemptMode::BreathOctave { want_octave } => Some(if *want_octave {
                NoteName::D5
            } else {
                NoteName::D4
            }),
            AttemptMode::SingleNote { note } | AttemptMode::Ornament { note, .. } => Some(*note),
            AttemptMode::Phrase { notes, .. } | AttemptMode::OnTheBreath { notes } => {
                notes.get(self.phrase_idx).copied()
            }
        }
    }

    fn current_expected_hz(&self) -> Option<f32> {
        let base = self.cfg.break_hz.unwrap_or(DEFAULT_LOW_D_HZ);
        self.current_expected_note().map(|n| target_hz(base, n))
    }

    fn hold_goal(&self) -> f32 {
        match self.cfg.mode {
            AttemptMode::FirstSound => SETTLE_SECONDS,
            AttemptMode::BreathOctave { want_octave: false } => 2.0,
            AttemptMode::BreathOctave { want_octave: true } => 1.5,
            AttemptMode::SingleNote { .. } => 0.8,
            _ => 0.0,
        }
    }

    fn hold_ratio(&self) -> f32 {
        let goal = self.hold_goal();
        if goal <= 0.0 {
            return 0.0;
        }
        (self.hold_secs / goal).clamp(0.0, 1.0)
    }

    fn accumulate_hold(&mut self, near: bool, sounding: bool, hz: Option<f32>, energy: f32) {
        if self.hold_goal() <= 0.0 {
            return;
        }
        let hop = self.cfg.hop_ms / 1000.0;
        if near && sounding {
            self.off_secs = 0.0;
            self.hold_secs += hop;
            if let Some(hz) = hz {
                self.hold_hz_acc += hz;
                self.hold_rms_acc += energy;
                self.hold_n += 1;
            }
        } else if matches!(self.cfg.mode, AttemptMode::FirstSound) {
            self.off_secs += hop;
            if self.off_secs > 0.30 {
                self.hold_secs = (self.hold_secs - hop).max(0.0);
            }
        }
    }

    /// Confirm phrase notes in order. A later note does not count until the one before it has dwelled.
    fn track_phrase(&mut self, hz: Option<f32>, sounding: bool, tol: f32) {
        let notes = match &self.cfg.mode {
            AttemptMode::Phrase { notes, .. } | AttemptMode::OnTheBreath { notes } => notes.clone(),
            _ => return,
        };
        if notes.is_empty() {
            return;
        }
        let base = self.cfg.break_hz.unwrap_or(DEFAULT_LOW_D_HZ);
        let hop = self.cfg.hop_ms / 1000.0;
        let on = |note: NoteName| {
            sounding
                && hz
                    .map(|h| (1200.0 * (h / target_hz(base, note)).log2()).abs() <= tol)
                    .unwrap_or(false)
        };

        if self.notes_confirmed >= notes.len() {
            self.phrase_idx = notes.len() - 1;
            return;
        }

        let next_i = self.notes_confirmed;
        if on(notes[next_i]) {
            if self.slice_starts.len() == next_i {
                self.slice_starts.push(self.contour.len().saturating_sub(1));
            }
            self.phrase_dwell += hop;
            self.phrase_idx = next_i;
            if self.phrase_dwell >= PHRASE_DWELL_SECS {
                self.notes_confirmed = next_i + 1;
                self.phrase_dwell = 0.0;
                self.gap_secs = 0.0;
                self.phrase_idx = if self.notes_confirmed < notes.len() {
                    next_i
                } else {
                    notes.len() - 1
                };
            }
            return;
        }

        if !sounding && self.notes_confirmed > 0 && self.notes_confirmed < notes.len() {
            self.gap_secs += hop;
            if self.gap_secs >= 0.28 {
                let idx = self.notes_confirmed - 1;
                if !self.breaths_heard.contains(&idx) {
                    self.breaths_heard.push(idx);
                }
            }
        } else if sounding {
            self.gap_secs = 0.0;
        }

        if next_i > 0 && on(notes[next_i - 1]) {
            self.phrase_idx = next_i - 1;
            self.phrase_dwell = 0.0;
            return;
        }

        self.phrase_dwell = 0.0;
        self.phrase_idx = next_i.min(notes.len() - 1);
    }

    /// Latch a stop once the attempt has been heard and then goes quiet.
    /// A breath, and a gap inside an ornament, stay under the tail.
    fn note_stop(&mut self, sounding: bool) {
        if self.stopped {
            return;
        }
        let heard = self.frames.iter().filter(|frame| frame.hz.is_some()).count();
        if heard < STOP_HEARD_FRAMES {
            self.quiet_secs = 0.0;
            return;
        }
        if sounding {
            self.quiet_secs = 0.0;
            return;
        }
        self.quiet_secs += self.cfg.hop_ms / 1000.0;
        if self.quiet_secs + 0.0001 >= self.stop_tail() {
            self.stopped = true;
        }
    }

    fn goal_met(&self) -> bool {
        match &self.cfg.mode {
            AttemptMode::Phrase { notes, .. } | AttemptMode::OnTheBreath { notes } => {
                !notes.is_empty() && self.notes_confirmed >= notes.len()
            }
            AttemptMode::Ornament { .. } => false,
            _ => {
                let goal = self.hold_goal();
                goal > 0.0 && self.hold_secs >= goal
            }
        }
    }

    fn stop_tail(&self) -> f32 {
        if matches!(self.cfg.mode, AttemptMode::Ornament { .. }) {
            return STOP_ORNAMENT_SECS;
        }
        if self.goal_met() || self.early_break_latched {
            STOP_AFTER_GOAL_SECS
        } else {
            STOP_ABANDON_SECS
        }
    }

    fn leak_for(&self, hz: Option<f32>, tol: f32) -> Option<u8> {
        let expected = self.current_expected_note()?;
        let hz = hz?;
        let base = self.cfg.break_hz.unwrap_or(DEFAULT_LOW_D_HZ);
        let heard = nearest_scale_note(hz, base, tol)?;
        if heard == expected {
            return None;
        }
        fingering::differing_hole(expected, heard)
    }

    pub fn finish(&self) -> AttemptResult {
        let evidence = self.decide();
        let settled = matches!(
            evidence,
            Evidence::LowDHeld
                | Evidence::OctaveThere
                | Evidence::PhraseOk
                | Evidence::NoteFound
                | Evidence::LetThemJoin
        ) || (matches!(evidence, Evidence::Abstain)
            && matches!(self.cfg.mode, AttemptMode::Ornament { .. }));

        let (break_hz, rms_floor) = if matches!(self.cfg.mode, AttemptMode::FirstSound)
            && matches!(evidence, Evidence::LowDHeld)
            && self.hold_n > 0
        {
            let held_rms = self.hold_rms_acc / self.hold_n as f32;
            // Floor is a quiet threshold under the held note, not the note itself.
            (
                Some(self.hold_hz_acc / self.hold_n as f32),
                Some((held_rms * 0.25).max(0.008)),
            )
        } else {
            (None, None)
        };

        let remark_note = match &self.cfg.mode {
            AttemptMode::SingleNote { note } => Some(note.as_str().to_string()),
            _ => None,
        };

        let settled = settled && !matches!(evidence, Evidence::EarlyBreak | Evidence::CouldntHear);
        AttemptResult {
            evidence,
            settled,
            break_hz,
            rms_floor,
            frames: self.frames.clone(),
            target_hz: self.frozen_target,
            remark_note,
            warm: self.warm_hold(),
            cnat_disagree: self.cnat_disagree(),
            ghost: self.ghost_trace(evidence),
            isolate_note: self.isolate_note(evidence),
            take_wav: self.take_wav(evidence),
        }
    }

    /// A phrase fault that names one pitch. Other remarks stay on the phrase.
    fn isolate_note(&self, evidence: Evidence) -> Option<String> {
        let notes = match &self.cfg.mode {
            AttemptMode::Phrase { notes, .. } => notes,
            _ => return None,
        };
        if notes.is_empty() || !matches!(evidence, Evidence::Sealed | Evidence::StillD) {
            return None;
        }
        if let Some(note) = self.dominant_leak_note() {
            return Some(note);
        }
        let i = self.notes_confirmed.min(notes.len() - 1);
        notes.get(i).map(|note| note.as_str().to_string())
    }

    fn dominant_leak_note(&self) -> Option<String> {
        let mut counts: Vec<(String, usize)> = Vec::new();
        for frame in &self.frames {
            if frame.leak_hole.is_none() {
                continue;
            }
            let Some(note) = frame.expected_note.clone() else {
                continue;
            };
            if let Some((_, n)) = counts.iter_mut().find(|(name, _)| name == &note) {
                *n += 1;
            } else {
                counts.push((note, 1));
            }
        }
        counts
            .into_iter()
            .max_by_key(|(_, n)| *n)
            .map(|(note, _)| note)
    }

    fn take_wav(&self, evidence: Evidence) -> Option<TakeWav> {
        if matches!(evidence, Evidence::CouldntHear | Evidence::Abstain) || self.take.len() < 256 {
            return None;
        }
        Some(TakeWav(super::synth::wav_bytes(
            self.cfg.sample_rate,
            &self.take,
        )))
    }

    /// Mean pitch of a low hold, against the stored break. An early octave is not warmth.
    fn warm_hold(&self) -> bool {
        let Some(break_hz) = self.cfg.break_hz.filter(|hz| *hz > 0.0) else {
            return false;
        };
        let low_hold = matches!(
            self.cfg.mode,
            AttemptMode::FirstSound | AttemptMode::BreathOctave { want_octave: false }
        );
        if !low_hold || self.early_break_latched {
            return false;
        }
        let voiced: Vec<f32> = self.frames.iter().filter_map(|f| f.hz).collect();
        if voiced.len() < 8 {
            return false;
        }
        let mean = voiced.iter().sum::<f32>() / voiced.len() as f32;
        crate::listen::types::cents_between(mean, break_hz).abs() > CENTS_TOLERANCE_POST_CAL
    }

    fn cnat_disagree(&self) -> bool {
        let AttemptMode::SingleNote { note } = &self.cfg.mode else {
            return false;
        };
        if *note != NoteName::C5 {
            return false;
        }
        let voiced: Vec<&Frame> = self.frames.iter().filter(|f| f.hz.is_some()).collect();
        if voiced.len() < 8 {
            return false;
        }
        if voiced.iter().any(|f| f.leak_hole.is_some()) {
            return false;
        }
        let near = voiced.iter().filter(|f| f.near_target).count();
        near * 2 < voiced.len()
    }

    fn ghost_trace(&self, evidence: Evidence) -> Option<GhostTrace> {
        if matches!(evidence, Evidence::CouldntHear | Evidence::Abstain) {
            return None;
        }
        let target = self.frozen_target.filter(|hz| *hz > 0.0)?;
        if self.frames.len() < 2 {
            return None;
        }
        const CAP: usize = 64;
        let step = (self.frames.len() / CAP).max(1);
        let mut points = Vec::new();
        for (i, frame) in self.frames.iter().enumerate() {
            let keep = i % step == 0 || i + 1 == self.frames.len();
            if !keep || points.len() >= CAP {
                continue;
            }
            let Some(hz) = frame.hz else {
                continue;
            };
            let cents = crate::listen::types::cents_between(hz, target);
            if !cents.is_finite() {
                continue;
            }
            points.push(GhostPoint {
                t_ms: frame.t_ms,
                cents: cents.clamp(-1200.0, 1200.0),
            });
        }
        if points.len() < 2 {
            return None;
        }
        let base = self.cfg.break_hz.unwrap_or(DEFAULT_LOW_D_HZ);
        let mut spans = Vec::new();
        let mut open: Option<(String, f32, usize)> = None;
        for frame in &self.frames {
            let Some(note) = frame.expected_note.clone() else {
                continue;
            };
            let idx = frame.phrase_index;
            match &open {
                Some((name, _, open_idx)) if name == &note && *open_idx == idx => {}
                Some((name, t0, open_idx)) => {
                    spans.push(self.span(name, *t0, frame.t_ms, base, target, *open_idx));
                    open = Some((note, frame.t_ms, idx));
                }
                None => open = Some((note, frame.t_ms, idx)),
            }
        }
        if let Some((name, t0, idx)) = open {
            let end = self.frames.last().map(|f| f.t_ms).unwrap_or(t0);
            spans.push(self.span(&name, t0, end, base, target, idx));
        }
        Some(GhostTrace { points, spans })
    }

    fn span(
        &self,
        note: &str,
        t0_ms: f32,
        t1_ms: f32,
        base: f32,
        target: f32,
        phrase_index: usize,
    ) -> GhostSpan {
        let cents = NoteName::from_str(note)
            .map(|named| crate::listen::types::cents_between(target_hz(base, named), target))
            .filter(|c| c.is_finite())
            .unwrap_or(0.0);
        GhostSpan {
            t0_ms,
            t1_ms,
            note: note.to_string(),
            cents,
            mark: self.mark_on(phrase_index),
        }
    }

    fn mark_on(&self, phrase_index: usize) -> String {
        match &self.cfg.mode {
            AttemptMode::Ornament { gesture, .. } => gesture.as_str().to_string(),
            AttemptMode::Phrase { marks, .. } => marks
                .iter()
                .find(|m| m.note_index == phrase_index)
                .map(|m| m.gesture.as_str().to_string())
                .unwrap_or_default(),
            _ => String::new(),
        }
    }

    fn decide(&self) -> Evidence {
        let voiced: usize = self.frames.iter().filter(|f| f.hz.is_some()).count();
        if voiced < 3 {
            return Evidence::CouldntHear;
        }

        // Unstable: many low-confidence frames while energy present
        let messy = self
            .frames
            .iter()
            .filter(|f| f.rms > 0.02 && f.confidence < 0.2)
            .count();
        if messy > self.frames.len() / 2 {
            return Evidence::CouldntHear;
        }

        match &self.cfg.mode {
            AttemptMode::FirstSound => {
                if self.early_break_latched && self.hold_secs < SETTLE_SECONDS {
                    return Evidence::EarlyBreak;
                }
                if self.hold_secs >= SETTLE_SECONDS {
                    return Evidence::LowDHeld;
                }
                // Partial hold — likely leak
                if self.hold_secs > 1.0 {
                    return Evidence::Sealed;
                }
                Evidence::Sealed
            }
            AttemptMode::BreathOctave { want_octave: false } => {
                if self.early_break_latched {
                    return Evidence::EarlyBreak;
                }
                if self.hold_secs >= 2.0 {
                    return Evidence::LowDHeld;
                }
                Evidence::Sealed
            }
            AttemptMode::BreathOctave { want_octave: true } => {
                if self.hold_secs >= 1.5 {
                    return Evidence::OctaveThere;
                }
                if self.early_break_latched {
                    // started low then broke — for want_octave that's progress-ish
                    return Evidence::BrokeEarly;
                }
                Evidence::CouldntHear
            }
            AttemptMode::SingleNote { note } => {
                if note.semitones_from_d4() >= 12 && self.crack_low > (self.frames.len() / 2) as u32
                {
                    return Evidence::Cracked;
                }
                if self.hold_secs >= 0.8 {
                    // Still on D when looking for higher?
                    if *note != NoteName::D4 {
                        let base = self.cfg.break_hz.unwrap_or(DEFAULT_LOW_D_HZ);
                        let d_count = self
                            .frames
                            .iter()
                            .filter(|f| {
                                f.hz.map(|h| 1200.0 * (h / base).log2().abs() < 50.0)
                                    .unwrap_or(false)
                            })
                            .count();
                        if d_count > self.frames.len() / 2 {
                            return Evidence::StillD;
                        }
                    }
                    return Evidence::NoteFound;
                }
                Evidence::Sealed
            }
            AttemptMode::Phrase {
                notes,
                breaths,
                marks,
            } => {
                let bad_breath =
                    !breaths.is_empty() && self.breaths_heard.iter().any(|i| !breaths.contains(i));
                if bad_breath && self.notes_confirmed > 0 {
                    return Evidence::BreathChops;
                }
                if !notes.is_empty() && self.notes_confirmed >= notes.len() {
                    if let Some(ev) = self.mark_evidence(notes, marks) {
                        if ev != Evidence::Abstain {
                            return ev;
                        }
                    }
                    return Evidence::PhraseOk;
                }
                let near = self.frames.iter().filter(|f| f.near_target).count();
                if near < 3 {
                    return Evidence::CouldntHear;
                }
                if self.stuck_on_low_d(notes) {
                    return Evidence::StillD;
                }
                Evidence::Sealed
            }
            AttemptMode::OnTheBreath { notes } => {
                let near = self.frames.iter().filter(|f| f.near_target).count();
                if near < 4 {
                    return Evidence::CouldntHear;
                }
                // One onset per note (or more) → tonguing every attack
                if self.onset_count as usize >= notes.len().max(3) {
                    return Evidence::RestartedNotes;
                }
                if near > 6 {
                    return Evidence::PhraseOk;
                }
                Evidence::LetThemJoin
            }
            AttemptMode::Ornament { note: _, gesture } => {
                let body = self.frozen_target.unwrap_or(DEFAULT_LOW_D_HZ);
                if ornaments::cut_became_note(&self.contour, body) {
                    return Evidence::CutTooLong;
                }
                ornaments::parse_gesture(&self.contour, body, *gesture)
            }
        }
    }

    /// A clear miss on a marked gesture blocks the phrase. Silence does not.
    fn mark_evidence(&self, notes: &[NoteName], marks: &[PhraseMark]) -> Option<Evidence> {
        if marks.is_empty() {
            return None;
        }
        let base = self.cfg.break_hz.unwrap_or(DEFAULT_LOW_D_HZ);
        let mut saw = false;
        for mark in marks {
            let start = *self.slice_starts.get(mark.note_index)?;
            let end = self
                .slice_starts
                .get(mark.note_index + 1)
                .copied()
                .unwrap_or(self.contour.len());
            if end <= start {
                continue;
            }
            let note = notes.get(mark.note_index).copied()?;
            let body = target_hz(base, note);
            let slice = &self.contour[start..end];
            if matches!(
                mark.gesture,
                GestureKind::Cut
                    | GestureKind::Roll
                    | GestureKind::ShortRoll
                    | GestureKind::Cran
                    | GestureKind::DoubleTap
                    | GestureKind::Triplet
            ) && ornaments::cut_became_note(slice, body)
            {
                return Some(Evidence::CutTooLong);
            }
            let ev = ornaments::parse_gesture(slice, body, mark.gesture);
            saw = true;
            if matches!(
                ev,
                Evidence::CutTooLong
                    | Evidence::TapMissing
                    | Evidence::BecameNotes
                    | Evidence::SlideMissed
            ) {
                return Some(ev);
            }
        }
        saw.then_some(Evidence::Abstain)
    }

    /// The pitch stays on low D while the phrase is waiting for a higher note.
    /// The frame label can still name the note already played.
    fn stuck_on_low_d(&self, notes: &[NoteName]) -> bool {
        if notes.is_empty() || self.notes_confirmed >= notes.len() {
            return false;
        }
        let asked = notes[self.notes_confirmed];
        if asked == NoteName::D4 {
            return false;
        }
        let base = self.cfg.break_hz.unwrap_or(DEFAULT_LOW_D_HZ);
        let voiced: Vec<_> = self
            .frames
            .iter()
            .filter(|frame| frame.hz.is_some())
            .collect();
        if voiced.len() < 3 {
            return false;
        }
        let on_d = voiced
            .iter()
            .filter(|frame| {
                frame
                    .hz
                    .map(|hz| (1200.0 * (hz / base).log2()).abs() < 50.0)
                    .unwrap_or(false)
            })
            .count();
        on_d * 2 > voiced.len()
    }
}

/// Helper: build engine for first_sound with optional existing profile.
pub fn first_sound_engine(sample_rate: u32, profile: Option<&WhistleProfile>) -> AttemptEngine {
    AttemptEngine::new(AttemptConfig {
        sample_rate,
        hop_ms: 30.0,
        mode: AttemptMode::FirstSound,
        break_hz: profile.and_then(|p| (p.break_hz > 0.0).then_some(p.break_hz)),
        rms_floor: profile.and_then(|p| (p.rms_floor > 0.0).then_some(p.rms_floor)),
        recalibrate: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::listen::synth;

    fn run(cfg: AttemptConfig, samples: &[f32]) -> AttemptResult {
        let mut eng = AttemptEngine::new(cfg);
        for chunk in samples.chunks(1024) {
            eng.push_samples(chunk);
        }
        eng.finish()
    }

    #[test]
    fn early_break_does_not_move_the_target() {
        let hz = DEFAULT_LOW_D_HZ;
        let result = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::BreathOctave { want_octave: false },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::fixture_early_break(44100, hz),
        );
        assert_eq!(result.evidence, Evidence::EarlyBreak);
        assert_eq!(result.target_hz, Some(hz));
        assert!(!result.settled);
    }

    #[test]
    fn first_sound_settles_only_after_the_hold() {
        let hz = DEFAULT_LOW_D_HZ;
        let short = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::FirstSound,
                break_hz: None,
                rms_floor: None,
                recalibrate: false,
            },
            &synth::sine(hz, 44100, 2.0, 0.35),
        );
        assert_ne!(short.evidence, Evidence::LowDHeld);
        let held = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::FirstSound,
                break_hz: None,
                rms_floor: None,
                recalibrate: false,
            },
            &synth::fixture_steady_low_d(44100, hz),
        );
        assert_eq!(held.evidence, Evidence::LowDHeld);
        assert!(held.settled);
        let ratio = held.frames.last().map(|f| f.hold_ratio).unwrap_or(0.0);
        assert!(ratio >= 1.0, "hold ratio {ratio}");
    }

    #[test]
    fn phrase_in_order_settles_and_a_scramble_does_not() {
        let hz = DEFAULT_LOW_D_HZ;
        let notes = vec![NoteName::D4, NoteName::E4, NoteName::Fs4, NoteName::G4];
        let ordered = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::Phrase {
                    notes: notes.clone(),
                    breaths: vec![],
                    marks: vec![],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::fixture_joined_phrase(44100, hz),
        );
        assert_eq!(ordered.evidence, Evidence::PhraseOk, "{ordered:?}");
        assert!(ordered.settled);
        assert!((ordered.target_hz.unwrap() - hz).abs() < 1.0);
        assert!(ordered.take_wav.is_some());
        assert!(ordered.isolate_note.is_none());

        let mut scramble = Vec::new();
        for sem in [5, 2, 0, 4] {
            let tone = hz * 2f32.powf(sem as f32 / 12.0);
            scramble.extend(synth::sine(tone, 44100, 0.4, 0.35));
        }
        let messy = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::Phrase {
                    notes,
                    breaths: vec![],
                    marks: vec![],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &scramble,
        );
        assert_ne!(messy.evidence, Evidence::PhraseOk);
        assert!(!messy.settled);
    }

    #[test]
    fn on_the_breath_follows_the_line_in_order() {
        let hz = DEFAULT_LOW_D_HZ;
        let result = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::OnTheBreath {
                    notes: vec![NoteName::D4, NoteName::E4, NoteName::Fs4, NoteName::G4],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::fixture_joined_phrase(44100, hz),
        );
        assert_eq!(result.evidence, Evidence::PhraseOk);
        let index = result.frames.last().map(|f| f.phrase_index).unwrap_or(0);
        assert!(index >= 2, "phrase index stayed at {index}");
    }

    #[test]
    fn one_hole_leak_names_that_hole() {
        let hz = DEFAULT_LOW_D_HZ;
        let result = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::SingleNote { note: NoteName::E4 },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::sine(hz, 44100, 1.0, 0.35),
        );
        assert!(
            result.frames.iter().any(|f| f.leak_hole == Some(5)),
            "expected the bottom hole"
        );
        assert!(result
            .frames
            .iter()
            .all(|f| f.leak_hole != Some(0) || f.hz.is_none()));
    }

    #[test]
    fn two_hole_miss_names_no_hole() {
        let hz = DEFAULT_LOW_D_HZ;
        let g = hz * 2f32.powf(5.0 / 12.0);
        let result = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::SingleNote { note: NoteName::D4 },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::sine(g, 44100, 0.8, 0.35),
        );
        assert!(result.frames.iter().all(|f| f.leak_hole.is_none()));
    }

    #[test]
    fn a_short_cut_abstains_and_settles() {
        let hz = DEFAULT_LOW_D_HZ;
        let body = target_hz(hz, NoteName::A4);
        let result = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 10.0,
                mode: AttemptMode::Ornament {
                    note: NoteName::A4,
                    gesture: GestureKind::Cut,
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::fixture_cut(44100, body),
        );
        let mut off = 0.0f32;
        let mut run = 0.0f32;
        let mut prev: Option<f32> = None;
        for f in &result.frames {
            let high =
                f.hz.map(|h| 1200.0 * (h / body).log2() > 80.0)
                    .unwrap_or(false);
            if high {
                if let Some(p) = prev {
                    run += f.t_ms - p;
                }
                off = off.max(run);
            } else {
                run = 0.0;
            }
            prev = Some(f.t_ms);
        }
        assert_eq!(
            result.evidence,
            Evidence::Abstain,
            "off_ms={off:.0} settled={} frames={}",
            result.settled,
            result.frames.len()
        );
        assert!(result.settled);
        assert!(result.take_wav.is_none());
    }

    #[test]
    fn ornament_without_a_gesture_abstains() {
        let hz = DEFAULT_LOW_D_HZ;
        let body = target_hz(hz, NoteName::A4);
        let result = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 10.0,
                mode: AttemptMode::Ornament {
                    note: NoteName::A4,
                    gesture: GestureKind::Cut,
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::sine(body, 44100, 1.0, 0.35),
        );
        assert_eq!(result.evidence, Evidence::Abstain);
    }

    #[test]
    fn a_breath_in_the_gap_settles_and_a_chop_does_not() {
        let hz = DEFAULT_LOW_D_HZ;
        let notes = vec![NoteName::D4, NoteName::E4];
        let held = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::Phrase {
                    notes: notes.clone(),
                    breaths: vec![0],
                    marks: vec![],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::fixture_two_notes_gap(44100, hz, 0.4),
        );
        assert_eq!(held.evidence, Evidence::PhraseOk, "{held:?}");
        assert!(held.settled);

        let chop = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::Phrase {
                    notes,
                    breaths: vec![1],
                    marks: vec![],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::fixture_two_notes_gap(44100, hz, 0.4),
        );
        assert_eq!(chop.evidence, Evidence::BreathChops, "{chop:?}");
        assert!(!chop.settled);
    }

    #[test]
    fn leaked_c_and_a_cracked_octave_e() {
        let hz = DEFAULT_LOW_D_HZ;
        let leaked = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::SingleNote { note: NoteName::C5 },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::fixture_leaked_c(44100, hz),
        );
        assert_eq!(leaked.evidence, Evidence::Sealed);
        assert_eq!(leaked.remark_note.as_deref(), Some("C5"));
        assert!(!leaked.settled);

        let crack = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::SingleNote { note: NoteName::E5 },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::fixture_octave_e_cracks(44100, hz),
        );
        assert_eq!(crack.evidence, Evidence::Cracked, "{crack:?}");
        assert!(!crack.settled);
        let e5 = target_hz(hz, NoteName::E5);
        assert!((crack.target_hz.unwrap() - e5).abs() < 1.0);
    }

    #[test]
    fn a_dorian_phrase_settles_from_the_break() {
        let hz = DEFAULT_LOW_D_HZ;
        let result = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::Phrase {
                    notes: vec![NoteName::A4, NoteName::G4, NoteName::C5, NoteName::A4],
                    breaths: vec![],
                    marks: vec![],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::fixture_a_dorian(44100, hz),
        );
        assert_eq!(result.evidence, Evidence::PhraseOk, "{result:?}");
        assert!(result.settled);
    }

    #[test]
    fn a_cut_that_became_a_note_blocks_the_phrase() {
        let hz = DEFAULT_LOW_D_HZ;
        let body = target_hz(hz, NoteName::A4);
        let result = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 10.0,
                mode: AttemptMode::Phrase {
                    notes: vec![NoteName::A4],
                    breaths: vec![],
                    marks: vec![PhraseMark {
                        note_index: 0,
                        gesture: GestureKind::Cut,
                    }],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::fixture_cut_too_long(44100, body),
        );
        assert_eq!(result.evidence, Evidence::CutTooLong, "{result:?}");
        assert!(!result.settled);
        let ghost = result.ghost.expect("cut ghost");
        assert!(
            ghost
                .spans
                .iter()
                .any(|s| s.note == "A4" && s.mark == "cut"),
            "{:?}",
            ghost.spans
        );
    }

    #[test]
    fn a_short_roll_that_splits_does_not_settle() {
        let hz = DEFAULT_LOW_D_HZ;
        let body = target_hz(hz, NoteName::A4);
        let result = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 10.0,
                mode: AttemptMode::Ornament {
                    note: NoteName::A4,
                    gesture: GestureKind::ShortRoll,
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::fixture_roll_split(44100, body),
        );
        assert_eq!(result.evidence, Evidence::BecameNotes, "{result:?}");
        assert!(!result.settled);
        let ghost = result.ghost.expect("roll ghost");
        assert!(ghost.spans.iter().any(|s| s.mark == "short_roll"));
    }

    #[test]
    fn a_phrase_keeps_a_capped_ghost_and_silence_does_not() {
        let hz = DEFAULT_LOW_D_HZ;
        let phrase = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::OnTheBreath {
                    notes: vec![NoteName::D4, NoteName::E4, NoteName::Fs4, NoteName::G4],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::fixture_joined_phrase(44100, hz),
        );
        let ghost = phrase.ghost.expect("phrase ghost");
        assert!(ghost.points.len() >= 2 && ghost.points.len() <= 64);
        assert!(!ghost.spans.is_empty());
        assert!(ghost.spans.iter().all(|s| s.mark.is_empty()));
        assert!(ghost.spans.iter().any(|s| s.note == "D4"));

        let noise = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::FirstSound,
                break_hz: None,
                rms_floor: None,
                recalibrate: false,
            },
            &synth::fixture_noise(44100),
        );
        assert!(noise.ghost.is_none());
        assert!(!noise.warm);
    }

    #[test]
    fn a_sharp_hold_is_warmth_and_a_one_hole_c_is_not_a_new_fingering() {
        let hz = DEFAULT_LOW_D_HZ;
        let sharp = hz * 2f32.powf(100.0 / 1200.0);
        let warm = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::FirstSound,
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::sine(sharp, 44100, 2.0, 0.35),
        );
        assert!(warm.warm, "{warm:?}");
        assert!(warm.ghost.is_some());

        let leaked = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::SingleNote { note: NoteName::C5 },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::fixture_leaked_c(44100, hz),
        );
        assert!(!leaked.cnat_disagree, "{leaked:?}");

        let other = target_hz(hz, NoteName::B4);
        let disagree = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::SingleNote { note: NoteName::C5 },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::sine(other, 44100, 1.2, 0.35),
        );
        assert!(disagree.cnat_disagree, "{disagree:?}");
        assert!(!disagree.settled);
    }

    #[test]
    fn recalibrate_accepts_a_warm_hold_and_leaves_the_target_frozen() {
        let hz = DEFAULT_LOW_D_HZ;
        let sharp = hz * 2f32.powf(80.0 / 1200.0);
        let result = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::FirstSound,
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: true,
            },
            &synth::sine(sharp, 44100, 10.5, 0.35),
        );
        assert_eq!(result.evidence, Evidence::LowDHeld, "{result:?}");
        assert!(result.settled);
        let written = result.break_hz.expect("new break");
        assert!((written - sharp).abs() < 5.0, "{written} vs {sharp}");
        assert!((result.target_hz.unwrap() - hz).abs() < 1.0);
    }

    #[test]
    fn couldnt_hear_keeps_no_take_and_drop_forgets_a_heard_one() {
        let noise = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::FirstSound,
                break_hz: None,
                rms_floor: None,
                recalibrate: false,
            },
            &synth::fixture_noise(44100),
        );
        assert_eq!(noise.evidence, Evidence::CouldntHear);
        assert!(noise.take_wav.is_none());

        let hz = DEFAULT_LOW_D_HZ;
        let held = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::FirstSound,
                break_hz: None,
                rms_floor: None,
                recalibrate: false,
            },
            &synth::fixture_steady_low_d(44100, hz),
        );
        let bytes = held.take_wav.expect("heard take").0;
        assert!(bytes.starts_with(b"RIFF"));
        assert!(bytes.len() > 44);
        let mut slot = TakeSlot::default();
        slot.store(Some(TakeWav(bytes)));
        assert!(slot.bytes().is_some());
        slot.drop_take();
        assert!(slot.bytes().is_none());
    }

    #[test]
    fn the_take_stops_at_thirty_seconds() {
        let mut take = vec![0.0; 10];
        append_take(&mut take, &[1.0; 5], 12);
        assert_eq!(take.len(), 12);
        append_take(&mut take, &[1.0; 5], 12);
        assert_eq!(take.len(), 12);
    }

    #[test]
    fn a_phrase_stuck_on_low_d_names_the_note_that_was_waiting() {
        let hz = DEFAULT_LOW_D_HZ;
        let result = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::Phrase {
                    notes: vec![NoteName::D4, NoteName::E4],
                    breaths: vec![],
                    marks: vec![],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &synth::sine(hz, 44100, 1.2, 0.35),
        );
        assert_eq!(result.evidence, Evidence::StillD, "{result:?}");
        assert_eq!(result.isolate_note.as_deref(), Some("E4"));
        assert!(!result.settled);
    }

    #[test]
    fn a_one_hole_miss_in_a_phrase_names_that_note() {
        let hz = DEFAULT_LOW_D_HZ;
        let mut samples = synth::sine(hz, 44100, 0.4, 0.35);
        let fs = hz * 2f32.powf(4.0 / 12.0);
        samples.extend(synth::sine(fs, 44100, 0.8, 0.35));
        let result = run(
            AttemptConfig {
                sample_rate: 44100,
                hop_ms: 30.0,
                mode: AttemptMode::Phrase {
                    notes: vec![NoteName::D4, NoteName::E4],
                    breaths: vec![],
                    marks: vec![],
                },
                break_hz: Some(hz),
                rms_floor: Some(0.05),
                recalibrate: false,
            },
            &samples,
        );
        assert_eq!(result.evidence, Evidence::Sealed, "{result:?}");
        assert_eq!(result.isolate_note.as_deref(), Some("E4"));
        assert!(result.take_wav.is_some());
        assert!((result.target_hz.unwrap() - hz).abs() < 1.0);
        assert!(!result.settled);
    }

    fn first_cfg() -> AttemptConfig {
        AttemptConfig {
            sample_rate: 44100,
            hop_ms: 30.0,
            mode: AttemptMode::FirstSound,
            break_hz: None,
            rms_floor: None,
            recalibrate: false,
        }
    }

    fn phrase_cfg(notes: Vec<NoteName>) -> AttemptConfig {
        AttemptConfig {
            sample_rate: 44100,
            hop_ms: 30.0,
            mode: AttemptMode::Phrase {
                notes,
                breaths: vec![],
                marks: vec![],
            },
            break_hz: Some(DEFAULT_LOW_D_HZ),
            rms_floor: Some(0.05),
            recalibrate: false,
        }
    }

    #[test]
    fn silence_before_a_note_does_not_stop() {
        let result = run(first_cfg(), &synth::silence(44100, 2.0));
        assert!(result.frames.iter().all(|frame| !frame.stopped));
    }

    #[test]
    fn a_finished_phrase_then_silence_stops() {
        let hz = DEFAULT_LOW_D_HZ;
        let mut samples = synth::fixture_joined_phrase(44100, hz);
        samples.extend(synth::silence(44100, 0.7));
        let result = run(
            phrase_cfg(vec![
                NoteName::D4,
                NoteName::E4,
                NoteName::Fs4,
                NoteName::G4,
            ]),
            &samples,
        );
        assert_eq!(result.evidence, Evidence::PhraseOk);
        assert!(result.frames.iter().any(|frame| !frame.stopped));
        assert!(result.frames.last().is_some_and(|frame| frame.stopped));
    }

    #[test]
    fn a_breath_inside_a_phrase_does_not_stop() {
        let hz = DEFAULT_LOW_D_HZ;
        let result = run(
            phrase_cfg(vec![NoteName::D4, NoteName::E4]),
            &synth::fixture_two_notes_gap(44100, hz, 0.4),
        );
        assert!(result.frames.iter().all(|frame| !frame.stopped));
    }

    #[test]
    fn a_full_hold_stops_only_after_the_note_ends() {
        let hz = DEFAULT_LOW_D_HZ;
        let sounding = run(first_cfg(), &synth::fixture_steady_low_d(44100, hz));
        assert!(sounding.frames.last().is_some_and(|frame| frame.hold_ratio >= 1.0));
        assert!(sounding.frames.last().is_some_and(|frame| !frame.stopped));

        let mut stopped = synth::fixture_steady_low_d(44100, hz);
        stopped.extend(synth::silence(44100, 0.7));
        let done = run(first_cfg(), &stopped);
        assert!(done.frames.last().is_some_and(|frame| frame.stopped));
    }

    #[test]
    fn a_short_breath_in_the_long_hold_does_not_stop() {
        let hz = DEFAULT_LOW_D_HZ;
        let mut samples = synth::sine(hz, 44100, 3.0, 0.35);
        samples.extend(synth::silence(44100, 0.4));
        samples.extend(synth::sine(hz, 44100, 1.0, 0.35));
        let result = run(first_cfg(), &samples);
        assert!(result.frames.iter().all(|frame| !frame.stopped));
    }

    #[test]
    fn a_long_quiet_after_a_partial_hold_stops() {
        let hz = DEFAULT_LOW_D_HZ;
        let mut samples = synth::sine(hz, 44100, 2.0, 0.35);
        samples.extend(synth::silence(44100, 2.0));
        let result = run(first_cfg(), &samples);
        assert!(result.frames.iter().any(|frame| !frame.stopped));
        assert!(result.frames.last().is_some_and(|frame| frame.stopped));
    }

    fn low_hold_cfg() -> AttemptConfig {
        AttemptConfig {
            sample_rate: 44100,
            hop_ms: 30.0,
            mode: AttemptMode::BreathOctave { want_octave: false },
            break_hz: Some(DEFAULT_LOW_D_HZ),
            rms_floor: Some(0.05),
            recalibrate: false,
        }
    }

    fn cut_cfg() -> AttemptConfig {
        AttemptConfig {
            sample_rate: 44100,
            hop_ms: 30.0,
            mode: AttemptMode::Ornament {
                note: NoteName::A4,
                gesture: GestureKind::Cut,
            },
            break_hz: Some(DEFAULT_LOW_D_HZ),
            rms_floor: Some(0.05),
            recalibrate: false,
        }
    }

    #[test]
    fn an_early_break_stops_on_the_short_tail_and_keeps_the_target() {
        let hz = DEFAULT_LOW_D_HZ;
        let sounding = run(low_hold_cfg(), &synth::fixture_early_break(44100, hz));
        assert_eq!(sounding.evidence, Evidence::EarlyBreak);
        assert_eq!(sounding.target_hz, Some(hz));
        assert!(sounding.frames.last().is_some_and(|frame| !frame.stopped));

        let mut brief = synth::fixture_early_break(44100, hz);
        brief.extend(synth::silence(44100, 0.4));
        let held = run(low_hold_cfg(), &brief);
        assert!(held.frames.iter().all(|frame| !frame.stopped));
        assert_eq!(held.target_hz, Some(hz));

        let mut quiet = synth::fixture_early_break(44100, hz);
        quiet.extend(synth::silence(44100, 0.7));
        let stopped = run(low_hold_cfg(), &quiet);
        assert_eq!(stopped.evidence, Evidence::EarlyBreak);
        assert!(!stopped.settled);
        assert_eq!(stopped.target_hz, Some(hz));
        assert!(stopped.frames.iter().any(|frame| !frame.stopped));
        assert!(stopped.frames.last().is_some_and(|frame| frame.stopped));
    }

    #[test]
    fn an_ornament_gap_is_shorter_than_a_stop() {
        let body = target_hz(DEFAULT_LOW_D_HZ, NoteName::A4);
        let mut gap = synth::sine(body, 44100, 0.4, 0.35);
        gap.extend(synth::silence(44100, 0.4));
        gap.extend(synth::sine(body, 44100, 0.4, 0.35));
        let through = run(cut_cfg(), &gap);
        assert!(through.frames.iter().all(|frame| !frame.stopped));

        let mut quiet = synth::sine(body, 44100, 0.5, 0.35);
        quiet.extend(synth::silence(44100, 0.9));
        let stopped = run(cut_cfg(), &quiet);
        assert!(stopped.frames.iter().any(|frame| !frame.stopped));
        assert!(stopped.frames.last().is_some_and(|frame| frame.stopped));
    }
}
