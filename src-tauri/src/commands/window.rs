use tauri::Manager;

#[tauri::command]
pub async fn window_ready(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
    }
    Ok(())
}
