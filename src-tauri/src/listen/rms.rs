//! RMS as breath-energy proxy.

pub fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum: f32 = samples.iter().map(|s| s * s).sum();
    (sum / samples.len() as f32).sqrt()
}

/// True if energy is above a quiet floor (tone present vs silence).
pub fn is_sounding(rms_val: f32, floor: f32) -> bool {
    let threshold = floor.max(0.008);
    rms_val >= threshold * 1.4
}
