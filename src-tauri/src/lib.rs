mod commands;
mod services;
mod tray;

use services::audio_player::AudioState;
use services::clipboard_monitor::ClipboardMonitor;
use services::music_provider::ProviderRegistry;
use std::sync::Mutex;
use tauri::Manager;

pub struct ClipboardMonitorState(Mutex<Option<ClipboardMonitor>>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // When second instance launches, focus the existing window
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
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
        .setup(|app| {
            tray::create_tray(app)?;

            // Position window at top-center of primary monitor
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_skip_taskbar(true);
                let _ = window.set_always_on_top(true);

                if let Some(monitor) = window.primary_monitor().ok().flatten() {
                    let scale = monitor.scale_factor();
                    let screen_w = monitor.size().width as f64 / scale;
                    let screen_h = monitor.size().height as f64 / scale;
                    let win_w: f64 = 52.0;
                    let _win_h: f64 = 380.0;
                    let x = ((screen_w - win_w) / 2.0).round();
                    let y = (screen_h * 0.10).round();
                    use tauri::Position;
                    let _ = window.set_position(Position::Logical(tauri::LogicalPosition::new(x, y)));
                }
                // Window starts hidden (visible: false in config).
                // Frontend calls "window_ready" to show with fade-in.
            }

            app.manage(AudioState::new(app.handle().clone()));

            let data_dir = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| std::env::temp_dir());
            app.manage(ProviderRegistry::new(data_dir));

            let monitor = ClipboardMonitor::new(app.handle().clone());
            if let Ok(mut guard) = app.state::<ClipboardMonitorState>().0.lock() {
                guard.replace(monitor);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::clipboard::clipboard_write_text,
            commands::clipboard::clipboard_set_monitor_paused,
            commands::dropzone::dropzone_store,
            commands::dropzone::dropzone_copy_out,
            commands::dropzone::dropzone_delete,
            commands::window::window_ready,
            commands::music::music_qr_generate,
            commands::music::music_qr_check,
            commands::music::music_login_status,
            commands::music::music_user_playlist,
            commands::music::music_playlist_detail,
            commands::music::music_song_url,
            commands::music::music_lyric,
            commands::music::music_search,
            commands::music::music_play,
            commands::music::music_pause,
            commands::music::music_resume,
            commands::music::music_stop,
            commands::music::music_set_volume,
            commands::music::music_seek,
            commands::music::music_personalized,
            commands::music::music_recommend_songs,
            commands::music::music_search_hot,
            commands::music::music_set_provider,
            commands::music::music_get_provider,
            commands::music::music_logout,
            commands::music::music_spectrum,
        ])
        .run(tauri::generate_context!())
        .expect("error while running TinyBox");
}
