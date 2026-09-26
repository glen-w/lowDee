use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::listen::types::WhistleProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEntry {
    pub node_id: String,
    /// Empty on records written before profiles owned their own progress.
    #[serde(default)]
    pub profile_id: String,
    /// Empty on records written before packs owned their own progress.
    #[serde(default)]
    pub pack_id: String,
    pub state_reached: String, // started | settled
    pub as_of: String,
    /// `heard` when the loop settled it, `stepped` when the player moved on unheard.
    #[serde(default)]
    pub via: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppStore {
    pub profiles: Vec<WhistleProfile>,
    pub active_profile_id: Option<String>,
    pub progress: Vec<ProgressEntry>,
}

pub fn store_path(app_dir: &Path) -> PathBuf {
    app_dir.join("low-d-store.json")
}

pub fn load(app_dir: &Path) -> Result<AppStore, String> {
    let path = store_path(app_dir);
    let text = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(AppStore::default()),
        Err(e) => return Err(format!("practice record: {e}")),
    };
    if text.trim().is_empty() {
        return Ok(AppStore::default());
    }
    let mut store: AppStore = match serde_json::from_str(&text) {
        Ok(store) => store,
        Err(_) => {
            let stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let bak = app_dir.join(format!("low-d-store.json.bak-{stamp}"));
            fs::rename(&path, &bak).map_err(|e| {
                format!("practice record could not be read, and could not be set aside: {e}")
            })?;
            return Err(format!(
                "The practice record could not be read. It was set aside as {}.",
                bak.display()
            ));
        }
    };
    if migrate(&mut store) {
        save(app_dir, &store)?;
    }
    Ok(store)
}

/// Attach a flat progress list to the active profile. Returns whether anything changed.
pub fn migrate(store: &mut AppStore) -> bool {
    let pid = store.active_profile_id.clone().unwrap_or_default();
    let mut changed = false;
    for entry in &mut store.progress {
        if entry.profile_id.is_empty() && !pid.is_empty() {
            entry.profile_id = pid.clone();
            changed = true;
        }
        if entry.via.is_empty() && entry.state_reached == "settled" {
            entry.via = "heard".into();
            changed = true;
        }
        if entry.pack_id.is_empty() {
            entry.pack_id = "may-morning-dew".into();
            changed = true;
        }
    }
    changed
}

pub fn save(app_dir: &Path, store: &AppStore) -> Result<(), String> {
    fs::create_dir_all(app_dir).map_err(|e| e.to_string())?;
    let path = store_path(app_dir);
    let tmp = app_dir.join("low-d-store.json.tmp");
    let body = serde_json::to_string_pretty(store).map_err(|e| e.to_string())?;
    fs::write(&tmp, body).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &path).map_err(|e| e.to_string())
}

pub fn upsert_profile(store: &mut AppStore, profile: WhistleProfile) {
    if let Some(existing) = store
        .profiles
        .iter_mut()
        .find(|p| p.profile_id == profile.profile_id)
    {
        *existing = profile.clone();
    } else {
        store.profiles.push(profile.clone());
    }
    store.active_profile_id = Some(profile.profile_id);
}

pub fn set_cnat(store: &mut AppStore, profile_id: &str, fingering: &str) {
    if let Some(profile) = store
        .profiles
        .iter_mut()
        .find(|p| p.profile_id == profile_id)
    {
        profile.cnat_fingering = Some(fingering.to_string());
    }
}

