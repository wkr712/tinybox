mod commands;
mod services;
mod tray;

use services::audio_player::AudioState;
use services::clipboard_monitor::ClipboardMonitor;
use services::ncm::NcmState;
use std::sync::Mutex;
use tauri::Manager;

pub struct ClipboardMonitorState(Mutex<Option<ClipboardMonitor>>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations(
                    "sqlite:tinybox.db",
                    vec![tauri_plugin_sql::Migration {
                        version: 1,
                        description: "create_initial_tables",
                        sql: include_str!("../sql/migrations/v1.sql"),
                        kind: tauri_plugin_sql::MigrationKind::Up,
                    }],
                )
                .build(),
        )
        .manage(ClipboardMonitorState(Mutex::new(None)))
        .manage(NcmState::new())
        .manage(AudioState::new())
        .setup(|app| {
            tray::create_tray(app)?;

            let monitor = ClipboardMonitor::new(app.handle().clone(), 100);
            app.state::<ClipboardMonitorState>()
                .0
                .lock()
                .unwrap()
                .replace(monitor);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::clipboard::clipboard_write_text,
            commands::dropzone::dropzone_store,
            commands::dropzone::dropzone_copy_out,
            commands::dropzone::dropzone_delete,
            commands::music::music_qr_generate,
            commands::music::music_qr_check,
            commands::music::music_login_status,
            commands::music::music_user_playlist,
            commands::music::music_playlist_detail,
            commands::music::music_song_url,
            commands::music::music_song_detail,
            commands::music::music_lyric,
            commands::music::music_search,
            commands::music::music_play,
            commands::music::music_pause,
            commands::music::music_resume,
            commands::music::music_stop,
            commands::music::music_set_volume,
        ])
        .run(tauri::generate_context!())
        .expect("error while running TinyBox");
}
