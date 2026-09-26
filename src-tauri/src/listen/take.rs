//! Grade one labeled take: a wav plus a sidecar. Used by `listen grade-take`.
//! Real takes stay under `corpus/`. The checked-in sidecars use synthetic audio.

use std::path::Path;

use serde::Deserialize;

use super::attempt::{AttemptConfig, AttemptEngine, AttemptMode};
use super::evidence::Evidence;
use super::ornaments::GestureKind;
use super::synth;
use super::types::NoteName;

#[derive(Debug, Clone, Deserialize)]
pub struct TakeSpec {
    /// Wav path, relative to the sidecar file.
    pub wav: String,
    /// `first_sound`, `breath_octave`, `single_note`, `phrase`, `on_the_breath`, `ornament`.
    pub mode: String,
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub gesture: String,
    #[serde(default)]
    pub breaths: Vec<usize>,
    #[serde(default)]
    pub want_octave: bool,
    #[serde(default)]
    pub break_hz: Option<f32>,
    #[serde(default)]
    pub rms_floor: Option<f32>,
    #[serde(default)]
    pub hop_ms: Option<f32>,
    pub expect_evidence: String,
    pub expect_settled: bool,
}

#[derive(Debug, Clone)]
pub struct TakeGrade {
    pub evidence: Evidence,
    pub settled: bool,
    pub pass: bool,
}

pub fn load_spec(path: &Path) -> Result<TakeSpec, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

pub fn grade_spec(spec_path: &Path) -> Result<TakeGrade, String> {
    let spec = load_spec(spec_path)?;
    let wav_path = spec_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(&spec.wav);
    let (sample_rate, samples) = synth::read_wav(&wav_path)?;
    grade_samples(sample_rate, &samples, &spec)
}

pub fn grade_samples(
    sample_rate: u32,
    samples: &[f32],
    spec: &TakeSpec,
) -> Result<TakeGrade, String> {
    let mode = attempt_mode(spec)?;
    let mut eng = AttemptEngine::new(AttemptConfig {
        sample_rate,
        hop_ms: spec.hop_ms.unwrap_or(30.0),
        mode,
        break_hz: spec.break_hz,
        rms_floor: spec.rms_floor,
        recalibrate: false,
    });
    for chunk in samples.chunks(1024) {
        eng.push_samples(chunk);
    }
    let result = eng.finish();
    let got = evidence_name(result.evidence);
    let pass = got == spec.expect_evidence && result.settled == spec.expect_settled;
    Ok(TakeGrade {
        evidence: result.evidence,
        settled: result.settled,
        pass,
    })
}

fn evidence_name(evidence: Evidence) -> String {
    serde_json::to_value(evidence)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{evidence:?}"))
}

fn attempt_mode(spec: &TakeSpec) -> Result<AttemptMode, String> {
    match spec.mode.as_str() {
        "first_sound" => Ok(AttemptMode::FirstSound),
        "breath_octave" => Ok(AttemptMode::BreathOctave {
            want_octave: spec.want_octave,
        }),
        "single_note" => Ok(AttemptMode::SingleNote {
            note: parse_note(&spec.note)?,
        }),
        "phrase" => Ok(AttemptMode::Phrase {
            notes: parse_notes(&spec.notes)?,
            breaths: spec.breaths.clone(),
            marks: vec![],
        }),
        "on_the_breath" => Ok(AttemptMode::OnTheBreath {
            notes: parse_notes(&spec.notes)?,
        }),
        "ornament" => Ok(AttemptMode::Ornament {
            note: parse_note(&spec.note)?,
            gesture: GestureKind::from_str(&spec.gesture)
                .ok_or_else(|| format!("unknown gesture {}", spec.gesture))?,
        }),
        other => Err(format!("unknown take mode {other}")),
    }
}

fn parse_note(name: &str) -> Result<NoteName, String> {
    NoteName::from_str(name).ok_or_else(|| format!("unknown note {name}"))
}

fn parse_notes(names: &[String]) -> Result<Vec<NoteName>, String> {
    names.iter().map(|n| parse_note(n)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::listen::synth;
    use crate::listen::types::{target_hz, NoteName, DEFAULT_LOW_D_HZ};

    fn grade_fixture(spec_name: &str, samples: &[f32]) {
        let spec_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../bench/takes/{spec_name}"));
        let spec = load_spec(&spec_path).unwrap_or_else(|e| panic!("{spec_name}: {e}"));
        let grade = grade_samples(44100, samples, &spec).expect("grade");
        assert!(
            grade.pass,
            "{spec_name}: evidence={:?} settled={} expected={} settled={}",
            grade.evidence, grade.settled, spec.expect_evidence, spec.expect_settled
        );
    }

    #[test]
    fn checked_in_sidecars_match_synthetic_audio() {
        let hz = DEFAULT_LOW_D_HZ;
        let body = target_hz(hz, NoteName::A4);
        grade_fixture("short_cut.json", &synth::fixture_cut(44100, body));
        grade_fixture("noise.json", &synth::fixture_noise(44100));
    }
}
