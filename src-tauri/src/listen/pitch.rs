//! YIN-family pitch tracker, band-limited for low D (~250–1600 Hz).

use super::types::{BAND_HIGH_HZ, BAND_LOW_HZ};

const YIN_THRESHOLD: f32 = 0.15;

#[derive(Debug, Clone, Copy)]
pub struct PitchEstimate {
    pub hz: Option<f32>,
    pub confidence: f32,
}

/// Estimate fundamental with YIN on a mono buffer.
pub fn estimate_pitch(samples: &[f32], sample_rate: u32) -> PitchEstimate {
    let n = samples.len();
    if n < 64 || sample_rate == 0 {
        return PitchEstimate {
            hz: None,
            confidence: 0.0,
        };
    }

    let max_tau = ((sample_rate as f32 / BAND_LOW_HZ) as usize).min(n / 2);
    let min_tau = ((sample_rate as f32 / BAND_HIGH_HZ) as usize).max(2);
    if min_tau >= max_tau {
        return PitchEstimate {
            hz: None,
            confidence: 0.0,
        };
    }

    let mut yin = vec![0.0f32; max_tau + 1];
    // Difference function
    for tau in 1..=max_tau {
        let mut sum = 0.0f32;
        for i in 0..(n - max_tau) {
            let delta = samples[i] - samples[i + tau];
            sum += delta * delta;
        }
        yin[tau] = sum;
    }

    // Cumulative mean normalized difference
    yin[0] = 1.0;
    let mut running = 0.0f32;
    for tau in 1..=max_tau {
        running += yin[tau];
        yin[tau] = if running > 0.0 {
            yin[tau] * tau as f32 / running
        } else {
            1.0
        };
    }

    // Absolute threshold
    let mut tau_est = 0usize;
    let mut found = false;
    for tau in min_tau..=max_tau {
        if yin[tau] < YIN_THRESHOLD {
            // Local minimum
            let mut t = tau;
            while t + 1 <= max_tau && yin[t + 1] < yin[t] {
                t += 1;
            }
            tau_est = t;
            found = true;
            break;
        }
    }

    if !found {
        // Take global min in band as weak estimate
        let mut best = min_tau;
        for tau in (min_tau + 1)..=max_tau {
            if yin[tau] < yin[best] {
                best = tau;
            }
        }
        if yin[best] < 0.4 {
            tau_est = best;
            found = true;
        } else {
            return PitchEstimate {
                hz: None,
                confidence: 0.0,
            };
        }
    }

    let conf = (1.0 - yin[tau_est]).clamp(0.0, 1.0);
    let refined = parabolic_interpolate(&yin, tau_est);
    let hz = sample_rate as f32 / refined;
    if !(BAND_LOW_HZ..=BAND_HIGH_HZ).contains(&hz) {
        return PitchEstimate {
            hz: None,
            confidence: 0.0,
        };
    }

    let _ = found;
    PitchEstimate {
        hz: Some(hz),
        confidence: conf,
    }
}

fn parabolic_interpolate(yin: &[f32], tau: usize) -> f32 {
    if tau == 0 || tau + 1 >= yin.len() {
        return tau as f32;
    }
    let s0 = yin[tau - 1];
    let s1 = yin[tau];
    let s2 = yin[tau + 1];
    let denom = 2.0 * (2.0 * s1 - s2 - s0);
    if denom.abs() < 1e-12 {
        return tau as f32;
    }
    tau as f32 + (s0 - s2) / denom
}

/// Prefer a pitch near `expected_hz` (score-guided), reducing octave jumps.
pub fn search_near_expected(
    samples: &[f32],
    sample_rate: u32,
    expected_hz: f32,
    cents_window: f32,
) -> PitchEstimate {
    let raw = estimate_pitch(samples, sample_rate);
    let Some(hz) = raw.hz else {
        return raw;
    };

    // If within window, keep.
    let cents = 1200.0 * (hz / expected_hz).log2().abs();
    if cents <= cents_window {
        return raw;
    }

    // Check half / double (common octave error from strong second harmonic).
    for factor in [0.5f32, 2.0] {
        let candidate = hz * factor;
        if !(BAND_LOW_HZ..=BAND_HIGH_HZ).contains(&candidate) {
            continue;
        }
        let c = 1200.0 * (candidate / expected_hz).log2().abs();
        if c <= cents_window {
            return PitchEstimate {
                hz: Some(candidate),
                confidence: raw.confidence * 0.9,
            };
        }
    }

    // Outside expected: still return raw with lower confidence so attempt can abstain.
    PitchEstimate {
        hz: Some(hz),
        confidence: raw.confidence * 0.5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sine(hz: f32, sr: u32, secs: f32) -> Vec<f32> {
        let n = (sr as f32 * secs) as usize;
        (0..n)
            .map(|i| (2.0 * std::f32::consts::PI * hz * i as f32 / sr as f32).sin() * 0.5)
            .collect()
    }

    #[test]
    fn tracks_low_d() {
        let sr = 44100;
        let samples = sine(293.66, sr, 0.08);
        let est = estimate_pitch(&samples, sr);
        let hz = est.hz.expect("pitch");
        let cents = 1200.0 * (hz / 293.66).log2().abs();
        assert!(cents < 30.0, "got {hz} Hz, {cents} cents off");
    }

    #[test]
    fn tracks_octave_d() {
        let sr = 44100;
        let samples = sine(587.33, sr, 0.08);
        let est = estimate_pitch(&samples, sr);
        let hz = est.hz.expect("pitch");
        let cents = 1200.0 * (hz / 587.33).log2().abs();
        assert!(cents < 30.0, "got {hz} Hz");
    }
}
