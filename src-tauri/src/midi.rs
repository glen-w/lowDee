//! A piano MIDI file already names each pitch. Read those notes, keep the
//! higher one when two sound together, and move the line onto this low D.

use std::collections::BTreeMap;

use crate::listen::types::NoteName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BroughtChunk {
    pub notes: Vec<String>,
    pub abc: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BroughtMelody {
    pub chunks: Vec<BroughtChunk>,
    pub placement: String,
}

#[derive(Clone)]
struct RawNote {
    start: u32,
    end: u32,
    note: u8,
    channel: u8,
}

pub fn melody_from_midi(bytes: &[u8]) -> Result<BroughtMelody, String> {
    let (division, tracks) = split_tracks(bytes)?;
    let mut best: Vec<RawNote> = Vec::new();
    for track in tracks {
        let notes = parse_track(&track)?;
        let sung: Vec<RawNote> = notes.into_iter().filter(|n| n.channel != 9).collect();
        if sung.len() > best.len() {
            best = sung;
        }
    }
    if best.is_empty() {
        return Err("no melody".into());
    }
    let (line, thinned) = top_line(&best);
    let (line, repeated) = one_pass(&line);
    let midi_notes: Vec<i32> = line.iter().map(|n| n.note as i32).collect();
    let (shift, shifted) = place(&midi_notes)?;
    let mut placement = shift_line(shift);
    if thinned {
        placement.push_str(" Where two notes sounded together, the higher one is kept.");
    }
    if repeated {
        placement.push_str(" A repeated pass is kept once.");
    }
    let tones = tones_of(&line, &shifted, division);
    let chunks = phrase_chunks(&tones);
    if chunks.is_empty() {
        return Err("no melody".into());
    }
    Ok(BroughtMelody { chunks, placement })
}

/// One quarter-note melody, for tests and for a folder that only holds a file.
pub fn quarter_note_file(notes: &[u8]) -> Vec<u8> {
    let mut track = Vec::new();
    for (i, note) in notes.iter().enumerate() {
        push_vlq(&mut track, if i == 0 { 0 } else { 480 });
        track.extend_from_slice(&[0x90, *note, 80]);
        push_vlq(&mut track, 480);
        track.extend_from_slice(&[0x80, *note, 0]);
    }
    push_vlq(&mut track, 0);
    track.extend_from_slice(&[0xFF, 0x2F, 0x00]);
    wrap_midi(&track)
}

fn wrap_midi(track: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"MThd");
    out.extend_from_slice(&6u32.to_be_bytes());
    out.extend_from_slice(&0u16.to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&480u16.to_be_bytes());
    out.extend_from_slice(b"MTrk");
    out.extend_from_slice(&(track.len() as u32).to_be_bytes());
    out.extend_from_slice(track);
    out
}

fn push_vlq(out: &mut Vec<u8>, mut n: u32) {
    let mut buf = [0u8; 4];
    let mut i = 3;
    buf[i] = (n & 0x7f) as u8;
    n >>= 7;
    while n > 0 {
        i -= 1;
        buf[i] = ((n & 0x7f) as u8) | 0x80;
        n >>= 7;
    }
    out.extend_from_slice(&buf[i..]);
}

fn split_tracks(bytes: &[u8]) -> Result<(u32, Vec<&[u8]>), String> {
    if bytes.len() < 14 || &bytes[0..4] != b"MThd" {
        return Err("not a midi file".into());
    }
    let len = u32::from_be_bytes(bytes[4..8].try_into().unwrap()) as usize;
    if len < 6 || bytes.len() < 8 + len {
        return Err("not a midi file".into());
    }
    let format = u16::from_be_bytes(bytes[8..10].try_into().unwrap());
    if format > 1 {
        return Err("midi has more than one song".into());
    }
    let division = i16::from_be_bytes(bytes[12..14].try_into().unwrap());
    if division <= 0 {
        return Err("midi uses a time code".into());
    }
    let mut i = 8 + len;
    let mut tracks = Vec::new();
    while i + 8 <= bytes.len() {
        if &bytes[i..i + 4] != b"MTrk" {
            return Err("not a midi file".into());
        }
        let tlen = u32::from_be_bytes(bytes[i + 4..i + 8].try_into().unwrap()) as usize;
        i += 8;
        if i + tlen > bytes.len() {
            return Err("midi ended early".into());
        }
        tracks.push(&bytes[i..i + tlen]);
        i += tlen;
    }
    if tracks.is_empty() {
        return Err("no melody".into());
    }
    Ok((division as u32, tracks))
}

fn read_vlq(data: &[u8], i: &mut usize) -> Result<u32, String> {
    let mut v = 0u32;
    for _ in 0..4 {
        if *i >= data.len() {
            return Err("midi ended early".into());
        }
        let b = data[*i];
        *i += 1;
        v = (v << 7) | (b & 0x7f) as u32;
        if b & 0x80 == 0 {
            return Ok(v);
        }
    }
    Err("midi length is too long".into())
}

