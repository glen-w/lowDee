use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

pub use crate::listen::attempt::{
    AttemptConfig, AttemptEngine, AttemptMode, AttemptResult, TakeSlot,
};
use crate::listen::mic::MicSession;
use crate::listen::ornaments::GestureKind;
use crate::listen::types::{Background, NoteName, Reads, WhistleProfile};
use crate::listen::Evidence;
use crate::pack::{self, Catalog, Pack, ProgressMark};
use crate::store::{self, AppStore};

pub struct AppState {
    pub pack: Mutex<Pack>,
    pub catalog: Mutex<Catalog>,
    pub store_dir: Mutex<PathBuf>,
    pub mic: Mutex<Option<MicSession>>,
    /// Frozen target for the in-flight attempt (UI display).
    pub attempt_target_hz: Mutex<Option<f32>>,
    /// Last heard take, as wav bytes. Memory only. Cleared when the attempt drops.
    pub last_take: Mutex<TakeSlot>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProfileArgs {
    pub label: String,
    pub reads: String,
    pub background: String,
}

#[derive(Debug, Deserialize)]
pub struct StartAttemptArgs {
    pub node_id: String,
    pub mode: String,
    pub note: Option<String>,
    pub notes: Option<Vec<String>>,
    pub want_octave: Option<bool>,
    pub gesture: Option<String>,
    pub breaths: Option<Vec<usize>>,
    pub marks: Option<Vec<MarkArg>>,
    /// A between-attempt low-D hold. Does not mark the node on screen.
    pub recalibrate: Option<bool>,
    /// False for a review or a single-note spot. Those do not write progress.
    pub record: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct MarkArg {
    pub note_index: usize,
    pub gesture: String,
}

#[derive(Debug, Serialize)]
pub struct FrameDto {
    pub t_ms: f32,
    pub hz: Option<f32>,
    pub rms: f32,
    pub confidence: f32,
    pub near_target: bool,
    pub early_break: bool,
    pub expected_note: Option<String>,
    pub phrase_index: usize,
    pub hold_ratio: f32,
    pub leak_hole: Option<u8>,
}

fn parse_reads(s: &str) -> Reads {
    match s {
        "some" => Reads::Some,
        "yes" => Reads::Yes,
        _ => Reads::No,
    }
}

fn parse_background(s: &str) -> Background {
    match s {
        "wind" => Background::Wind,
        "other" => Background::Other,
        "high_d" => Background::HighD,
        _ => Background::None,
    }
}

fn data_dir(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from(".").join("low-d-data"))
}

#[tauri::command]
pub fn get_pack(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let pack = state.pack.lock().map_err(|e| e.to_string())?;
    Ok(json!({
        "manifest": {
            "id": pack.manifest.id,
            "version": pack.manifest.version,
            "track": pack.manifest.track,
            "title": pack.manifest.title,
            "node_ids": pack.manifest.node_ids,
            "nodes": pack.manifest.nodes,
            "pages": pack.manifest.pages,
            "wave": pack.manifest.wave,
            "after": pack.manifest.after,
            "book_ref": pack.manifest.book_ref,
            "shelf": pack.manifest.shelf,
            "rights": pack.manifest.rights,
            "aka": pack.manifest.aka,
            "source": pack.manifest.source,
            "session": pack.manifest.session,
            "pulse": pack.manifest.pulse,
            "pulse_beats": pack.manifest.pulse_beats,
            "cnat_id": pack.manifest.cnat_id,
        },
        "phrases": pack.phrases,
        "fingering": pack.fingering,
        "ornaments": pack.ornaments,
        "remarks": pack.remarks,
        "tune_abc": pack.tune_abc,
        "words": pack.words,
    }))
}

#[tauri::command]
pub fn get_catalog(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<pack::CatalogView, String> {
    let catalog = state.catalog.lock().map_err(|e| e.to_string())?;
    let store = store::load(&data_dir(&app))?;
    let profile_id = store.active_profile_id.clone().unwrap_or_default();
    let marks: Vec<ProgressMark> = store
        .progress
        .iter()
        .map(|p| ProgressMark {
            profile_id: p.profile_id.clone(),
            pack_id: p.pack_id.clone(),
            node_id: p.node_id.clone(),
            state_reached: p.state_reached.clone(),
        })
        .collect();
    Ok(pack::catalog_view(&catalog, &marks, &profile_id))
}

#[tauri::command]
pub fn open_pack(
    app: AppHandle,
    state: State<'_, AppState>,
    pack_id: String,
) -> Result<(), String> {
    let catalog = state.catalog.lock().map_err(|e| e.to_string())?;
    let pack = catalog
        .packs
        .iter()
        .find(|p| p.manifest.id == pack_id && p.playable)
        .cloned()
        .ok_or_else(|| "pack not found".to_string())?;
    if pack.origin == "teacher" {
        let store = store::load(&data_dir(&app))?;
        let profile_id = store.active_profile_id.clone().unwrap_or_default();
        let marks: Vec<ProgressMark> = store
            .progress
            .iter()
            .map(|p| ProgressMark {
                profile_id: p.profile_id.clone(),
                pack_id: p.pack_id.clone(),
                node_id: p.node_id.clone(),
                state_reached: p.state_reached.clone(),
            })
            .collect();
        let view = pack::catalog_view(&catalog, &marks, &profile_id);
        if !view.desk.iter().any(|p| p.id == pack_id && p.open) {
            return Err("The desk opens after The May Morning Dew.".into());
        }
    }
    drop(catalog);
    *state.pack.lock().map_err(|e| e.to_string())? = pack;
    Ok(())
}

#[tauri::command]
pub fn get_store(app: AppHandle, state: State<'_, AppState>) -> Result<AppStore, String> {
    let dir = data_dir(&app);
    *state.store_dir.lock().map_err(|e| e.to_string())? = dir.clone();
    store::load(&dir)
}

#[tauri::command]
pub fn save_profile(
    app: AppHandle,
    state: State<'_, AppState>,
    args: CreateProfileArgs,
) -> Result<WhistleProfile, String> {
    let dir = data_dir(&app);
    let mut st = store::load(&dir)?;
    let profile = WhistleProfile {
        profile_id: uuid::Uuid::new_v4().to_string(),
        label: args.label,
        break_hz: 0.0,
        rms_floor: 0.0,
        cal_as_of: String::new(),
        reads: parse_reads(&args.reads),
        background: parse_background(&args.background),
        cnat_fingering: None,
    };
    store::upsert_profile(&mut st, profile.clone());
    store::save(&dir, &st)?;
    *state.store_dir.lock().map_err(|e| e.to_string())? = dir;
    Ok(profile)
}

#[tauri::command]
pub fn update_profile_answers(
    app: AppHandle,
    profile_id: String,
    reads: String,
    background: String,
) -> Result<WhistleProfile, String> {
    let dir = data_dir(&app);
    let mut st = store::load(&dir)?;
    let profile = st
        .profiles
        .iter_mut()
        .find(|p| p.profile_id == profile_id)
        .ok_or_else(|| "profile not found".to_string())?;
    profile.reads = parse_reads(&reads);
    profile.background = parse_background(&background);
    let out = profile.clone();
    store::save(&dir, &st)?;
    Ok(out)
}

#[tauri::command]
pub fn select_profile(app: AppHandle, profile_id: String) -> Result<(), String> {
    let dir = data_dir(&app);
    let mut st = store::load(&dir)?;
    if !st.profiles.iter().any(|p| p.profile_id == profile_id) {
        return Err("profile not found".into());
    }
    st.active_profile_id = Some(profile_id);
    store::save(&dir, &st)
}

#[tauri::command]
pub fn set_progress(
    app: AppHandle,
    state: State<'_, AppState>,
    node_id: String,
    state_reached: String,
    via: Option<String>,
) -> Result<(), String> {
    write_progress(
        &app,
        &state,
        &node_id,
        &state_reached,
        via.unwrap_or_default().as_str(),
    )
}

fn write_progress(
    app: &AppHandle,
    state: &State<'_, AppState>,
    node_id: &str,
    state_reached: &str,
    via: &str,
) -> Result<(), String> {
    let dir = data_dir(app);
    let mut st = store::load(&dir)?;
    let profile_id = st
        .active_profile_id
        .clone()
        .ok_or_else(|| "no whistle profile".to_string())?;
    let pack_id = state
        .pack
        .lock()
        .map_err(|e| e.to_string())?
        .manifest
        .id
        .clone();
    store::set_progress(&mut st, &profile_id, &pack_id, node_id, state_reached, via);
    store::save(&dir, &st)
}

#[tauri::command]
pub fn ref_available(state: State<'_, AppState>, relative: String) -> Result<bool, String> {
    let pack = state.pack.lock().map_err(|e| e.to_string())?;
    Ok(pack::ref_exists(&pack, &relative))
}

#[tauri::command]
pub fn read_ref(
    state: State<'_, AppState>,
    relative: String,
) -> Result<tauri::ipc::Response, String> {
    let pack = state.pack.lock().map_err(|e| e.to_string())?;
    let path = pack::ref_file(&pack, &relative).ok_or_else(|| "ref not found".to_string())?;
    let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
    Ok(tauri::ipc::Response::new(bytes))
}

fn active_profile(app: &AppHandle) -> Result<Option<WhistleProfile>, String> {
    let dir = data_dir(app);
    let st = store::load(&dir)?;
    let Some(id) = st.active_profile_id else {
        return Ok(None);
    };
    Ok(st.profiles.into_iter().find(|p| p.profile_id == id))
}

fn parse_notes(notes: &[String]) -> Result<Vec<NoteName>, String> {
    notes
        .iter()
        .map(|s| NoteName::from_str(s).ok_or_else(|| format!("unknown note {s}")))
        .collect()
}

fn build_mode(args: &StartAttemptArgs) -> Result<AttemptMode, String> {
    match args.mode.as_str() {
        "first_sound" => Ok(AttemptMode::FirstSound),
        "breath_octave" => Ok(AttemptMode::BreathOctave {
            want_octave: args.want_octave.unwrap_or(false),
        }),
        "single_note" => {
            let note = args
                .note
                .as_deref()
                .and_then(NoteName::from_str)
                .ok_or("note required")?;
            Ok(AttemptMode::SingleNote { note })
        }
        "phrase" => {
            let notes = parse_notes(args.notes.as_deref().unwrap_or(&[]))?;
            let breaths = args.breaths.clone().unwrap_or_default();
            let marks = args
                .marks
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .map(|m| {
                    let gesture = GestureKind::from_str(&m.gesture)
                        .ok_or_else(|| format!("unknown gesture {}", m.gesture))?;
                    Ok(crate::listen::attempt::PhraseMark {
                        note_index: m.note_index,
                        gesture,
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(AttemptMode::Phrase {
                notes,
                breaths,
                marks,
            })
        }
        "on_the_breath" => {
            let notes = parse_notes(args.notes.as_deref().unwrap_or(&[]))?;
            Ok(AttemptMode::OnTheBreath { notes })
        }
        "ornament" => {
            let note = args
                .note
                .as_deref()
                .and_then(NoteName::from_str)
                .unwrap_or(NoteName::A4);
            let gesture = GestureKind::from_str(args.gesture.as_deref().unwrap_or("cut"))
                .unwrap_or(GestureKind::Cut);
            Ok(AttemptMode::Ornament { note, gesture })
        }
        other => Err(format!("unknown mode {other}")),
    }
}

#[tauri::command]
pub fn start_attempt(
    app: AppHandle,
    state: State<'_, AppState>,
    args: StartAttemptArgs,
) -> Result<serde_json::Value, String> {
    // Stop any prior session, and the take that belonged to it.
    {
        let mut mic = state.mic.lock().map_err(|e| e.to_string())?;
        *mic = None;
    }
    state
        .last_take
        .lock()
        .map_err(|e| e.to_string())?
        .drop_take();

    let profile = active_profile(&app)?;
    let break_hz = profile.as_ref().and_then(|p| {
        if p.break_hz > 0.0 {
            Some(p.break_hz)
        } else {
            None
        }
    });
    let rms_floor = profile.as_ref().and_then(|p| {
        if p.rms_floor > 0.0 {
            Some(p.rms_floor)
        } else {
            None
        }
    });

    let mode = build_mode(&args)?;
    let cfg = AttemptConfig {
        sample_rate: 44100,
        hop_ms: 30.0,
        mode,
        break_hz,
        rms_floor,
        recalibrate: args.recalibrate.unwrap_or(false),
    };

    // Prefer live mic; fall back to a silent engine so UI still works in CI
    let session = match MicSession::start(cfg.clone()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("mic unavailable ({e})");
            return Ok(json!({
                "ok": false,
                "mic": false,
                "error": e,
            }));
        }
    };

    let target = session.frozen_target();
    *state.attempt_target_hz.lock().map_err(|e| e.to_string())? = target;
    *state.mic.lock().map_err(|e| e.to_string())? = Some(session);

    // Mark node started. A review and a single-note spot do not.
    if args.record.unwrap_or(true) && !args.recalibrate.unwrap_or(false) {
        let _ = write_progress(&app, &state, &args.node_id, "started", "");
    }

    Ok(json!({
        "ok": true,
        "mic": true,
        "target_hz": target,
    }))
}

#[tauri::command]
pub fn set_grading(state: State<'_, AppState>, enabled: bool) -> Result<(), String> {
    if let Some(mic) = state.mic.lock().map_err(|e| e.to_string())?.as_ref() {
        mic.set_grading(enabled);
    }
    Ok(())
}

#[tauri::command]
pub fn poll_frame(state: State<'_, AppState>) -> Result<Option<FrameDto>, String> {
    let mic = state.mic.lock().map_err(|e| e.to_string())?;
    let Some(session) = mic.as_ref() else {
        return Ok(None);
    };
    let frame = session.latest_frame.lock().clone();
    Ok(frame.map(|f| FrameDto {
        t_ms: f.t_ms,
        hz: f.hz,
        rms: f.rms,
        confidence: f.confidence,
        near_target: f.near_target,
        early_break: f.early_break,
        expected_note: f.expected_note,
        phrase_index: f.phrase_index,
        hold_ratio: f.hold_ratio,
        leak_hole: f.leak_hole,
    }))
}

#[tauri::command]
pub fn finish_attempt(
    app: AppHandle,
    state: State<'_, AppState>,
    node_id: String,
    mark_settled: Option<bool>,
    recalibrate: Option<bool>,
    cnat_fingering: Option<String>,
) -> Result<serde_json::Value, String> {
    let mut result = {
        let mut mic = state.mic.lock().map_err(|e| e.to_string())?;
        match mic.take() {
            Some(session) => session.finish(),
            None => {
                // No mic — couldnt_hear
                AttemptResult {
                    evidence: Evidence::CouldntHear,
                    settled: false,
                    break_hz: None,
                    rms_floor: None,
                    frames: vec![],
                    target_hz: *state.attempt_target_hz.lock().map_err(|e| e.to_string())?,
                    remark_note: None,
                    warm: false,
                    cnat_disagree: false,
                    ghost: None,
                    isolate_note: None,
                    take_wav: None,
                }
            }
        }
    };

    let recalibrate = recalibrate.unwrap_or(false);
    // Calibration is the settled low-D hold. Recalibrate uses that same hold
    // between attempts and does not write progress for the node on screen.
    if node_id == "first_sound" || recalibrate {
        if let (Some(bh), Some(rf)) = (result.break_hz, result.rms_floor) {
            let dir = data_dir(&app);
            let mut st = store::load(&dir)?;
            if let Some(id) = st.active_profile_id.clone() {
                if let Some(p) = st.profiles.iter_mut().find(|p| p.profile_id == id) {
                    p.break_hz = bh;
                    p.rms_floor = rf;
                    p.cal_as_of = chrono::Utc::now().to_rfc3339();
                }
            }
            store::save(&dir, &st)?;
        }
    }

    if result.settled && mark_settled.unwrap_or(true) && !recalibrate {
        let _ = write_progress(&app, &state, &node_id, "settled", "heard");
        let (pack_id, cnat_id) = {
            let pack = state.pack.lock().map_err(|e| e.to_string())?;
            (pack.manifest.id.clone(), pack.manifest.cnat_id.clone())
        };
        if pack_id == "c-natural" && node_id == "cnat_hold" && !cnat_id.is_empty() {
            let chosen = cnat_fingering
                .filter(|id| id == "oxxooo" || id == "oxxoxx")
                .unwrap_or(cnat_id);
            let dir = data_dir(&app);
            let mut st = store::load(&dir)?;
            if let Some(id) = st.active_profile_id.clone() {
                store::set_cnat(&mut st, &id, &chosen);
            }
            store::save(&dir, &st)?;
        }
    }

    let has_take = result.take_wav.is_some();
    state
        .last_take
        .lock()
        .map_err(|e| e.to_string())?
        .store(result.take_wav.take());

    let target_after = result.target_hz;
    Ok(json!({
        "evidence": result.evidence,
        "settled": result.settled,
        "break_hz": result.break_hz,
        "rms_floor": result.rms_floor,
        "target_hz": target_after,
        "remark_note": result.remark_note,
        "frame_count": result.frames.len(),
        "warm": result.warm,
        "cnat_disagree": result.cnat_disagree,
        "ghost": result.ghost,
        "isolate_note": result.isolate_note,
        "has_take": has_take,
    }))
}

/// Release the microphone when the player leaves a part. Writes no progress.
#[tauri::command]
pub fn drop_attempt(state: State<'_, AppState>) -> Result<(), String> {
    *state.mic.lock().map_err(|e| e.to_string())? = None;
    *state.attempt_target_hz.lock().map_err(|e| e.to_string())? = None;
    state
        .last_take
        .lock()
        .map_err(|e| e.to_string())?
        .drop_take();
    Ok(())
}

/// The last take, as wav bytes. Absent after a drop, a couldn’t-hear, or an abstain.
#[tauri::command]
pub fn read_last_take(state: State<'_, AppState>) -> Result<Vec<u8>, String> {
    let slot = state.last_take.lock().map_err(|e| e.to_string())?;
    slot.bytes()
        .map(|bytes| bytes.to_vec())
        .ok_or_else(|| "no take".into())
}

#[tauri::command]
pub fn step_past(
    app: AppHandle,
    state: State<'_, AppState>,
    node_id: String,
) -> Result<(), String> {
    // The player moved on without a listen result. Hedwig uses this too:
    // the tune is not in the pack, so nothing was heard.
    write_progress(&app, &state, &node_id, "settled", "stepped")
}

#[tauri::command]
pub fn get_frozen_target(state: State<'_, AppState>) -> Result<Option<f32>, String> {
    Ok(*state.attempt_target_hz.lock().map_err(|e| e.to_string())?)
}

/// Offline grade of raw mono f32 samples (for tests / harness).
#[allow(dead_code)]
pub fn grade_samples(cfg: AttemptConfig, samples: &[f32]) -> AttemptResult {
    let mut eng = AttemptEngine::new(cfg);
    let chunk = 1024;
    for c in samples.chunks(chunk) {
        eng.push_samples(c);
    }
    eng.finish()
}
