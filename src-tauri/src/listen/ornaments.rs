//! Ornament contour parser — cut / tap / roll. Abstains on messy contour.

use serde::{Deserialize, Serialize};

use super::evidence::Evidence;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContourPoint {
    pub t_ms: f32,
    pub hz: Option<f32>,
    pub rms: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureKind {
    Cut,
    Tap,
    Roll,
    /// Wave-1 roll on a shorter body. A gap in the body means it split.
    ShortRoll,
    Slide,
    Cran,
    DoubleTap,
    Triplet,
}

impl GestureKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "cut" => Some(GestureKind::Cut),
            "tap" => Some(GestureKind::Tap),
            "roll" => Some(GestureKind::Roll),
            "short_roll" => Some(GestureKind::ShortRoll),
            "slide" => Some(GestureKind::Slide),
            "cran" => Some(GestureKind::Cran),
            "double_tap" => Some(GestureKind::DoubleTap),
            "triplet" => Some(GestureKind::Triplet),
            _ => None,
        }
    }
}

const BLIP_MAX_MS: f32 = 70.0;
const MIN_JUMP_CENTS: f32 = 80.0;

/// Parse a pitch contour for a gesture on a sustained body note.
pub fn parse_gesture(contour: &[ContourPoint], body_hz: f32, kind: GestureKind) -> Evidence {
    if contour.len() < 4 || body_hz <= 0.0 {
        return Evidence::Abstain;
    }

    // Require a reasonably stable body.
    let voiced: Vec<(f32, f32)> = contour
        .iter()
        .filter_map(|p| p.hz.map(|h| (p.t_ms, h)))
        .collect();
    if voiced.len() < 4 {
        return Evidence::Abstain;
    }

    // Contour stability: too many large swings → abstain.
    let mut big_swings = 0;
    for w in voiced.windows(2) {
        let cents = 1200.0 * (w[1].1 / w[0].1).log2().abs();
        if cents > 400.0 {
            big_swings += 1;
        }
    }
    if big_swings > 4 {
        return Evidence::Abstain;
    }

    let cut = find_blip(&voiced, body_hz, true);
    let tap = find_blip(&voiced, body_hz, false);

    match kind {
        GestureKind::Cut => match cut {
            Some(dur) if dur > BLIP_MAX_MS => Evidence::CutTooLong,
            Some(_) => Evidence::Abstain, // success → silent / phrase remark from UI
            None => Evidence::Abstain,
        },
        GestureKind::Tap => match tap {
            Some(_) => Evidence::Abstain,
            None => Evidence::TapMissing,
        },
        GestureKind::Roll => roll_evidence(&voiced, body_hz, cut, tap),
        GestureKind::ShortRoll => {
            if voiced_gap_ms(&voiced) > 80.0 {
                return Evidence::BecameNotes;
            }
            roll_evidence(&voiced, body_hz, cut, tap)
        }
        GestureKind::Slide => slide_evidence(&voiced, body_hz),
        // Cran, double tap, and triplet may abstain. A blip that became a note is still named.
        GestureKind::Cran | GestureKind::DoubleTap | GestureKind::Triplet => {
            if cut_became_note(contour, body_hz) {
                Evidence::CutTooLong
            } else {
                Evidence::Abstain
            }
        }
    }
}

fn roll_evidence(
    voiced: &[(f32, f32)],
    body_hz: f32,
    cut: Option<f32>,
    tap: Option<f32>,
) -> Evidence {
    match (cut, tap) {
        (Some(c), Some(t)) if c <= BLIP_MAX_MS && t <= BLIP_MAX_MS => {
            if cut_time(voiced, body_hz, true) < cut_time(voiced, body_hz, false) {
                Evidence::Abstain
            } else {
                Evidence::TapMissing
            }
        }
        (Some(c), _) if c > BLIP_MAX_MS => Evidence::CutTooLong,
        (_, None) => Evidence::TapMissing,
        _ => Evidence::Abstain,
    }
}