fn parse_track(data: &[u8]) -> Result<Vec<RawNote>, String> {
    let mut i = 0usize;
    let mut tick = 0u32;
    let mut status = 0u8;
    let mut open: BTreeMap<(u8, u8), u32> = BTreeMap::new();
    let mut notes = Vec::new();
    while i < data.len() {
        let delta = read_vlq(data, &mut i)?;
        tick = tick.saturating_add(delta);
        if i >= data.len() {
            break;
        }
        let b = data[i];
        if b >= 0x80 {
            i += 1;
            if b == 0xFF {
                if i >= data.len() {
                    break;
                }
                i += 1;
                let len = read_vlq(data, &mut i)? as usize;
                if i + len > data.len() {
                    return Err("midi ended early".into());
                }
                i += len;
                status = 0;
                continue;
            }
            if b == 0xF0 || b == 0xF7 {
                let len = read_vlq(data, &mut i)? as usize;
                if i + len > data.len() {
                    return Err("midi ended early".into());
                }
                i += len;
                status = 0;
                continue;
            }
            status = b;
        }
        if status < 0x80 {
            return Err("bad midi event".into());
        }
        let needed = match status & 0xF0 {
            0xC0 | 0xD0 => 1,
            _ => 2,
        };
        if i + needed > data.len() {
            return Err("midi ended early".into());
        }
        let d1 = data[i];
        let d2 = if needed == 2 { data[i + 1] } else { 0 };
        i += needed;
        let ch = status & 0x0F;
        match status & 0xF0 {
            0x90 if d2 > 0 => {
                if let Some(start) = open.insert((ch, d1), tick) {
                    if tick > start {
                        notes.push(RawNote {
                            start,
                            end: tick,
                            note: d1,
                            channel: ch,
                        });
                    }
                }
            }
            0x80 | 0x90 => {
                if let Some(start) = open.remove(&(ch, d1)) {
                    if tick > start {
                        notes.push(RawNote {
                            start,
                            end: tick,
                            note: d1,
                            channel: ch,
                        });
                    }
                }
            }
            _ => {}
        }
    }
    for ((channel, note), start) in open {
        notes.push(RawNote {
            start,
            end: tick.max(start.saturating_add(1)),
            note,
            channel,
        });
    }
    Ok(notes)
}

/// Keep the higher pitch when a chord, or a lower note still sounding, shares the attack.
fn top_line(notes: &[RawNote]) -> (Vec<RawNote>, bool) {
    let mut notes = notes.to_vec();
    notes.sort_by_key(|n| (n.start, std::cmp::Reverse(n.note)));
    let mut line = Vec::new();
    let mut cursor = 0u32;
    let mut thinned = false;
    let mut idx = 0;
    while idx < notes.len() {
        let start = notes[idx].start;
        let mut group = Vec::new();
        while idx < notes.len() && notes[idx].start == start {
            group.push(notes[idx].clone());
            idx += 1;
        }
        if group.len() > 1 {
            thinned = true;
        }
        let top = group.iter().max_by_key(|n| n.note).unwrap().clone();
        if !line.is_empty() && top.start < cursor {
            thinned = true;
            continue;
        }
        cursor = top.end;
        line.push(top);
    }
    (line, thinned)
}

fn one_pass(line: &[RawNote]) -> (Vec<RawNote>, bool) {
    if line.len() < 2 {
        return (line.to_vec(), false);
    }
    for parts in (2..=8).rev() {
        if line.len() % parts != 0 {
            continue;
        }
        let n = line.len() / parts;
        let chunk = &line[..n];
        let mut same = true;
        for part in 1..parts {
            let slice = &line[part * n..(part + 1) * n];
            for (a, b) in chunk.iter().zip(slice) {
                if a.note != b.note || a.end - a.start != b.end - b.start {
                    same = false;
                    break;
                }
            }
            if !same {
                break;
            }
        }
        if same {
            return (chunk.to_vec(), true);
        }
    }
    (line.to_vec(), false)
}

fn on_whistle(note: i32) -> bool {
    matches!(
        note,
        62 | 64 | 66 | 67 | 69 | 71 | 72 | 73 | 74 | 76 | 78 | 79 | 81 | 83
    )
}

fn note_name(note: i32) -> NoteName {
    match note {
        62 => NoteName::D4,
        64 => NoteName::E4,
        66 => NoteName::Fs4,
        67 => NoteName::G4,
        69 => NoteName::A4,
        71 => NoteName::B4,
        72 => NoteName::C5,
        73 => NoteName::Cs5,
        74 => NoteName::D5,
        76 => NoteName::E5,
        78 => NoteName::Fs5,
        79 => NoteName::G5,
        81 => NoteName::A5,
        _ => NoteName::B5,
    }
}

