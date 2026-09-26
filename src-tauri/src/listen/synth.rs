//! Synthetic audio fixtures for the listen bench.

use std::f32::consts::PI;

pub fn sine(hz: f32, sample_rate: u32, duration_secs: f32, amp: f32) -> Vec<f32> {
    let n = (sample_rate as f32 * duration_secs) as usize;
    (0..n)
        .map(|i| (2.0 * PI * hz * i as f32 / sample_rate as f32).sin() * amp)
        .collect()
}

pub fn silence(sample_rate: u32, duration_secs: f32) -> Vec<f32> {
    vec![0.0; (sample_rate as f32 * duration_secs) as usize]
}

pub fn noise(sample_rate: u32, duration_secs: f32, amp: f32) -> Vec<f32> {
    let n = (sample_rate as f32 * duration_secs) as usize;
    let mut x = 0x12345678u32;
    (0..n)
        .map(|_| {
            x = x.wrapping_mul(1664525).wrapping_add(1013904223);
            let u = (x >> 9) as f32 / (1u32 << 23) as f32;
            (u * 2.0 - 1.0) * amp
        })
        .collect()
}

/// Steady low D for settle (~10.5s).
pub fn fixture_steady_low_d(sample_rate: u32, hz: f32) -> Vec<f32> {
    sine(hz, sample_rate, 10.5, 0.35)
}

/// Low D then jump to octave mid-hold (early break).
pub fn fixture_early_break(sample_rate: u32, hz: f32) -> Vec<f32> {
    let mut v = sine(hz, sample_rate, 2.0, 0.35);
    v.extend(sine(hz * 2.0, sample_rate, 3.0, 0.4));
    v
}

/// After an early break: clean low D hold for settle.
pub fn fixture_retry_settle(sample_rate: u32, hz: f32) -> Vec<f32> {
    sine(hz, sample_rate, 10.5, 0.35)
}

/// Breath noise / no tone.
pub fn fixture_noise(sample_rate: u32) -> Vec<f32> {
    noise(sample_rate, 3.0, 0.08)
}

/// Octave D held.
pub fn fixture_octave(sample_rate: u32, hz: f32) -> Vec<f32> {
    sine(hz * 2.0, sample_rate, 2.5, 0.4)
}

/// Short upward blip (cut) on a sustained A — keep the blip under 50ms.
pub fn fixture_cut(sample_rate: u32, body_hz: f32) -> Vec<f32> {
    let mut v = sine(body_hz, sample_rate, 0.5, 0.35);
    v.extend(sine(body_hz * 1.334, sample_rate, 0.035, 0.3));
    v.extend(sine(body_hz, sample_rate, 0.7, 0.35));
    v
}

/// Cut that becomes a note (~150ms).
pub fn fixture_cut_too_long(sample_rate: u32, body_hz: f32) -> Vec<f32> {
    let mut v = sine(body_hz, sample_rate, 0.3, 0.35);
    v.extend(sine(body_hz * 1.33, sample_rate, 0.15, 0.35));
    v.extend(sine(body_hz, sample_rate, 0.4, 0.35));
    v
}

/// Phrase D E F# G with gaps (tongued).
pub fn fixture_tongued_phrase(sample_rate: u32, d_hz: f32) -> Vec<f32> {
    let notes = [0, 2, 4, 5];
    let mut v = Vec::new();
    for (i, sem) in notes.iter().enumerate() {
        if i > 0 {
            v.extend(silence(sample_rate, 0.12));
        }
        let hz = d_hz * 2f32.powf(*sem as f32 / 12.0);
        v.extend(sine(hz, sample_rate, 0.35, 0.35));
    }
    v
}

/// Joined phrase (legato).
pub fn fixture_joined_phrase(sample_rate: u32, d_hz: f32) -> Vec<f32> {
    let notes = [0, 2, 4, 5];
    let mut v = Vec::new();
    for sem in notes {
        let hz = d_hz * 2f32.powf(sem as f32 / 12.0);
        v.extend(sine(hz, sample_rate, 0.45, 0.35));
    }
    v
}

/// Two notes with a gap between them. `gap` is the silence after the first note.
pub fn fixture_two_notes_gap(sample_rate: u32, d_hz: f32, gap: f32) -> Vec<f32> {
    let mut v = sine(d_hz, sample_rate, 0.4, 0.35);
    v.extend(silence(sample_rate, gap));
    v.extend(sine(d_hz * 2f32.powf(2.0 / 12.0), sample_rate, 0.4, 0.35));
    v
}

/// C natural asked, G played — the note did not seal.
pub fn fixture_leaked_c(sample_rate: u32, d_hz: f32) -> Vec<f32> {
    let g = d_hz * 2f32.powf(5.0 / 12.0);
    sine(g, sample_rate, 1.2, 0.35)
}

/// Second-octave E that falls to the low E.
pub fn fixture_octave_e_cracks(sample_rate: u32, d_hz: f32) -> Vec<f32> {
    let e5 = d_hz * 2f32.powf(14.0 / 12.0);
    let e4 = d_hz * 2f32.powf(2.0 / 12.0);
    let mut v = sine(e5, sample_rate, 0.2, 0.35);
    v.extend(sine(e4, sample_rate, 1.2, 0.35));
    v
}

/// A dorian fragment: A G C A, joined.
pub fn fixture_a_dorian(sample_rate: u32, d_hz: f32) -> Vec<f32> {
    let mut v = Vec::new();
    for sem in [7, 5, 10, 7] {
        let hz = d_hz * 2f32.powf(sem as f32 / 12.0);
        v.extend(sine(hz, sample_rate, 0.4, 0.35));
    }
    v
}

/// Body, a long gap, body again — a short roll that became two notes.
pub fn fixture_roll_split(sample_rate: u32, body_hz: f32) -> Vec<f32> {
    let mut v = sine(body_hz, sample_rate, 0.25, 0.35);
    v.extend(silence(sample_rate, 0.16));
    v.extend(sine(body_hz, sample_rate, 0.25, 0.35));
    v
}

/// Glide that stays a whole step under the body.
pub fn fixture_slide_missed(sample_rate: u32, body_hz: f32) -> Vec<f32> {
    let below = body_hz * 2f32.powf(-2.0 / 12.0);
    let mut v = sine(below, sample_rate, 0.35, 0.35);
    v.extend(sine(body_hz * 2f32.powf(-1.0 / 12.0), sample_rate, 0.15, 0.35));
    v.extend(sine(below, sample_rate, 0.35, 0.35));
    v
}

pub fn write_wav(path: &std::path::Path, sample_rate: u32, samples: &[f32]) -> Result<(), String> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec).map_err(|e| e.to_string())?;
    for &s in samples {
        let amp = (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        writer.write_sample(amp).map_err(|e| e.to_string())?;
    }
    writer.finalize().map_err(|e| e.to_string())
}
