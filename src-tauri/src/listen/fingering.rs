//! Low-D hole chart. The pack file must match this table.
//! A leak is named only when the heard note differs by exactly one hole.

use super::types::{cents_between, target_hz, NoteName};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hole {
    Closed,
    Open,
    Half,
}

impl Hole {
    pub fn as_str(self) -> &'static str {
        match self {
            Hole::Closed => "closed",
            Hole::Open => "open",
            Hole::Half => "half",
        }
    }
}

/// Six holes, top (window) to bottom (bell): L1 L2 L3 R1 R2 R3.
pub fn holes_for(note: NoteName) -> [Hole; 6] {
    use Hole::{Closed, Open};
    match note {
        NoteName::D4 | NoteName::D5 => [Closed, Closed, Closed, Closed, Closed, Closed],
        NoteName::E4 | NoteName::E5 => [Closed, Closed, Closed, Closed, Closed, Open],
        NoteName::Fs4 | NoteName::Fs5 => [Closed, Closed, Closed, Closed, Open, Open],
        NoteName::G4 | NoteName::G5 => [Closed, Closed, Closed, Open, Open, Open],
        NoteName::A4 | NoteName::A5 => [Closed, Closed, Open, Open, Open, Open],
        NoteName::B4 | NoteName::B5 => [Closed, Open, Open, Open, Open, Open],
        NoteName::C5 => [Open, Closed, Closed, Open, Open, Open],
        NoteName::Cs5 => [Open, Open, Open, Open, Open, Open],
    }
}

/// Index of the single hole that differs, or none when the charts match or differ twice.
pub fn differing_hole(expected: NoteName, heard: NoteName) -> Option<u8> {
    let a = holes_for(expected);
    let b = holes_for(heard);
    let mut found: Option<u8> = None;
    for i in 0..6 {
        if a[i] != b[i] {
            if found.is_some() {
                return None;
            }
            found = Some(i as u8);
        }
    }
    found
}

/// Nearest scale note within `tol_cents` of `hz`, measured from this whistle's break.
pub fn nearest_scale_note(hz: f32, break_hz: f32, tol_cents: f32) -> Option<NoteName> {
    if hz <= 0.0 || break_hz <= 0.0 {
        return None;
    }
    let mut best: Option<(NoteName, f32)> = None;
    for note in NoteName::ALL {
        let cents = cents_between(hz, target_hz(break_hz, note)).abs();
        if cents <= tol_cents && best.map(|(_, b)| cents < b).unwrap_or(true) {
            best = Some((note, cents));
        }
    }
    best.map(|(n, _)| n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bottom_finger_is_the_only_difference_between_d_and_e() {
        assert_eq!(differing_hole(NoteName::E4, NoteName::D4), Some(5));
    }

    #[test]
    fn octave_d_is_the_same_picture() {
        assert_eq!(differing_hole(NoteName::D4, NoteName::D5), None);
    }

    #[test]
    fn two_holes_are_not_named() {
        assert_eq!(differing_hole(NoteName::G4, NoteName::D4), None);
    }

    #[test]
    fn second_octave_e_shares_the_e_holes() {
        assert_eq!(holes_for(NoteName::E5), holes_for(NoteName::E4));
        assert_eq!(differing_hole(NoteName::E5, NoteName::D5), Some(5));
        assert_eq!(NoteName::E5.semitones_from_d4(), 14);
    }
}