fn pitch_class(note: i32) -> String {
    const NAMES: [&str; 12] = [
        "C", "C♯", "D", "E♭", "E", "F", "F♯", "G", "A♭", "A", "B♭", "B",
    ];
    NAMES[note.rem_euclid(12) as usize].to_string()
}

fn place(notes: &[i32]) -> Result<(i32, Vec<i32>), String> {
    if notes.is_empty() {
        return Err("no melody".into());
    }
    let min = *notes.iter().min().unwrap();
    let max = *notes.iter().max().unwrap();
    if max - min > 21 {
        return Err("wider than a low D".into());
    }
    if let Some(shift) = best_shift(notes, true).or_else(|| best_shift(notes, false)) {
        let shifted = notes.iter().map(|n| n + shift).collect();
        return Ok((shift, shifted));
    }
    let mut best: Option<(i32, i32, i32)> = None;
    for shift in -72..=72 {
        let on = notes.iter().filter(|n| on_whistle(*n + shift)).count() as i32;
        let abs = shift.abs();
        let better = match best {
            None => true,
            Some((best_on, best_abs, _)) => on > best_on || (on == best_on && abs < best_abs),
        };
        if better {
            best = Some((on, abs, shift));
        }
    }
    let shift = best.map(|(_, _, shift)| shift).unwrap_or(0);
    let mut missing = Vec::new();
    for n in notes {
        let sounded = n + shift;
        if !on_whistle(sounded) {
            let name = pitch_class(sounded);
            if !missing.contains(&name) {
                missing.push(name);
            }
        }
    }
    Err(format!("notes not on a low D: {}", missing.join(", ")))
}

/// An octave move keeps the tune. A smaller step that changes the key is used only when no octave fits.
fn best_shift(notes: &[i32], octave_only: bool) -> Option<i32> {
    let mut best: Option<(i32, i32)> = None;
    for shift in -72..=72 {
        if octave_only && shift % 12 != 0 {
            continue;
        }
        if notes.iter().all(|n| on_whistle(*n + shift)) {
            let abs = shift.abs();
            let better = match best {
                None => true,
                Some((best_abs, _)) => abs < best_abs,
            };
            if better {
                best = Some((abs, shift));
            }
        }
    }
    best.map(|(_, shift)| shift)
}

fn shift_line(shift: i32) -> String {
    let moved = if shift == 0 {
        "They already sit on this low D.".to_string()
    } else if shift % 12 == 0 {
        let n = shift.abs() / 12;
        let way = if shift < 0 { "down" } else { "up" };
        let octaves = if n == 1 {
            "one octave".to_string()
        } else {
            format!("{n} octaves")
        };
        format!("Moved {way} {octaves} so it sits on this low D.")
    } else {
        let way = if shift < 0 { "down" } else { "up" };
        let n = shift.abs();
        let steps = if n == 1 {
            "1 semitone".to_string()
        } else {
            format!("{n} semitones")
        };
        format!("Moved {way} {steps} so the notes sit on this low D.")
    };
    format!("The file names each pitch. {moved}")
}

#[derive(Clone)]
enum Tone {
    Note { name: NoteName, eighths: u8 },
    Rest { eighths: u8 },
}

fn quantize(ticks: u32, division: u32, min: u8) -> u8 {
    let eighth = (division / 2).max(1);
    let q = (ticks as u64 + eighth as u64 / 2) / eighth as u64;
    (q as u8).clamp(min, 8)
}

fn tones_of(line: &[RawNote], shifted: &[i32], division: u32) -> Vec<Tone> {
    let mut tones = Vec::new();
    for (i, raw) in line.iter().enumerate() {
        let eighths = quantize(raw.end.saturating_sub(raw.start), division, 1);
        tones.push(Tone::Note {
            name: note_name(shifted[i]),
            eighths,
        });
        if let Some(next) = line.get(i + 1) {
            let gap = next.start.saturating_sub(raw.end);
            let rest = quantize(gap, division, 0);
            if rest >= 1 {
                tones.push(Tone::Rest { eighths: rest });
            }
        }
    }
    tones
}

fn phrase_chunks(tones: &[Tone]) -> Vec<BroughtChunk> {
    let mut groups: Vec<Vec<Tone>> = Vec::new();
    let mut cur = Vec::new();
    for tone in tones {
        if let Tone::Rest { eighths } = tone {
            if *eighths >= 4 {
                if cur.iter().any(|t| matches!(t, Tone::Note { .. })) {
                    groups.push(std::mem::take(&mut cur));
                }
                continue;
            }
        }
        cur.push(tone.clone());
        let sounded = cur.iter().filter(|t| matches!(t, Tone::Note { .. })).count();
        if sounded >= 16 {
            groups.push(std::mem::take(&mut cur));
        }
    }
    if cur.iter().any(|t| matches!(t, Tone::Note { .. })) {
        groups.push(cur);
    }
    groups.into_iter().map(|g| chunk_of(&g)).collect()
}