fn voiced_gap_ms(voiced: &[(f32, f32)]) -> f32 {
    voiced
        .windows(2)
        .map(|w| w[1].0 - w[0].0)
        .fold(0.0, f32::max)
}

/// A slide arrives when the contour moves and the end sits on the body.
/// A clear move that finishes away is the miss. Anything else abstains.
fn slide_evidence(voiced: &[(f32, f32)], body_hz: f32) -> Evidence {
    if voiced.len() < 4 {
        return Evidence::Abstain;
    }
    let cents = |h: f32| 1200.0 * (h / body_hz).log2();
    let end = cents(voiced.last().unwrap().1).abs();
    let moved = voiced.iter().any(|(_, h)| cents(*h).abs() >= MIN_JUMP_CENTS);
    if end > 150.0 && moved {
        Evidence::SlideMissed
    } else {
        Evidence::Abstain
    }
}

fn find_blip(voiced: &[(f32, f32)], body_hz: f32, upward: bool) -> Option<f32> {
    let mut i = 0;
    while i + 2 < voiced.len() {
        let (_, h0) = voiced[i];
        let cents0 = 1200.0 * (h0 / body_hz).log2();
        if cents0.abs() > 60.0 {
            i += 1;
            continue;
        }
        // Look ahead for a jump away and return
        let mut j = i + 1;
        let mut peak_cents = 0.0f32;
        let mut peak_t = voiced[i].0;
        while j < voiced.len() {
            let (t, h) = voiced[j];
            let cents = 1200.0 * (h / body_hz).log2();
            let directed = if upward { cents } else { -cents };
            if directed > peak_cents {
                peak_cents = directed;
                peak_t = t;
            }
            if cents.abs() < 60.0 && j > i + 1 && peak_cents >= MIN_JUMP_CENTS {
                let dur = peak_t - voiced[i].0;
                // require return
                return Some(dur.max(voiced[j].0 - voiced[i].0));
            }
            if t - voiced[i].0 > 120.0 {
                break;
            }
            j += 1;
        }
        i += 1;
    }
    None
}

fn cut_time(voiced: &[(f32, f32)], body_hz: f32, upward: bool) -> f32 {
    // Approximate start time of first matching blip
    let mut i = 0;
    while i + 2 < voiced.len() {
        let (_, h0) = voiced[i];
        let cents0 = 1200.0 * (h0 / body_hz).log2();
        if cents0.abs() > 60.0 {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < voiced.len() {
            let (t, h) = voiced[j];
            let cents = 1200.0 * (h / body_hz).log2();
            let directed = if upward { cents } else { -cents };
            if directed >= MIN_JUMP_CENTS {
                return t;
            }
            if t - voiced[i].0 > 120.0 {
                break;
            }
            j += 1;
        }
        i += 1;
    }
    f32::MAX
}

/// Detect a cut that became a full note (> ~100ms held off-body).
pub fn cut_became_note(contour: &[ContourPoint], body_hz: f32) -> bool {
    let voiced: Vec<(f32, f32)> = contour
        .iter()
        .filter_map(|p| p.hz.map(|h| (p.t_ms, h)))
        .collect();
    let mut off_start: Option<f32> = None;
    for (t, h) in voiced {
        let cents = 1200.0 * (h / body_hz).log2();
        if cents > MIN_JUMP_CENTS {
            if off_start.is_none() {
                off_start = Some(t);
            } else if t - off_start.unwrap() > 100.0 {
                return true;
            }
        } else {
            off_start = None;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_short_roll_with_no_contour_abstains() {
        assert_eq!(
            parse_gesture(&[], 440.0, GestureKind::ShortRoll),
            Evidence::Abstain
        );
        let messy: Vec<ContourPoint> = (0..8)
            .map(|i| ContourPoint {
                t_ms: i as f32 * 20.0,
                hz: Some(if i % 2 == 0 { 440.0 } else { 880.0 }),
                rms: 0.3,
            })
            .collect();
        assert_eq!(
            parse_gesture(&messy, 440.0, GestureKind::ShortRoll),
            Evidence::Abstain
        );
    }
}
