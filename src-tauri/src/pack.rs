use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::listen::fingering::{self, Hole};
use crate::listen::types::NoteName;

const HASH_FILES: &[&str] = &[
    "phrases.json",
    "fingering-low-d.json",
    "ornaments.json",
    "remarks.json",
    "tune.abc",
];

const OPTIONAL_HASH_FILES: &[&str] = &["words.json"];

const KNOWN_NODES: &[&str] = &[
    "first_sound",
    "staircase",
    "breath_octave",
    "hedwig",
    "on_the_breath",
    "air_bare",
    "orn_cut",
    "orn_tap",
    "orn_roll",
    "air_may_morning_dew",
];

const KNOWN_MODES: &[&str] = &[
    "first_sound",
    "staircase",
    "breath_octave",
    "page",
    "on_the_breath",
    "phrase",
    "ornament",
    "vibrato",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackPage {
    pub title: String,
    pub url: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackNode {
    pub id: String,
    pub mode: String,
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub high_d_body: String,
    #[serde(default)]
    pub gesture: String,
    /// Ornament demo note, when it is not the pack default.
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub hide_pictures: bool,
    #[serde(default)]
    pub grade_marks: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackManifest {
    pub id: String,
    pub version: String,
    pub track: String,
    pub title: String,
    pub node_ids: Vec<String>,
    #[serde(default)]
    pub nodes: Vec<PackNode>,
    #[serde(default)]
    pub pages: BTreeMap<String, PackPage>,
    pub content_hash: String,
    #[serde(default)]
    pub wave: u8,
    #[serde(default)]
    pub after: String,
    #[serde(default)]
    pub book_ref: String,
    #[serde(default)]
    pub shelf: String,
    #[serde(default)]
    pub rights: String,
    #[serde(default)]
    pub aka: Vec<String>,
    #[serde(default)]
    pub source: String,
    /// Set when a licence is known. Empty on a public-domain pack.
    #[serde(default)]
    pub licence: String,
    /// Set when an author is named.
    #[serde(default)]
    pub author: String,
    /// A page to look at. The app does not fetch it.
    #[serde(default)]
    pub site: String,
    /// How a brought melody was moved onto this whistle.
    #[serde(default)]
    pub placement: String,
    #[serde(default)]
    pub session: String,
    #[serde(default)]
    pub pulse: String,
    #[serde(default)]
    pub pulse_beats: u8,
    /// Fingering id written when this pack's C-natural hold settles.
    #[serde(default)]
    pub cnat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhraseChunk {
    pub id: String,
    pub label: String,
    pub notes: Vec<String>,
    pub abc: String,
    #[serde(rename = "ref")]
    pub reference: String,
    /// Note indexes after which a breath belongs.
    #[serde(default)]
    pub breaths: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreathPhrase {
    pub notes: Vec<String>,
    pub abc: String,
    #[serde(rename = "ref")]
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phrases {
    pub air_title: String,
    pub chunks: Vec<PhraseChunk>,
    pub staircase_notes: Vec<String>,
    pub on_the_breath: BreathPhrase,
    #[serde(default = "default_meter")]
    pub meter: String,
    #[serde(default = "default_key")]
    pub key: String,
}

fn default_meter() -> String {
    "3/4".into()
}

fn default_key() -> String {
    "D".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordLine {
    pub chunk_id: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Words {
    #[serde(default)]
    pub lines: Vec<WordLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoleChart {
    pub holes: Vec<String>,
    pub octave: u8,
    pub label: String,
    #[serde(default)]
    pub half: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fingering {
    pub holes_top_to_bottom: Vec<String>,
    pub notes: BTreeMap<String, HoleChart>,
    pub intervals_from_break: BTreeMap<String, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureSpec {
    #[serde(default)]
    pub max_ms: Option<u32>,
    #[serde(default)]
    pub direction: Option<String>,
    #[serde(default)]
    pub parts: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrnamentMark {
    pub chunk_id: String,
    pub note_index: usize,
    pub gesture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ornaments {
    pub gestures: BTreeMap<String, GestureSpec>,
    pub demo_note: String,
    pub marks: Vec<OrnamentMark>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Pack {
    #[serde(skip)]
    pub root: PathBuf,
    pub manifest: PackManifest,
    pub phrases: Phrases,
    pub fingering: Fingering,
    pub ornaments: Ornaments,
    pub remarks: BTreeMap<String, BTreeMap<String, String>>,
    pub tune_abc: String,
    pub words: Words,
    pub playable: bool,
    /// `pack` for the bundled chain, `teacher` for a folder on this machine.
    #[serde(skip)]
    pub origin: String,
}

pub fn resolve_pack_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let candidates = [
        manifest.join("../pack/may-morning-dew"),
        PathBuf::from("../pack/may-morning-dew"),
        PathBuf::from("pack/may-morning-dew"),
        PathBuf::from("../../pack/may-morning-dew"),
        PathBuf::from("./pack/may-morning-dew"),
    ];
    for c in candidates {
        if c.join("manifest.json").exists() {
            return c;
        }
    }
    manifest.join("../pack/may-morning-dew")
}

pub fn load_pack(root: &Path) -> Result<Pack, String> {
    load_pack_kind(root, false)
}

fn load_pack_kind(root: &Path, teacher: bool) -> Result<Pack, String> {
    let manifest_text = fs::read_to_string(root.join("manifest.json"))
        .map_err(|e| format!("manifest.json: {e}"))?;
    let mut manifest: PackManifest =
        serde_json::from_str(&manifest_text).map_err(|e| format!("manifest: {e}"))?;

    if manifest.rights == "page_only" {
        return load_page_only(root, manifest);
    }

    let read = |name: &str| -> Result<String, String> {
        fs::read_to_string(root.join(name)).map_err(|e| format!("{name}: {e}"))
    };
    let phrases: Phrases =
        serde_json::from_str(&read("phrases.json")?).map_err(|e| format!("phrases: {e}"))?;
    let fingering: Fingering = serde_json::from_str(&read("fingering-low-d.json")?)
        .map_err(|e| format!("fingering: {e}"))?;
    let ornaments: Ornaments =
        serde_json::from_str(&read("ornaments.json")?).map_err(|e| format!("ornaments: {e}"))?;
    let remarks: BTreeMap<String, BTreeMap<String, String>> =
        serde_json::from_str(&read("remarks.json")?).map_err(|e| format!("remarks: {e}"))?;
    let tune_abc = read("tune.abc")?;
    let words = if root.join("words.json").is_file() {
        serde_json::from_str(&read("words.json")?).map_err(|e| format!("words: {e}"))?
    } else {
        Words::default()
    };

    let hash = content_hash(&manifest, root)?;
    if manifest.content_hash != hash {
        return Err(format!(
            "content hash mismatch: manifest has {}, files hash to {hash}",
            manifest.content_hash
        ));
    }
    validate(&manifest, &phrases, &fingering, &ornaments)?;
    if teacher && matches!(rights_kind(&manifest.rights), "brought" | "empty") {
        manifest.rights = "brought".into();
    }
    check_rights(&manifest, teacher)?;

    Ok(Pack {
        root: root.to_path_buf(),
        manifest,
        phrases,
        fingering,
        ornaments,
        remarks,
        tune_abc,
        words,
        playable: true,
        origin: if teacher {
            "teacher".into()
        } else {
            "pack".into()
        },
    })
}

fn load_page_only(root: &Path, manifest: PackManifest) -> Result<Pack, String> {
    if root.join("tune.abc").is_file() {
        return Err(format!("{} is page only and has a tune", manifest.id));
    }
    if let Ok(rd) = fs::read_dir(root.join("ref")) {
        for entry in rd.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".wav") {
                return Err(format!("{} is page only and has audio", manifest.id));
            }
        }
    }
    let hash = content_hash(&manifest, root)?;
    if manifest.content_hash != hash {
        return Err(format!(
            "content hash mismatch: manifest has {}, files hash to {hash}",
            manifest.content_hash
        ));
    }
    Ok(Pack {
        root: root.to_path_buf(),
        manifest,
        phrases: Phrases {
            air_title: String::new(),
            chunks: vec![],
            staircase_notes: vec![],
            on_the_breath: BreathPhrase {
                notes: vec![],
                abc: String::new(),
                reference: String::new(),
            },
            meter: default_meter(),
            key: default_key(),
        },
        fingering: Fingering {
            holes_top_to_bottom: vec![
                "L1".into(),
                "L2".into(),
                "L3".into(),
                "R1".into(),
                "R2".into(),
                "R3".into(),
            ],
            notes: BTreeMap::new(),
            intervals_from_break: BTreeMap::new(),
        },
        ornaments: Ornaments {
            gestures: BTreeMap::new(),
            demo_note: String::new(),
            marks: vec![],
        },
        remarks: BTreeMap::new(),
        tune_abc: String::new(),
        words: Words::default(),
        playable: false,
        origin: "pack".into(),
    })
}

pub fn content_hash(manifest: &PackManifest, root: &Path) -> Result<String, String> {
    let mut hasher = Sha256::new();
    hasher.update(manifest.id.as_bytes());
    hasher.update([0]);
    hasher.update(manifest.version.as_bytes());
    hasher.update([0]);
    hasher.update(manifest.track.as_bytes());
    hasher.update([0]);
    hasher.update(manifest.title.as_bytes());
    hasher.update([0]);
    for id in &manifest.node_ids {
        hasher.update(id.as_bytes());
        hasher.update([0]);
    }
    for (id, page) in &manifest.pages {
        hasher.update(id.as_bytes());
        hasher.update([0]);
        hasher.update(page.title.as_bytes());
        hasher.update([0]);
        hasher.update(page.url.as_bytes());
        hasher.update([0]);
        hasher.update(page.key.as_bytes());
        hasher.update([0]);
    }
    hasher.update([manifest.wave]);
    hasher.update(manifest.after.as_bytes());
    hasher.update([0]);
    hasher.update(manifest.book_ref.as_bytes());
    hasher.update([0]);
    hasher.update(manifest.shelf.as_bytes());
    hasher.update([0]);
    hasher.update(manifest.rights.as_bytes());
    hasher.update([0]);
    hasher.update(manifest.source.as_bytes());
    hasher.update([0]);
    hasher.update(manifest.session.as_bytes());
    hasher.update([0]);
    hasher.update(manifest.pulse.as_bytes());
    hasher.update([0]);
    hasher.update([manifest.pulse_beats]);
    hasher.update(manifest.cnat_id.as_bytes());
    hasher.update([0]);
    for aka in &manifest.aka {
        hasher.update(aka.as_bytes());
        hasher.update([0]);
    }
    hash_present(&mut hasher, "licence", &manifest.licence);
    hash_present(&mut hasher, "author", &manifest.author);
    hash_present(&mut hasher, "site", &manifest.site);
    hash_present(&mut hasher, "placement", &manifest.placement);
    for node in &manifest.nodes {
        hasher.update(node.id.as_bytes());
        hasher.update([0]);
        hasher.update(node.mode.as_bytes());
        hasher.update([0]);
        hasher.update(node.title.as_bytes());
        hasher.update([0]);
        hasher.update(node.body.as_bytes());
        hasher.update([0]);
        hasher.update(node.high_d_body.as_bytes());
        hasher.update([0]);
        hasher.update(node.gesture.as_bytes());
        hasher.update([0]);
        hasher.update(node.note.as_bytes());
        hasher.update([0]);
        hasher.update([u8::from(node.hide_pictures)]);
        hasher.update([u8::from(node.grade_marks)]);
    }
    if manifest.rights == "page_only" {
        return Ok(hex(&hasher.finalize()));
    }
    for name in HASH_FILES {
        let bytes = fs::read(root.join(name)).map_err(|e| format!("{name}: {e}"))?;
        hasher.update(name.as_bytes());
        hasher.update([0]);
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(&bytes);
    }
    for name in OPTIONAL_HASH_FILES {
        let path = root.join(name);
        if !path.is_file() {
            continue;
        }
        let bytes = fs::read(&path).map_err(|e| format!("{name}: {e}"))?;
        hasher.update(name.as_bytes());
        hasher.update([0]);
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(&bytes);
    }
    Ok(hex(&hasher.finalize()))
}

fn validate(
    manifest: &PackManifest,
    phrases: &Phrases,
    fingering: &Fingering,
    ornaments: &Ornaments,
) -> Result<(), String> {
    if manifest.node_ids.is_empty() {
        return Err("pack has no nodes".into());
    }
    if manifest.node_ids.iter().any(|id| id == "hedwig") {
        let page = manifest
            .pages
            .get("hedwig")
            .ok_or_else(|| "hedwig needs a letter-note page".to_string())?;
        if page.key != "D" {
            return Err("hedwig letter notes are in D".into());
        }
        if !page.url.starts_with("https://") {
            return Err("hedwig page must be https".into());
        }
    }
    if !manifest.nodes.is_empty() {
        let ids: Vec<&str> = manifest.nodes.iter().map(|n| n.id.as_str()).collect();
        let listed: Vec<&str> = manifest.node_ids.iter().map(String::as_str).collect();
        if ids != listed {
            return Err("node list does not match nodes".into());
        }
        for node in &manifest.nodes {
            if !KNOWN_MODES.contains(&node.mode.as_str()) {
                return Err(format!("unknown mode {}", node.mode));
            }
        }
    } else {
        for id in &manifest.node_ids {
            if !KNOWN_NODES.contains(&id.as_str()) {
                return Err(format!("unknown node {id}"));
            }
        }
    }
    if fingering.holes_top_to_bottom != ["L1", "L2", "L3", "R1", "R2", "R3"] {
        return Err("holes must run L1..R3 from the window to the bell".into());
    }

    let mut notes = Vec::new();
    notes.extend(phrases.staircase_notes.iter().cloned());
    notes.extend(phrases.on_the_breath.notes.iter().cloned());
    for chunk in &phrases.chunks {
        notes.extend(chunk.notes.iter().cloned());
    }
    notes.push(ornaments.demo_note.clone());

    for note in notes {
        if note.is_empty() {
            continue;
        }
        let Some(named) = NoteName::from_str(&note) else {
            return Err(format!("{note} is not on this whistle"));
        };
        let chart = fingering
            .notes
            .get(&note)
            .ok_or_else(|| format!("no fingering for {note}"))?;
        check_chart(&note, chart)?;
        let interval = fingering
            .intervals_from_break
            .get(&note)
            .ok_or_else(|| format!("no interval for {note}"))?;
        if *interval != named.semitones_from_d4() {
            return Err(format!(
                "{note} interval {interval} does not match the scale"
            ));
        }
        let want = fingering::holes_for(named);
        let want_octave: u8 = if named.semitones_from_d4() >= 12 {
            2
        } else {
            1
        };
        if chart.octave != want_octave {
            return Err(format!(
                "{note} octave is {}, scale says {want_octave}",
                chart.octave
            ));
        }
        for (i, hole) in want.iter().enumerate() {
            if chart.holes[i] != hole.as_str() {
                return Err(format!(
                    "{note} hole {} is {}, chart says {}",
                    i + 1,
                    hole.as_str(),
                    chart.holes[i]
                ));
            }
        }
    }
    if !fits_low_d(&phrases.key) {
        return Err(format!("{} does not fit a low D", phrases.key));
    }
    Ok(())
}

/// Modes this tube can play with the notes in the scale. F natural and other keys stay out.
fn fits_low_d(key: &str) -> bool {
    matches!(
        key,
        "D" | "Dmaj" | "Ador" | "Ddor" | "Dmix" | "Edor" | "Em" | "Emin"
    )
}

fn check_chart(note: &str, chart: &HoleChart) -> Result<(), String> {
    if chart.holes.len() != 6 {
        return Err(format!("{note} needs six holes"));
    }
    for hole in &chart.holes {
        if hole != Hole::Closed.as_str()
            && hole != Hole::Open.as_str()
            && hole != Hole::Half.as_str()
        {
            return Err(format!("{note} has hole state {hole}"));
        }
    }
    Ok(())
}

fn hash_present(hasher: &mut Sha256, label: &str, value: &str) {
    if value.is_empty() {
        return;
    }
    hasher.update(label.as_bytes());
    hasher.update([0]);
    hasher.update(value.as_bytes());
    hasher.update([0]);
}

fn rights_kind(raw: &str) -> &'static str {
    match raw.trim().to_ascii_lowercase().as_str() {
        "pd" | "public_domain" | "public-domain" | "public domain" => "pd",
        "licence" | "license" => "licence",
        "brought" | "unknown" | "unsettled" => "brought",
        "page_only" | "page-only" => "page_only",
        "" => "empty",
        _ => "other",
    }
}

fn check_rights(manifest: &PackManifest, teacher: bool) -> Result<(), String> {
    match rights_kind(&manifest.rights) {
        "pd" => {
            let named = !manifest.source.trim().is_empty() || !manifest.site.trim().is_empty();
            if !named && (teacher || manifest.wave >= 2) {
                return Err(format!("{} needs a named source", manifest.id));
            }
            Ok(())
        }
        "licence" => {
            if manifest.licence.trim().is_empty() {
                return Err(format!("{} needs a stated licence", manifest.id));
            }
            Ok(())
        }
        "brought" => {
            if !teacher {
                return Err(format!(
                    "{} cannot ship with unsettled copyright",
                    manifest.id
                ));
            }
            Ok(())
        }
        "empty" if teacher => Ok(()),
        "page_only" => Ok(()),
        _ => Err(format!("{} needs a known rights mark", manifest.id)),
    }
}

fn hex(bytes: impl AsRef<[u8]>) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let bytes = bytes.as_ref();
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0xf) as usize] as char);
    }
    s
}

/// A pack reference is `ref/<name>.wav` only. No absolute paths, no `..`.
pub fn ref_file(pack: &Pack, relative: &str) -> Option<PathBuf> {
    let path = Path::new(relative);
    if path.is_absolute() || relative.contains('\0') || relative.contains("..") {
        return None;
    }
    let mut comps = path.components();
    let (Some(std::path::Component::Normal(dir)), Some(std::path::Component::Normal(file)), None) =
        (comps.next(), comps.next(), comps.next())
    else {
        return None;
    };
    let file = file.to_string_lossy();
    if dir != "ref" || !file.ends_with(".wav") || file.contains('/') || file.contains('\\') {
        return None;
    }
    let full = pack.root.join(relative);
    full.is_file().then_some(full)
}

pub fn ref_exists(pack: &Pack, relative: &str) -> bool {
    ref_file(pack, relative).is_some()
}

#[derive(Debug, Clone, Serialize)]
pub struct PackRefusal {
    pub folder: String,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct Catalog {
    pub packs: Vec<Pack>,
    pub refused: Vec<PackRefusal>,
}

/// One sentence for the desk. The loader's detail stays in the log of the reason string.
pub fn refusal_sentence(err: &str) -> String {
    if err.contains("page only and has a tune") || err.contains("page only and has audio") {
        return "This folder is on the page only, and it has notes or audio.".into();
    }
    if err.contains("needs a named source") {
        return "This folder has no named source.".into();
    }
    if err.contains("does not fit a low D") || err.contains("not on this whistle") {
        return "This setting does not fit a low D.".into();
    }
    if err.contains("needs a known rights mark") {
        return "This folder does not say what its copyright is.".into();
    }
    if err.contains("cannot ship with unsettled copyright") {
        return "This folder's copyright is not something the app can ship.".into();
    }
    if err.contains("needs a stated licence") {
        return "This folder does not name the licence.".into();
    }
    if let Some(rest) = err.split("notes not on a low D: ").nth(1) {
        return format!("These notes are not on a low D: {rest}.");
    }
    if err.contains("wider than a low D") {
        return "This melody spans more than a low D can play.".into();
    }
    if err.contains("no melody") {
        return "This MIDI file has no melody.".into();
    }
    if err.contains("more than one midi") {
        return "This folder has more than one MIDI file.".into();
    }
    if err.contains("notice") {
        return "This folder's note could not be read.".into();
    }
    if err.contains("midi") {
        return "This MIDI file could not be read.".into();
    }
    "This folder could not be opened.".into()
}

#[derive(Debug, Clone)]
pub struct ProgressMark {
    pub profile_id: String,
    pub pack_id: String,
    pub node_id: String,
    pub state_reached: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PackSummary {
    pub id: String,
    pub title: String,
    pub wave: u8,
    pub after: String,
    pub book_ref: String,
    pub shelf: String,
    pub rights: String,
    pub aka: Vec<String>,
    pub source: String,
    pub licence: String,
    pub author: String,
    pub site: String,
    pub placement: String,
    pub session: String,
    pub pulse: String,
    pub pulse_beats: u8,
    pub playable: bool,
    pub open: bool,
    pub settled: bool,
    /// A page, or a phrase titled as the pack. Drills stay false.
    pub song: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CatalogView {
    pub shelf_open: bool,
    pub desk_open: bool,
    pub packs: Vec<PackSummary>,
    pub desk: Vec<PackSummary>,
    pub refused: Vec<PackRefusal>,
}

fn folder_name(dir: &Path) -> String {
    dir.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| dir.display().to_string())
}

fn has_midi(dir: &Path) -> bool {
    midi_files(dir).map(|files| !files.is_empty()).unwrap_or(false)
}

fn midi_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext.eq_ignore_ascii_case("mid") || ext.eq_ignore_ascii_case("midi") {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

fn slug(name: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            dash = false;
        } else if !dash && !out.is_empty() {
            out.push('-');
            dash = true;
        }
    }
    let out = out.trim_matches('-').to_string();
    if out.is_empty() {
        "brought-tune".into()
    } else {
        out
    }
}

#[derive(Debug, Deserialize, Default)]
struct Notice {
    #[serde(default)]
    title: String,
    #[serde(default)]
    rights: String,
    #[serde(default)]
    source: String,
    #[serde(default)]
    author: String,
    #[serde(default)]
    licence: String,
    #[serde(default)]
    site: String,
}

fn load_midi_folder(dir: &Path) -> Result<Pack, String> {
    let files = midi_files(dir)?;
    if files.len() != 1 {
        return Err("more than one midi".into());
    }
    let bytes = fs::read(&files[0]).map_err(|e| format!("midi: {e}"))?;
    let melody = crate::midi::melody_from_midi(&bytes)?;
    let notice = if dir.join("notice.json").is_file() {
        let text =
            fs::read_to_string(dir.join("notice.json")).map_err(|e| format!("notice: {e}"))?;
        serde_json::from_str(&text).map_err(|e| format!("notice: {e}"))?
    } else {
        Notice::default()
    };
    let folder = folder_name(dir);
    let title = if notice.title.trim().is_empty() {
        folder.replace('-', " ")
    } else {
        notice.title
    };
    let mut rights = notice.rights;
    if matches!(rights_kind(&rights), "brought" | "empty") {
        rights = "brought".into();
    }
    let manifest = PackManifest {
        id: slug(&folder),
        version: "0.1.0".into(),
        track: "teacher".into(),
        title: title.clone(),
        node_ids: vec!["line".into()],
        nodes: vec![PackNode {
            id: "line".into(),
            mode: "phrase".into(),
            title: title.clone(),
            body: melody.placement.clone(),
            high_d_body: String::new(),
            gesture: String::new(),
            note: String::new(),
            hide_pictures: false,
            grade_marks: false,
        }],
        pages: BTreeMap::new(),
        content_hash: hex(Sha256::digest(&bytes)),
        wave: 0,
        after: String::new(),
        book_ref: String::new(),
        shelf: String::new(),
        rights,
        aka: Vec::new(),
        source: notice.source,
        licence: notice.licence,
        author: notice.author,
        site: notice.site,
        placement: melody.placement,
        session: String::new(),
        pulse: String::new(),
        pulse_beats: 0,
        cnat_id: String::new(),
    };
    check_rights(&manifest, true)?;
    let mut chunks = Vec::new();
    for (i, chunk) in melody.chunks.iter().enumerate() {
        chunks.push(PhraseChunk {
            id: format!("line_{}", i + 1),
            label: format!("Phrase {}", i + 1),
            notes: chunk.notes.clone(),
            abc: chunk.abc.clone(),
            reference: String::new(),
            breaths: vec![chunk.notes.len().saturating_sub(1)],
        });
    }
    let phrases = Phrases {
        air_title: title,
        chunks,
        staircase_notes: Vec::new(),
        on_the_breath: BreathPhrase {
            notes: Vec::new(),
            abc: String::new(),
            reference: String::new(),
        },
        meter: "4/4".into(),
        key: "D".into(),
    };
    let fingering = scale_fingering();
    let demo = phrases.chunks[0].notes[0].clone();
    let ornaments = Ornaments {
        gestures: BTreeMap::new(),
        demo_note: demo,
        marks: Vec::new(),
    };
    validate(&manifest, &phrases, &fingering, &ornaments)?;
    Ok(Pack {
        root: dir.to_path_buf(),
        manifest,
        phrases,
        fingering,
        ornaments,
        remarks: BTreeMap::new(),
        tune_abc: String::new(),
        words: Words::default(),
        playable: true,
        origin: "teacher".into(),
    })
}

fn scale_fingering() -> Fingering {
    let mut notes = BTreeMap::new();
    let mut intervals = BTreeMap::new();
    for note in NoteName::ALL {
        let holes: Vec<String> = fingering::holes_for(note)
            .iter()
            .map(|hole| hole.as_str().to_string())
            .collect();
        let octave = if note.semitones_from_d4() >= 12 { 2 } else { 1 };
        notes.insert(
            note.as_str().to_string(),
            HoleChart {
                holes,
                octave,
                label: fingering_label(note),
                half: false,
            },
        );
        intervals.insert(note.as_str().to_string(), note.semitones_from_d4());
    }
    Fingering {
        holes_top_to_bottom: ["L1", "L2", "L3", "R1", "R2", "R3"]
            .into_iter()
            .map(str::to_string)
            .collect(),
        notes,
        intervals_from_break: intervals,
    }
}

fn fingering_label(note: NoteName) -> String {
    let base = match note {
        NoteName::D4 | NoteName::D5 => "D",
        NoteName::E4 | NoteName::E5 => "E",
        NoteName::Fs4 | NoteName::Fs5 => "F#",
        NoteName::G4 | NoteName::G5 => "G",
        NoteName::A4 | NoteName::A5 => "A",
        NoteName::B4 | NoteName::B5 => "B",
        NoteName::C5 => "C",
        NoteName::Cs5 => "C#",
    };
    if note.semitones_from_d4() >= 12 {
        format!("{base}′")
    } else {
        base.to_string()
    }
}

pub fn catalog_roots() -> Vec<PathBuf> {
    let door = resolve_pack_dir();
    let pack_root = door
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("pack"));
    let mut roots = vec![pack_root.clone()];
    if let Some(parent) = pack_root.parent() {
        let teacher = parent.join("teacher");
        if teacher.is_dir() {
            roots.push(teacher);
        }
    }
    roots
}

pub fn load_catalog(roots: &[PathBuf]) -> Result<Catalog, String> {
    let mut packs = Vec::new();
    let mut refused = Vec::new();
    for root in roots {
        if !root.is_dir() {
            continue;
        }
        let teacher = root.file_name().and_then(|n| n.to_str()) == Some("teacher");
        let mut dirs: Vec<PathBuf> = fs::read_dir(root)
            .map_err(|e| format!("{}: {e}", root.display()))?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir() && (p.join("manifest.json").is_file() || (teacher && has_midi(p))))
            .collect();
        dirs.sort();
        for dir in dirs {
            let loaded = if teacher && !dir.join("manifest.json").is_file() {
                load_midi_folder(&dir)
            } else {
                load_pack_kind(&dir, teacher)
            };
            match loaded {
                Ok(mut pack) => {
                    pack.origin = if teacher {
                        "teacher".into()
                    } else {
                        "pack".into()
                    };
                    if teacher && packs.iter().any(|p: &Pack| p.manifest.id == pack.manifest.id) {
                        refused.push(PackRefusal {
                            folder: folder_name(&dir),
                            reason: "This folder uses a name the app already has.".into(),
                        });
                        continue;
                    }
                    packs.push(pack);
                }
                Err(reason) if teacher => {
                    refused.push(PackRefusal {
                        folder: folder_name(&dir),
                        reason: refusal_sentence(&reason),
                    });
                }
                Err(reason) => return Err(reason),
            }
        }
    }
    if !packs.iter().any(|p| p.playable) {
        return Err("no packs".into());
    }
    Ok(Catalog { packs, refused })
}

fn settled(marks: &[ProgressMark], profile_id: &str, pack_id: &str, node_id: &str) -> bool {
    marks.iter().any(|m| {
        m.profile_id == profile_id
            && m.pack_id == pack_id
            && m.node_id == node_id
            && m.state_reached == "settled"
    })
}

fn pack_is_settled(pack: &Pack, marks: &[ProgressMark], profile_id: &str) -> bool {
    pack.playable
        && !pack.manifest.node_ids.is_empty()
        && pack
            .manifest
            .node_ids
            .iter()
            .all(|id| settled(marks, profile_id, &pack.manifest.id, id))
}

fn dependency_settled(
    after: &str,
    packs: &[Pack],
    marks: &[ProgressMark],
    profile_id: &str,
) -> bool {
    if after.is_empty() {
        return true;
    }
    if let Some((pack_id, node_id)) = after.split_once('/') {
        return settled(marks, profile_id, pack_id, node_id);
    }
    packs
        .iter()
        .find(|p| p.manifest.id == after)
        .map(|p| pack_is_settled(p, marks, profile_id))
        .unwrap_or(false)
}

/// A page, or a phrase that is the pack’s own tune. A drill phrase stays a lesson.
pub fn is_song_pack(pack: &Pack) -> bool {
    pack.manifest.nodes.iter().any(|node| {
        node.mode == "page"
            || (node.mode == "phrase"
                && (node.id.starts_with("air_") || node.title == pack.manifest.title))
    })
}

/// Keep requested ids that are allowed, in the order given, without duplicates.
pub fn retain_lesson_packs(requested: &[String], allowed: &[String]) -> Vec<String> {
    let mut kept = Vec::new();
    for id in requested {
        if allowed.iter().any(|ok| ok == id) && !kept.iter().any(|have| have == id) {
            kept.push(id.clone());
        }
    }
    kept
}

/// Playable public-domain songs that are already open. Teacher folders stay out.
pub fn open_song_ids(catalog: &Catalog, marks: &[ProgressMark], profile_id: &str) -> Vec<String> {
    catalog_view(catalog, marks, profile_id)
        .packs
        .into_iter()
        .filter(|pack| {
            pack.song
                && pack.playable
                && pack.open
                && pack.rights == "pd"
                && !pack.source.trim().is_empty()
        })
        .map(|pack| pack.id)
        .collect()
}

fn summary_for(pack: &Pack, open: bool, settled: bool) -> PackSummary {
    PackSummary {
        id: pack.manifest.id.clone(),
        title: pack.manifest.title.clone(),
        wave: pack.manifest.wave,
        after: pack.manifest.after.clone(),
        book_ref: pack.manifest.book_ref.clone(),
        shelf: pack.manifest.shelf.clone(),
        rights: pack.manifest.rights.clone(),
        aka: pack.manifest.aka.clone(),
        source: pack.manifest.source.clone(),
        licence: pack.manifest.licence.clone(),
        author: pack.manifest.author.clone(),
        site: pack.manifest.site.clone(),
        placement: pack.manifest.placement.clone(),
        session: pack.manifest.session.clone(),
        pulse: pack.manifest.pulse.clone(),
        pulse_beats: pack.manifest.pulse_beats,
        playable: pack.playable,
        open,
        settled,
        song: is_song_pack(pack),
    }
}

pub fn catalog_view(catalog: &Catalog, marks: &[ProgressMark], profile_id: &str) -> CatalogView {
    let door_settled = catalog
        .packs
        .iter()
        .any(|p| p.manifest.id == "may-morning-dew" && pack_is_settled(p, marks, profile_id));
    let mut playable: Vec<&Pack> = catalog
        .packs
        .iter()
        .filter(|p| p.playable && p.origin != "teacher")
        .collect();
    let mut ordered: Vec<&Pack> = Vec::new();
    while !playable.is_empty() {
        let idx = playable.iter().position(|p| {
            if p.manifest.after.is_empty() {
                return ordered.is_empty();
            }
            let dep = p.manifest.after.split('/').next().unwrap_or("");
            ordered.iter().any(|o| o.manifest.id == dep)
        });
        let Some(idx) = idx else {
            ordered.extend(playable.drain(..));
            break;
        };
        ordered.push(playable.remove(idx));
    }
    let mut page_only: Vec<&Pack> = catalog
        .packs
        .iter()
        .filter(|p| !p.playable && p.origin != "teacher")
        .collect();
    page_only.sort_by(|a, b| a.manifest.title.cmp(&b.manifest.title));

    let shelf_open = catalog
        .packs
        .iter()
        .any(|p| p.manifest.id == "salley-gardens" && pack_is_settled(p, marks, profile_id));

    let mut summaries = Vec::new();
    for pack in ordered.into_iter().chain(page_only) {
        let open = pack.playable
            && dependency_settled(&pack.manifest.after, &catalog.packs, marks, profile_id);
        summaries.push(summary_for(
            pack,
            open,
            pack_is_settled(pack, marks, profile_id),
        ));
    }
    let desk = catalog
        .packs
        .iter()
        .filter(|p| p.origin == "teacher")
        .map(|pack| {
            summary_for(
                pack,
                door_settled && pack.playable,
                pack_is_settled(pack, marks, profile_id),
            )
        })
        .collect();
    CatalogView {
        shelf_open,
        desk_open: door_settled,
        packs: summaries,
        desk,
        refused: catalog.refused.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pack() -> Pack {
        load_pack(&resolve_pack_dir()).expect("may-morning-dew pack")
    }

    #[test]
    fn may_morning_dew_loads_and_hash_matches() {
        let pack = pack();
        assert_eq!(pack.manifest.id, "may-morning-dew");
        assert_eq!(pack.manifest.node_ids.len(), 10);
        assert_eq!(
            pack.manifest.node_ids.get(3).map(String::as_str),
            Some("hedwig")
        );
        assert_eq!(pack.manifest.pages["hedwig"].key, "D");
        let again = content_hash(&pack.manifest, &pack.root).unwrap();
        assert_eq!(again, pack.manifest.content_hash);
    }

    #[test]
    fn unknown_node_and_a_bad_hash_are_refused() {
        let src = resolve_pack_dir();
        let dest = std::env::temp_dir().join(format!("low-d-pack-bad-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dest);
        fs::create_dir_all(&dest).unwrap();
        for name in HASH_FILES {
            fs::copy(src.join(name), dest.join(name)).unwrap();
        }
        let raw = fs::read_to_string(src.join("manifest.json")).unwrap();
        let mut manifest: PackManifest = serde_json::from_str(&raw).unwrap();
        manifest.content_hash = "0".repeat(64);
        fs::write(
            dest.join("manifest.json"),
            serde_json::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
        let mismatch = load_pack(&dest).unwrap_err();
        assert!(mismatch.contains("content hash mismatch"), "{mismatch}");

        manifest.node_ids.push("not_a_node".into());
        manifest.content_hash = content_hash(&manifest, &dest).unwrap();
        fs::write(
            dest.join("manifest.json"),
            serde_json::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
        let unknown = load_pack(&dest).unwrap_err();
        assert!(
            unknown.contains("unknown node") || unknown.contains("node list"),
            "{unknown}"
        );
        let _ = fs::remove_dir_all(&dest);
    }

    #[test]
    fn ref_paths_stay_inside_the_pack() {
        let pack = pack();
        assert!(ref_file(&pack, "../tune.abc").is_none());
        assert!(ref_file(&pack, "/etc/passwd").is_none());
        assert!(ref_file(&pack, "ref/../manifest.json").is_none());
        assert!(ref_file(&pack, "other/phrase_1.wav").is_none());
        assert!(ref_file(&pack, "ref/phrase_1.txt").is_none());
        assert!(ref_file(&pack, "ref/no-such.wav").is_none());
    }

    #[test]
    fn write_content_hashes() {
        if std::env::var_os("LOWD_WRITE_HASH").is_none()
            && std::env::var_os("LOWD_PRINT_HASH").is_none()
        {
            return;
        }
        let door = resolve_pack_dir();
        let root = door.parent().unwrap();
        let print_only = std::env::var_os("LOWD_WRITE_HASH").is_none();
        for entry in fs::read_dir(root).unwrap().flatten() {
            let dir = entry.path();
            if !dir.join("manifest.json").is_file() {
                continue;
            }
            let raw = fs::read_to_string(dir.join("manifest.json")).unwrap();
            let mut manifest: PackManifest = serde_json::from_str(&raw).unwrap();
            manifest.content_hash = content_hash(&manifest, &dir).unwrap();
            if print_only {
                println!("{} {}", manifest.id, manifest.content_hash);
                continue;
            }
            fs::write(
                dir.join("manifest.json"),
                serde_json::to_string_pretty(&manifest).unwrap() + "\n",
            )
            .unwrap();
            println!("{} {}", manifest.id, manifest.content_hash);
        }
    }

    #[test]
    fn the_door_is_open_and_the_shelf_is_not() {
        let cat = load_catalog(&catalog_roots()).expect("catalog");
        let view = catalog_view(&cat, &[], "player");
        assert_eq!(view.packs[0].id, "may-morning-dew");
        assert!(view.packs[0].open);
        assert!(!view.shelf_open);
        let stair = view
            .packs
            .iter()
            .find(|p| p.id == "book-staircase")
            .expect("book staircase");
        assert!(!stair.open);
        assert!(view
            .packs
            .iter()
            .any(|p| p.id == "lonesome-boatman" && !p.playable));
        assert!(view
            .packs
            .iter()
            .any(|p| p.rights == "page_only" && !p.open));
    }

    #[test]
    fn a_lesson_set_keeps_open_songs_only() {
        let cat = load_catalog(&catalog_roots()).expect("catalog");
        let view = catalog_view(&cat, &[], "player");
        let song = |id: &str| view.packs.iter().find(|p| p.id == id).unwrap().song;
        assert!(song("may-morning-dew"));
        assert!(song("salley-gardens"));
        assert!(!song("book-staircase"));
        assert!(!song("c-natural"));
        assert!(!song("lonesome-boatman"));
        let allowed = open_song_ids(&cat, &[], "player");
        assert!(allowed.iter().any(|id| id == "may-morning-dew"));
        assert!(!allowed.iter().any(|id| id == "salley-gardens"));
        assert!(!allowed.iter().any(|id| id == "lonesome-boatman"));
        assert!(!allowed.iter().any(|id| id == "book-staircase"));
        let kept = retain_lesson_packs(
            &[
                "salley-gardens".into(),
                "lonesome-boatman".into(),
                "book-staircase".into(),
                "may-morning-dew".into(),
                "may-morning-dew".into(),
            ],
            &allowed,
        );
        assert_eq!(kept, vec!["may-morning-dew".to_string()]);
    }

    #[test]
    fn salley_joins_the_lesson_set_when_the_staircase_is_settled() {
        let cat = load_catalog(&catalog_roots()).expect("catalog");
        let closed = catalog_view(&cat, &[], "player");
        assert!(closed
            .packs
            .iter()
            .find(|p| p.id == "drops-of-brandy")
            .unwrap()
            .song);
        assert!(!closed
            .packs
            .iter()
            .find(|p| p.id == "more-ornament")
            .unwrap()
            .song);
        let mut marks = Vec::new();
        for id in ["may-morning-dew", "book-staircase"] {
            let pack = cat.packs.iter().find(|p| p.manifest.id == id).unwrap();
            for node in &pack.manifest.node_ids {
                marks.push(ProgressMark {
                    profile_id: "player".into(),
                    pack_id: id.into(),
                    node_id: node.clone(),
                    state_reached: "settled".into(),
                });
            }
        }
        let allowed = open_song_ids(&cat, &marks, "player");
        assert!(allowed.iter().any(|id| id == "may-morning-dew"));
        assert!(allowed.iter().any(|id| id == "salley-gardens"));
        assert!(!allowed.iter().any(|id| id == "amazing-grace"));
        assert!(!allowed.iter().any(|id| id == "c-natural"));
        let kept = retain_lesson_packs(
            &[
                "teacher-tune".into(),
                "salley-gardens".into(),
                "amazing-grace".into(),
                "lonesome-boatman".into(),
            ],
            &allowed,
        );
        assert_eq!(kept, vec!["salley-gardens".to_string()]);
    }

    fn stage_door(name: &str) -> PathBuf {
        let src = resolve_pack_dir();
        let dest = std::env::temp_dir().join(format!("low-d-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dest);
        fs::create_dir_all(&dest).unwrap();
        for name in HASH_FILES {
            fs::copy(src.join(name), dest.join(name)).unwrap();
        }
        fs::copy(src.join("manifest.json"), dest.join("manifest.json")).unwrap();
        dest
    }

    fn rewrite_hash(dir: &Path) {
        let raw = fs::read_to_string(dir.join("manifest.json")).unwrap();
        let mut manifest: PackManifest = serde_json::from_str(&raw).unwrap();
        manifest.content_hash = content_hash(&manifest, dir).unwrap();
        fs::write(
            dir.join("manifest.json"),
            serde_json::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn a_foreign_key_and_an_unknown_note_are_refused() {
        let dir = stage_door("key");
        let mut phrases: Phrases =
            serde_json::from_str(&fs::read_to_string(dir.join("phrases.json")).unwrap()).unwrap();
        phrases.key = "F".into();
        fs::write(
            dir.join("phrases.json"),
            serde_json::to_string_pretty(&phrases).unwrap(),
        )
        .unwrap();
        rewrite_hash(&dir);
        let err = load_pack(&dir).unwrap_err();
        assert!(err.contains("does not fit a low D"), "{err}");

        let dir = stage_door("note");
        let mut phrases: Phrases =
            serde_json::from_str(&fs::read_to_string(dir.join("phrases.json")).unwrap()).unwrap();
        phrases.staircase_notes.push("Bb4".into());
        fs::write(
            dir.join("phrases.json"),
            serde_json::to_string_pretty(&phrases).unwrap(),
        )
        .unwrap();
        let mut fingering: Fingering =
            serde_json::from_str(&fs::read_to_string(dir.join("fingering-low-d.json")).unwrap())
                .unwrap();
        fingering.notes.insert(
            "Bb4".into(),
            HoleChart {
                holes: vec!["closed".into(); 6],
                octave: 1,
                label: "Bb".into(),
                half: false,
            },
        );
        fingering.intervals_from_break.insert("Bb4".into(), 8);
        fs::write(
            dir.join("fingering-low-d.json"),
            serde_json::to_string_pretty(&fingering).unwrap(),
        )
        .unwrap();
        rewrite_hash(&dir);
        let err = load_pack(&dir).unwrap_err();
        assert!(err.contains("not on this whistle"), "{err}");
        let _ = fs::remove_dir_all(stage_door("key"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn rights_and_a_page_only_tune_are_refused() {
        let dir = stage_door("rights");
        let raw = fs::read_to_string(dir.join("manifest.json")).unwrap();
        let mut manifest: PackManifest = serde_json::from_str(&raw).unwrap();
        manifest.rights = "borrowed".into();
        fs::write(
            dir.join("manifest.json"),
            serde_json::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
        rewrite_hash(&dir);
        let err = load_pack(&dir).unwrap_err();
        assert!(err.contains("needs a known rights mark"), "{err}");

        let raw = fs::read_to_string(dir.join("manifest.json")).unwrap();
        let mut manifest: PackManifest = serde_json::from_str(&raw).unwrap();
        manifest.rights = "brought".into();
        fs::write(
            dir.join("manifest.json"),
            serde_json::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
        rewrite_hash(&dir);
        let err = load_pack(&dir).unwrap_err();
        assert!(err.contains("cannot ship with unsettled copyright"), "{err}");

        let page = std::env::temp_dir().join(format!("low-d-page-{}", std::process::id()));
        let _ = fs::remove_dir_all(&page);
        fs::create_dir_all(&page).unwrap();
        let mut stub = manifest;
        stub.id = "not-a-tune".into();
        stub.rights = "page_only".into();
        stub.node_ids.clear();
        stub.nodes.clear();
        stub.source = "In the book. No notes.".into();
        fs::write(page.join("tune.abc"), "X:1\nK:D\nD2\n").unwrap();
        stub.content_hash = content_hash(&stub, &page).unwrap();
        fs::write(
            page.join("manifest.json"),
            serde_json::to_string_pretty(&stub).unwrap(),
        )
        .unwrap();
        let err = load_pack(&page).unwrap_err();
        assert!(err.contains("page only and has a tune"), "{err}");
        let _ = fs::remove_dir_all(&dir);
        let _ = fs::remove_dir_all(&page);
    }

    #[test]
    fn the_staircase_opens_after_the_door_and_the_shelf_after_salley() {
        let pack_root = resolve_pack_dir()
            .parent()
            .expect("pack root")
            .to_path_buf();
        let cat = load_catalog(&[pack_root]).expect("catalog");
        let door = cat
            .packs
            .iter()
            .find(|p| p.manifest.id == "may-morning-dew")
            .unwrap();
        let salley = cat
            .packs
            .iter()
            .find(|p| p.manifest.id == "salley-gardens")
            .unwrap();
        let mut marks = Vec::new();
        for id in &door.manifest.node_ids {
            marks.push(ProgressMark {
                profile_id: "player".into(),
                pack_id: door.manifest.id.clone(),
                node_id: id.clone(),
                state_reached: "settled".into(),
            });
        }
        let after_door = catalog_view(&cat, &marks, "player");
        let stair = after_door
            .packs
            .iter()
            .find(|p| p.id == "book-staircase")
            .unwrap();
        assert!(stair.open);
        assert!(!after_door.shelf_open);

        let stair_pack = cat
            .packs
            .iter()
            .find(|p| p.manifest.id == "book-staircase")
            .unwrap();
        for id in &stair_pack.manifest.node_ids {
            marks.push(ProgressMark {
                profile_id: "player".into(),
                pack_id: stair_pack.manifest.id.clone(),
                node_id: id.clone(),
                state_reached: "settled".into(),
            });
        }
        for id in &salley.manifest.node_ids {
            marks.push(ProgressMark {
                profile_id: "player".into(),
                pack_id: salley.manifest.id.clone(),
                node_id: id.clone(),
                state_reached: "settled".into(),
            });
        }
        let open = catalog_view(&cat, &marks, "player");
        assert!(open.shelf_open);
        let cnat = open.packs.iter().find(|p| p.id == "c-natural").unwrap();
        assert!(cnat.open);
        let grace = open.packs.iter().find(|p| p.id == "amazing-grace").unwrap();
        assert!(!grace.open);
        assert!(open.desk_open);
        assert!(open.desk.is_empty());
    }

    #[test]
    fn a_bad_teacher_folder_does_not_drop_the_door() {
        assert_eq!(
            refusal_sentence("tune does not fit a low D"),
            "This setting does not fit a low D."
        );
        assert_eq!(
            refusal_sentence("air needs a named source"),
            "This folder has no named source."
        );
        assert_eq!(
            refusal_sentence("x is page only and has a tune"),
            "This folder is on the page only, and it has notes or audio."
        );

        let pack_root = resolve_pack_dir()
            .parent()
            .expect("pack root")
            .to_path_buf();
        let base = std::env::temp_dir().join(format!("low-d-desk-{}", std::process::id()));
        let teacher = base.join("teacher");
        let _ = fs::remove_dir_all(&base);
        let bad = teacher.join("not-a-whistle");
        fs::create_dir_all(&bad).unwrap();
        fs::write(bad.join("manifest.json"), "{\"id\":\"nope\"}").unwrap();
        let cat = load_catalog(&[pack_root, teacher]).expect("door still loads");
        assert!(cat.packs.iter().any(|p| p.manifest.id == "may-morning-dew"));
        assert!(cat.refused.iter().any(|r| {
            r.folder == "not-a-whistle" && r.reason == "This folder could not be opened."
        }));
        let shut = catalog_view(&cat, &[], "player");
        assert!(!shut.desk_open);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn a_piano_midi_on_the_desk_is_shifted_and_marked_brought() {
        let pack_root = resolve_pack_dir()
            .parent()
            .expect("pack root")
            .to_path_buf();
        let base = std::env::temp_dir().join(format!("low-d-midi-{}", std::process::id()));
        let teacher = base.join("teacher");
        let tune = teacher.join("high-line");
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&tune).unwrap();
        fs::write(
            tune.join("line.mid"),
            crate::midi::quarter_note_file(&[86, 88, 90]),
        )
        .unwrap();
        fs::write(
            tune.join("notice.json"),
            r#"{"title":"High line","author":"A player","site":"https://www.traditionalmusic.co.uk/song-midis/songs-midis.html"}"#,
        )
        .unwrap();
        let cat = load_catalog(&[pack_root.clone(), teacher.clone()]).expect("catalog");
        let pack = cat
            .packs
            .iter()
            .find(|p| p.manifest.id == "high-line")
            .expect("brought midi");
        assert_eq!(pack.manifest.rights, "brought");
        assert_eq!(pack.manifest.author, "A player");
        assert_eq!(
            pack.phrases.chunks[0].notes,
            vec!["D5".to_string(), "E5".to_string(), "F#5".to_string()]
        );
        assert!(
            pack.manifest.placement.contains("down one octave"),
            "{}",
            pack.manifest.placement
        );
        assert!(cat.packs.iter().any(|p| p.manifest.id == "may-morning-dew"));

        let off = teacher.join("too-wide");
        fs::create_dir_all(&off).unwrap();
        fs::write(
            off.join("wide.mid"),
            crate::midi::quarter_note_file(&[60, 84]),
        )
        .unwrap();
        let cat = load_catalog(&[pack_root, teacher]).expect("catalog");
        assert!(cat.refused.iter().any(|r| {
            r.folder == "too-wide" && r.reason == "This melody spans more than a low D can play."
        }));
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn local_teacher_folders_load() {
        let cat = load_catalog(&catalog_roots()).expect("catalog");
        assert!(cat.packs.iter().any(|p| p.manifest.id == "may-morning-dew"));
        for pack in cat.packs.iter().filter(|p| p.origin == "teacher") {
            assert!(pack.playable, "{}", pack.manifest.id);
            assert!(!pack.phrases.chunks.is_empty(), "{}", pack.manifest.id);
            if std::env::var_os("LOWD_DESK").is_some() {
                let notes: usize = pack.phrases.chunks.iter().map(|c| c.notes.len()).sum();
                eprintln!(
                    "OPEN\t{}\t{notes}\t{}",
                    pack.manifest.title, pack.manifest.placement
                );
            }
        }
        if std::env::var_os("LOWD_DESK").is_some() {
            for item in &cat.refused {
                eprintln!("SHUT\t{}\t{}", item.folder, item.reason);
            }
        }
    }
}