fn chunk_of(tones: &[Tone]) -> BroughtChunk {
    let mut abc = String::new();
    let mut notes = Vec::new();
    let mut acc = 0u32;
    for tone in tones {
        if acc > 0 && acc % 8 == 0 {
            abc.push_str("| ");
        }
        match tone {
            Tone::Rest { eighths } => {
                abc.push_str(&abc_rest(*eighths));
                abc.push(' ');
                acc += *eighths as u32;
            }
            Tone::Note { name, eighths } => {
                abc.push_str(&abc_note(*name, *eighths));
                abc.push(' ');
                notes.push(name.as_str().to_string());
                acc += *eighths as u32;
            }
        }
    }
    BroughtChunk {
        notes,
        abc: abc.trim().to_string(),
    }
}

fn abc_note(name: NoteName, eighths: u8) -> String {
    let letter = match name {
        NoteName::D4 => "D",
        NoteName::E4 => "E",
        NoteName::Fs4 => "F",
        NoteName::G4 => "G",
        NoteName::A4 => "A",
        NoteName::B4 => "B",
        NoteName::C5 => "=c",
        NoteName::Cs5 => "c",
        NoteName::D5 => "d",
        NoteName::E5 => "e",
        NoteName::Fs5 => "f",
        NoteName::G5 => "g",
        NoteName::A5 => "a",
        NoteName::B5 => "b",
    };
    if eighths == 1 {
        letter.to_string()
    } else {
        format!("{letter}{eighths}")
    }
}

fn abc_rest(eighths: u8) -> String {
    if eighths == 1 {
        "z".into()
    } else {
        format!("z{eighths}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_high_melody_moves_down_one_octave() {
        let melody = melody_from_midi(&quarter_note_file(&[86, 88, 90])).unwrap();
        assert_eq!(
            melody.chunks[0].notes,
            vec!["D5".to_string(), "E5".to_string(), "F#5".to_string()]
        );
        assert!(melody.placement.contains("down one octave"), "{}", melody.placement);
        assert!(melody.placement.contains("names each pitch"));
    }

    #[test]
    fn a_melody_already_in_range_stays() {
        let melody = melody_from_midi(&quarter_note_file(&[74, 76, 78])).unwrap();
        assert_eq!(melody.chunks[0].notes[0], "D5");
        assert!(melody.placement.contains("already sit"));
    }

    #[test]
    fn a_key_that_is_not_d_is_moved_onto_the_scale() {
        // F A C becomes G B D.
        let melody = melody_from_midi(&quarter_note_file(&[65, 69, 72])).unwrap();
        assert_eq!(
            melody.chunks[0].notes,
            vec!["G4".to_string(), "B4".to_string(), "D5".to_string()]
        );
        assert!(melody.placement.contains("2 semitones"), "{}", melody.placement);
    }

    #[test]
    fn a_chromatic_run_the_whistle_cannot_play_is_named() {
        // Five chromatic steps. This whistle's longest run is B–C–C♯–D.
        let err = melody_from_midi(&quarter_note_file(&[64, 65, 66, 67, 68])).unwrap_err();
        assert!(err.contains("notes not on a low D"), "{err}");
    }

    #[test]
    fn a_span_wider_than_the_whistle_is_refused() {
        let err = melody_from_midi(&quarter_note_file(&[60, 84])).unwrap_err();
        assert!(err.contains("wider than a low D"), "{err}");
    }

    #[test]
    fn a_repeated_pass_is_kept_once() {
        let melody = melody_from_midi(&quarter_note_file(&[74, 76, 74, 76])).unwrap();
        assert_eq!(melody.chunks[0].notes, vec!["D5".to_string(), "E5".to_string()]);
        assert!(melody.placement.contains("kept once"));
    }

    #[test]
    fn a_chord_keeps_the_higher_note() {
        let mut track = Vec::new();
        push_vlq(&mut track, 0);
        track.extend_from_slice(&[0x90, 67, 80]); // G4
        push_vlq(&mut track, 0);
        track.extend_from_slice(&[0x90, 74, 80]); // D5, together
        push_vlq(&mut track, 480);
        track.extend_from_slice(&[0x80, 67, 0]);
        push_vlq(&mut track, 0);
        track.extend_from_slice(&[0x80, 74, 0]);
        push_vlq(&mut track, 0);
        track.extend_from_slice(&[0xFF, 0x2F, 0x00]);
        let melody = melody_from_midi(&wrap_midi(&track)).unwrap();
        assert_eq!(melody.chunks[0].notes, vec!["D5".to_string()]);
        assert!(melody.placement.contains("higher one"));
    }
}
