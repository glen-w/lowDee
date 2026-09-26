//! Local lyrics and a melody, fetched for personal practice. Not a pack.
//! `songs/library` stays off git. A sheet with any other licence is ignored.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::pack::resolve_pack_dir;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MelodyNote {
    pub note: String,
    pub beats: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SongSheet {
    pub pack_id: String,
    pub title: String,
    #[serde(default)]
    pub queries: Vec<String>,
    pub verses: Vec<String>,
    pub credit: String,
    pub licence: String,
    pub source_name: String,
    pub source_url: String,
    #[serde(default)]
    pub also_name: String,
    #[serde(default)]
    pub also_url: String,
    pub retrieved: String,
    pub melody: Vec<MelodyNote>,
    pub playable: bool,
    pub folded: bool,
    #[serde(default)]
    pub trimmed: bool,
}

pub fn songs_dir() -> Option<PathBuf> {
    let door = resolve_pack_dir();
    let root = door.parent()?.parent()?;
    let dir = root.join("songs").join("library");
    dir.is_dir().then_some(dir)
}

pub fn find_sheet(dir: &Path, pack_id: &str, title: &str, aka: &[String]) -> Option<SongSheet> {
    if let Some(sheet) = read_named(dir, pack_id) {
        return Some(sheet);
    }
    let entries = std::fs::read_dir(dir).ok()?;
    for (n, entry) in entries.flatten().enumerate() {
        if n >= 200 {
            break;
        }
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        if name == pack_id || !safe_id(name) {
            continue;
        }
        let Some(sheet) = read_named(dir, name) else {
            continue;
        };
        if sheet_matches(&sheet, title, aka) {
            return Some(sheet);
        }
    }
    None
}

fn read_named(dir: &Path, pack_id: &str) -> Option<SongSheet> {
    if !safe_id(pack_id) {
        return None;
    }
    let path = dir.join(format!("{pack_id}.json"));
    if !path.is_file() {
        return None;
    }
    let text = std::fs::read_to_string(&path).ok()?;
    if text.len() > 200_000 {
        return None;
    }
    let sheet: SongSheet = serde_json::from_str(&text).ok()?;
    if sheet.pack_id != pack_id {
        return None;
    }
    check_sheet(&sheet).ok()?;
    Some(sheet)
}

fn sheet_matches(sheet: &SongSheet, title: &str, aka: &[String]) -> bool {
    let want: Vec<String> = std::iter::once(title)
        .chain(aka.iter().map(String::as_str))
        .map(norm_title)
        .filter(|item| !item.is_empty())
        .collect();
    let have: Vec<String> = std::iter::once(sheet.title.as_str())
        .chain(sheet.queries.iter().map(String::as_str))
        .map(norm_title)
        .filter(|item| !item.is_empty())
        .collect();
    want.iter().any(|left| have.iter().any(|right| left == right))
}

fn norm_title(value: &str) -> String {
    let text = value.to_lowercase().replace('’', "'");
    let text = text.replace("salley", "sally").replace("gardens", "garden");
    let mut out = String::new();
    let mut pending = false;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending && !out.is_empty() {
                out.push(' ');
            }
            pending = false;
            out.push(ch);
        } else {
            pending = true;
        }
    }
    out
}

fn safe_id(id: &str) -> bool {
    if id.is_empty() || id.len() > 64 {
        return false;
    }
    let mut hyphen = true;
    for ch in id.chars() {
        if ch.is_ascii_lowercase() || ch.is_ascii_digit() {
            hyphen = false;
            continue;
        }
        if ch == '-' && !hyphen {
            hyphen = true;
            continue;
        }
        return false;
    }
    !hyphen
}

