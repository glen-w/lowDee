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
    let moved = voiced
        .iter()
        .any(|(_, h)| cents(*h).abs() >= MIN_JUMP_CENTS);
    if end > 150.0 && moved {
        Evidence::SlideMissed
    } else {
        Evidence::Abstain
    }
}

/// Duration of one directed blip: time off the body, from the jump until the return.
/// The clock starts when the pitch leaves, not at the beginning of the look-ahead.
fn find_blip(voiced: &[(f32, f32)], body_hz: f32, upward: bool) -> Option<f32> {
    let mut i = 0;
    while i + 2 < voiced.len() {
        let (t0, h0) = voiced[i];
        let cents0 = 1200.0 * (h0 / body_hz).log2();
        if cents0.abs() > 60.0 {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        let mut left_at: Option<f32> = None;
        let mut peak_cents = 0.0f32;
        while j < voiced.len() {
            let (t, h) = voiced[j];
            let cents = 1200.0 * (h / body_hz).log2();
            let directed = if upward { cents } else { -cents };
            if directed > 60.0 {
                if left_at.is_none() {
                    left_at = Some(t);
                }
                if directed > peak_cents {
                    peak_cents = directed;
                }
            } else if cents.abs() < 60.0 && left_at.is_some() {
                if peak_cents >= MIN_JUMP_CENTS {
                    return Some((t - left_at.unwrap()).max(0.0));
                }
                // A wobble, not a gesture. Start again after it.
                break;
            }
            if t - t0 > 500.0 {
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
    fn a_short_blip_is_under_seventy_ms_and_a_long_one_is_not() {
        let body = 440.0;
        let short = blip_contour(body, body * 1.2, 40.0);
        let long = blip_contour(body, body * 1.2, 150.0);
        let short_dur = find_blip(&voiced(&short), body, true).unwrap();
        let long_dur = find_blip(&voiced(&long), body, true).unwrap();
        assert!(short_dur <= BLIP_MAX_MS, "{short_dur}");
        assert!(long_dur > BLIP_MAX_MS, "{long_dur}");
        assert_eq!(
            parse_gesture(&short, body, GestureKind::Cut),
            Evidence::Abstain
        );
        assert_eq!(
            parse_gesture(&long, body, GestureKind::Cut),
            Evidence::CutTooLong
        );
    }

    fn voiced(contour: &[ContourPoint]) -> Vec<(f32, f32)> {
        contour
            .iter()
            .filter_map(|p| p.hz.map(|h| (p.t_ms, h)))
            .collect()
    }

    fn blip_contour(body: f32, jump: f32, blip_ms: f32) -> Vec<ContourPoint> {
        let mut pts = Vec::new();
        let mut t = 0.0;
        while t < 200.0 {
            pts.push(ContourPoint {
                t_ms: t,
                hz: Some(body),
                rms: 0.3,
            });
            t += 10.0;
        }
        let start = t;
        while t < start + blip_ms {
            pts.push(ContourPoint {
                t_ms: t,
                hz: Some(jump),
                rms: 0.3,
            });
            t += 10.0;
        }
        let back = t;
        while t < back + 200.0 {
            pts.push(ContourPoint {
                t_ms: t,
                hz: Some(body),
                rms: 0.3,
            });
            t += 10.0;
        }
        pts
    }

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