pub fn set_progress(
    store: &mut AppStore,
    profile_id: &str,
    pack_id: &str,
    node_id: &str,
    state_reached: &str,
    via: &str,
) {
    let as_of = chrono::Utc::now().to_rfc3339();
    if let Some(entry) = store
        .progress
        .iter_mut()
        .find(|p| p.profile_id == profile_id && p.pack_id == pack_id && p.node_id == node_id)
    {
        if entry.state_reached == "settled" && state_reached != "settled" {
            return;
        }
        if entry.state_reached == "settled" && entry.via == "heard" && via == "stepped" {
            return;
        }
        entry.state_reached = state_reached.to_string();
        entry.via = via.to_string();
        entry.as_of = as_of;
    } else {
        store.progress.push(ProgressEntry {
            node_id: node_id.to_string(),
            profile_id: profile_id.to_string(),
            pack_id: pack_id.to_string(),
            state_reached: state_reached.to_string(),
            as_of,
            via: via.to_string(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::listen::types::{AutoAdvance, Background, Reads, WhistleProfile};

    fn profile(id: &str) -> WhistleProfile {
        WhistleProfile {
            profile_id: id.into(),
            label: "horn".into(),
            break_hz: 290.0,
            rms_floor: 0.02,
            cal_as_of: "t".into(),
            reads: Reads::No,
            background: Background::None,
            cnat_fingering: None,
            skip_book_talk: true,
            warmup_on_launch: true,
            lesson_packs: Vec::new(),
            auto_advance: AutoAdvance::Inside,
        }
    }

    fn dir(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!("low-d-store-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn an_old_profile_skips_the_book_talk() {
        let raw = r#"{
            "profile_id": "a",
            "label": "horn",
            "break_hz": 290.0,
            "rms_floor": 0.02,
            "cal_as_of": "t",
            "reads": "no",
            "background": "none"
        }"#;
        let profile: WhistleProfile = serde_json::from_str(raw).unwrap();
        assert!(profile.skip_book_talk);
        assert!(profile.warmup_on_launch);
        assert!(profile.lesson_packs.is_empty());
        assert_eq!(profile.auto_advance, AutoAdvance::Inside);
    }

    #[test]
    fn warmup_on_launch_can_be_turned_off() {
        let path = dir("warmup-off");
        let mut store = AppStore::default();
        let mut off = profile("off");
        off.warmup_on_launch = false;
        upsert_profile(&mut store, off);
        save(&path, &store).unwrap();
        let loaded = load(&path).unwrap();
        assert!(!loaded.profiles[0].warmup_on_launch);
    }

    #[test]
    fn auto_advance_off_round_trips() {
        let path = dir("advance-off");
        let mut store = AppStore::default();
        let mut horn = profile("off");
        horn.auto_advance = AutoAdvance::Off;
        upsert_profile(&mut store, horn);
        save(&path, &store).unwrap();
        let loaded = load(&path).unwrap();
        assert_eq!(loaded.profiles[0].auto_advance, AutoAdvance::Off);
    }

    #[test]
    fn round_trip_and_profiles_do_not_share_progress() {
        let path = dir("round");
        let mut store = AppStore::default();
        upsert_profile(&mut store, profile("a"));
        set_progress(
            &mut store,
            "a",
            "may-morning-dew",
            "first_sound",
            "settled",
            "heard",
        );
        upsert_profile(&mut store, profile("b"));
        set_progress(
            &mut store,
            "b",
            "may-morning-dew",
            "first_sound",
            "started",
            "",
        );
        save(&path, &store).unwrap();

        let loaded = load(&path).unwrap();
        let a = loaded
            .progress
            .iter()
            .find(|p| p.profile_id == "a")
            .unwrap();
        let b = loaded
            .progress
            .iter()
            .find(|p| p.profile_id == "b")
            .unwrap();
        assert_eq!(a.state_reached, "settled");
        assert_eq!(a.via, "heard");
        assert_eq!(b.state_reached, "started");
        assert!(!path.join("low-d-store.json.tmp").exists());
    }

    #[test]
    fn heard_settle_is_not_replaced_by_a_step() {
        let mut store = AppStore::default();
        upsert_profile(&mut store, profile("a"));
        set_progress(
            &mut store,
            "a",
            "may-morning-dew",
            "first_sound",
            "settled",
            "heard",
        );
        set_progress(
            &mut store,
            "a",
            "may-morning-dew",
            "first_sound",
            "settled",
            "stepped",
        );
        set_progress(
            &mut store,
            "a",
            "may-morning-dew",
            "first_sound",
            "started",
            "",
        );
        let entry = store
            .progress
            .iter()
            .find(|p| p.profile_id == "a" && p.node_id == "first_sound")
            .unwrap();
        assert_eq!(entry.state_reached, "settled");
        assert_eq!(entry.via, "heard");
    }

    #[test]
    fn cnat_is_written_onto_this_whistle_only() {
        let mut store = AppStore::default();
        upsert_profile(&mut store, profile("a"));
        upsert_profile(&mut store, profile("b"));
        set_cnat(&mut store, "a", "oxxooo");
        set_cnat(&mut store, "a", "oxxooo");
        let a = store.profiles.iter().find(|p| p.profile_id == "a").unwrap();
        let b = store.profiles.iter().find(|p| p.profile_id == "b").unwrap();
        assert_eq!(a.cnat_fingering.as_deref(), Some("oxxooo"));
        assert!(b.cnat_fingering.is_none());
        assert!(a.break_hz > 0.0);
    }

    #[test]
    fn corrupt_record_is_set_aside_and_not_replaced() {
        let path = dir("corrupt");
        let file = store_path(&path);
        fs::write(&file, "{ not json").unwrap();
        let err = load(&path).unwrap_err();
        assert!(err.contains("could not be read"), "{err}");
        assert!(!file.exists());
        let bak = fs::read_dir(&path)
            .unwrap()
            .filter_map(|e| e.ok())
            .any(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with("low-d-store.json.bak-")
            });
        assert!(bak);
        let fresh = load(&path).unwrap();
        assert!(fresh.profiles.is_empty());
    }

    #[test]
    fn flat_progress_attaches_to_the_active_profile() {
        let path = dir("migrate");
        let body = r#"{
          "profiles": [{
            "profile_id": "p1",
            "label": "horn",
            "break_hz": 290.0,
            "rms_floor": 0.02,
            "cal_as_of": "t",
            "reads": "no",
            "background": "none"
          }],
          "active_profile_id": "p1",
          "progress": [{ "node_id": "first_sound", "state_reached": "settled", "as_of": "t" }]
        }"#;
        fs::write(store_path(&path), body).unwrap();
        let loaded = load(&path).unwrap();
        assert_eq!(loaded.progress[0].profile_id, "p1");
        assert_eq!(loaded.progress[0].pack_id, "may-morning-dew");
        assert_eq!(loaded.progress[0].via, "heard");
        assert!(loaded.profiles[0].cnat_fingering.is_none());
        let again = load(&path).unwrap();
        assert_eq!(again.progress[0].profile_id, "p1");
    }
}