fn check_sheet(sheet: &SongSheet) -> Result<(), String> {
    if sheet.licence != "personal" {
        return Err("licence".into());
    }
    if sheet.title.is_empty() || sheet.title.len() > 160 || sheet.credit.is_empty() || sheet.credit.len() > 1200
    {
        return Err("credit".into());
    }
    if sheet.source_name.is_empty() || sheet.source_name.len() > 120 {
        return Err("source".into());
    }
    if !sheet.retrieved.chars().all(|ch| ch.is_ascii_digit() || ch == '-') || sheet.retrieved.len() != 10 {
        return Err("retrieved".into());
    }
    if !allowed_source(&sheet.source_url) {
        return Err("url".into());
    }
    if !sheet.also_url.is_empty() && !allowed_source(&sheet.also_url) {
        return Err("url".into());
    }
    if sheet.verses.is_empty() && !sheet.playable {
        return Err("empty".into());
    }
    if sheet.verses.len() > 40 {
        return Err("verses".into());
    }
    for verse in &sheet.verses {
        if verse.is_empty() || verse.len() > 1800 || verse.contains('<') || verse.contains('>') {
            return Err("verse".into());
        }
    }
    if sheet.credit.contains('<') || sheet.credit.contains('>') || sheet.title.contains('<') {
        return Err("markup".into());
    }
    if sheet.playable && !sheet.melody.iter().any(|note| note.note != "rest") {
        return Err("melody".into());
    }
    if sheet.melody.len() > 200 {
        return Err("melody".into());
    }
    for note in &sheet.melody {
        if !note_ok(&note.note) || !(0.2..=8.0).contains(&note.beats) {
            return Err("note".into());
        }
    }
    Ok(())
}

fn note_ok(note: &str) -> bool {
    if note == "rest" {
        return true;
    }
    let mut chars = note.chars();
    let Some(letter) = chars.next() else {
        return false;
    };
    if !matches!(letter, 'A'..='G') {
        return false;
    }
    let rest: String = chars.collect();
    let rest = rest.strip_prefix('#').unwrap_or(&rest);
    matches!(rest, "3" | "4" | "5" | "6")
}

fn allowed_source(url: &str) -> bool {
    let Some(rest) = url.strip_prefix("https://") else {
        return false;
    };
    if rest.contains('@') || rest.contains("..") {
        return false;
    }
    let host = rest.split('/').next().unwrap_or("");
    let host = host.split(':').next().unwrap_or("");
    matches!(
        host,
        "www.traditionalmusic.co.uk" | "traditionalmusic.co.uk" | "mudcat.org" | "www.mudcat.org"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> SongSheet {
        SongSheet {
            pack_id: "salley-gardens".into(),
            title: "Down by the Sally Gardens".into(),
            queries: vec!["Salley Gardens".into()],
            verses: vec!["Down by the salley gardens\nmy love and I did meet".into()],
            credit: "Personal practice. Not for performance.".into(),
            licence: "personal".into(),
            source_name: "Traditional Music Library".into(),
            source_url: "https://www.traditionalmusic.co.uk/song-midis/Practice_Air.htm".into(),
            also_name: String::new(),
            also_url: String::new(),
            retrieved: "2026-09-26".into(),
            melody: vec![MelodyNote {
                note: "D4".into(),
                beats: 1.0,
            }],
            playable: true,
            folded: false,
            trimmed: false,
        }
    }

    fn write(dir: &Path, sheet: &SongSheet) {
        let path = dir.join(format!("{}.json", sheet.pack_id));
        std::fs::write(path, serde_json::to_string(sheet).unwrap()).unwrap();
    }

    #[test]
    fn a_personal_sheet_loads_and_a_spelling_still_matches() {
        let dir = std::env::temp_dir().join(format!("low-d-songs-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        write(&dir, &sample());
        let found = find_sheet(&dir, "salley-gardens", "Down by the Salley Gardens", &[]).unwrap();
        assert_eq!(found.verses.len(), 1);
        assert!(norm_title("Down by the Salley Gardens") == norm_title("Down by the Sally Gardens"));

        let mut other = sample();
        other.pack_id = "extra-sally".into();
        other.title = "Down by the Salley Gardens".into();
        write(&dir, &other);
        let _ = std::fs::remove_file(dir.join("salley-gardens.json"));
        let found = find_sheet(&dir, "salley-gardens", "Down by the Salley Gardens", &[]).unwrap();
        assert_eq!(found.pack_id, "extra-sally");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_shared_licence_or_a_path_is_ignored() {
        let dir = std::env::temp_dir().join(format!("low-d-songs-bad-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let mut sheet = sample();
        sheet.licence = "share".into();
        write(&dir, &sheet);
        assert!(find_sheet(&dir, "salley-gardens", "Down by the Salley Gardens", &[]).is_none());
        assert!(find_sheet(&dir, "../salley-gardens", "x", &[]).is_none());
        assert!(check_sheet(&sample()).is_ok());
        let mut marked = sample();
        marked.credit = "<script>".into();
        assert!(check_sheet(&marked).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
