mod book;
mod commands;
mod song;
mod midi;
pub mod listen;
mod pack;
mod store;

// Re-export for the listen CLI binary
pub use listen::Evidence;

use std::path::PathBuf;
use std::sync::Mutex;

use commands::AppState;
use pack::resolve_pack_dir;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let roots = pack::catalog_roots();
    let catalog = pack::load_catalog(&roots).unwrap_or_else(|e| {
        let alt = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../pack");
        pack::load_catalog(&[alt]).unwrap_or_else(|e2| panic!("pack required: {e}; {e2}"))
    });
    let pack = catalog
        .packs
        .iter()
        .find(|p| p.manifest.id == "may-morning-dew")
        .cloned()
        .or_else(|| catalog.packs.iter().find(|p| p.playable).cloned())
        .expect("a playable pack");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            pack: Mutex::new(pack),
            catalog: Mutex::new(catalog),
            store_dir: Mutex::new(std::path::PathBuf::from(".")),
            mic: Mutex::new(None),
            attempt_target_hz: Mutex::new(None),
            last_take: Mutex::new(commands::TakeSlot::default()),
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_pack,
            commands::get_catalog,
            commands::open_pack,
            commands::get_store,
            commands::save_profile,
            commands::update_profile_answers,
            commands::select_profile,
            commands::set_lesson_packs,
            commands::set_progress,
            commands::ref_available,
            commands::read_ref,
            commands::book_clip,
            commands::read_book,
            commands::song_sheet,
            commands::read_last_take,
            commands::start_attempt,
            commands::set_grading,
            commands::poll_frame,
            commands::finish_attempt,
            commands::drop_attempt,
            commands::step_past,
            commands::get_frozen_target,
        ])
        .setup(|app| {
            let dir = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| PathBuf::from(".").join("low-d-data"));
            std::fs::create_dir_all(&dir).ok();
            {
                let state = app.state::<AppState>();
                let mut d = state.store_dir.lock().map_err(|e| e.to_string())?;
                *d = dir;
            }
            let mut candidates = vec![
                std::env::current_dir().unwrap_or_default().join("pack"),
                std::env::current_dir().unwrap_or_default().join("../pack"),
            ];
            if let Ok(resource) = app.path().resource_dir() {
                candidates.push(resource.join("pack"));
            }
            if let Some(root) = resolve_pack_dir().parent() {
                candidates.push(root.to_path_buf());
            }
            for root in candidates {
                if !root.join("may-morning-dew/manifest.json").exists() {
                    continue;
                }
                let mut roots = vec![root.clone()];
                if let Some(parent) = root.parent() {
                    let teacher = parent.join("teacher");
                    if teacher.is_dir() {
                        roots.push(teacher);
                    }
                }
                if let Ok(found) = pack::load_catalog(&roots) {
                    let state = app.state::<AppState>();
                    if let Some(door) = found
                        .packs
                        .iter()
                        .find(|p| p.manifest.id == "may-morning-dew")
                        .cloned()
                    {
                        let mut slot = state.pack.lock().map_err(|e| e.to_string())?;
                        *slot = door;
                    }
                    let mut cat = state.catalog.lock().map_err(|e| e.to_string())?;
                    *cat = found;
                    break;
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
