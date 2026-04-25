use crate::ClipboardMonitorState;

#[tauri::command]
pub async fn clipboard_write_text(text: String) -> Result<(), String> {
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(&text).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn clipboard_set_monitor_paused(
    state: tauri::State<'_, ClipboardMonitorState>,
    paused: bool,
) -> Result<(), String> {
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    if let Some(monitor) = guard.as_ref() {
        monitor.set_paused(paused);
    }
    Ok(())
}
