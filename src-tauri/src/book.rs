//! Local copy of the Low Whistle Book recordings. The mp3s stay off git.
//! `tracks.json` names the file for a step, and the whistled passages inside it.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::pack::resolve_pack_dir;

#[derive(Debug, Clone, Deserialize)]
pub struct Clip {
    pub pack_id: String,
    pub node_id: String,
    pub step: String,
    pub file: String,
    pub spans: Vec<[f32; 2]>,
}

#[derive(Debug, Deserialize)]
struct Catalog {
    clips: Vec<Clip>,
}

pub fn book_dir() -> Option<PathBuf> {
    let door = resolve_pack_dir();
    let root = door.parent()?.parent()?;
    let dir = root.join("book");
    dir.is_dir().then_some(dir)
}

pub fn load_clips(dir: &Path) -> Result<Vec<Clip>, String> {
    let text = std::fs::read_to_string(dir.join("tracks.json"))
        .map_err(|e| format!("book tracks: {e}"))?;
    let catalog: Catalog = serde_json::from_str(&text).map_err(|e| format!("book tracks: {e}"))?;
    for clip in &catalog.clips {
        check_clip(clip)?;
    }
    Ok(catalog.clips)
}

fn check_clip(clip: &Clip) -> Result<(), String> {
    if !safe_name(&clip.file) {
        return Err(format!("bad book file {}", clip.file));
    }
    if clip.spans.is_empty() {
        return Err(format!("no whistle in {}", clip.file));
    }
    for span in &clip.spans {
        if !(span[0] >= 0.0 && span[1] > span[0]) {
            return Err(format!("bad span in {}", clip.file));
        }
    }
    Ok(())
}

fn safe_name(file: &str) -> bool {
    !file.is_empty()
        && !file.contains('/')
        && !file.contains('\\')
        && !file.contains("..")
        && file.ends_with(".mp3")
}

pub fn find_clips<'a>(
    clips: &'a [Clip],
    pack_id: &str,
    node_id: &str,
    step: &str,
) -> Vec<&'a Clip> {
    clips
        .iter()
        .filter(|clip| clip.pack_id == pack_id && clip.node_id == node_id && clip.step == step)
        .collect()
}

pub fn allowed_file<'a>(clips: &'a [Clip], file: &str) -> Option<&'a Clip> {
    if !safe_name(file) {
        return None;
    }
    clips.iter().find(|clip| clip.file == file)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clips() -> Vec<Clip> {
        let dir = book_dir().expect("book dir");
        load_clips(&dir).expect("tracks")
    }

    #[test]
    fn a_note_and_a_tune_point_at_the_whistle() {
        let clips = clips();
        let b = find_clips(&clips, "book-staircase", "book_stair", "B4");
        assert_eq!(b[0].file, "004 B 1 finger Note.mp3");
        assert!(b[0].spans[0][0] > 5.0);
        let grace = find_clips(&clips, "amazing-grace", "lines", "");
        assert_eq!(grace[0].file, "029 Amazing Grace.mp3");
        assert!(grace[0].spans[0][0] > 10.0);
        assert!(find_clips(&clips, "book-staircase", "book_stair", "A4")[0]
            .file
            .starts_with("005"));
        assert!(find_clips(&clips, "may-morning-dew", "air_may_morning_dew", "").is_empty());
    }

    #[test]
    fn the_door_cut_tap_roll_and_octave_use_the_book() {
        let clips = clips();
        let cut = find_clips(&clips, "may-morning-dew", "orn_cut", "");
        assert_eq!(cut.len(), 1);
        assert_eq!(cut[0].file, "034 Rolls General Intro.mp3");
        assert!(cut[0].spans[0][0] > 10.0 && cut[0].spans[0][1] < 25.0);
        let tap = find_clips(&clips, "may-morning-dew", "orn_tap", "");
        assert_eq!(tap[0].file, cut[0].file);
        assert!(tap[0].spans[0][0] >= cut[0].spans[0][1] - 0.2);
        let roll = find_clips(&clips, "may-morning-dew", "orn_roll", "");
        assert_eq!(
            roll.iter().map(|clip| clip.file.as_str()).collect::<Vec<_>>(),
            vec![
                "034 Rolls General Intro.mp3",
                "037 Roll on A  1.mp3",
                "038 Roll on A 2.mp3",
            ]
        );
        let octave = find_clips(&clips, "may-morning-dew", "breath_octave", "D5");
        assert_eq!(octave.len(), 1);
        assert!(octave[0].file.starts_with("012"));
        assert!(octave[0].spans[0][1] > 18.0);
        assert!(find_clips(&clips, "may-morning-dew", "breath_octave", "D4").is_empty());
    }

    #[test]
    fn a_path_outside_the_list_is_refused() {
        let clips = clips();
        assert!(allowed_file(&clips, "../pack/may-morning-dew/ref/D4.wav").is_none());
        assert!(allowed_file(&clips, "001 Getting Started.mp3").is_none());
        assert!(allowed_file(&clips, "029 Amazing Grace.mp3").is_some());
    }
}
