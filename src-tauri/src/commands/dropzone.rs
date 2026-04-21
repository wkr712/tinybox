use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize)]
pub struct DroppedFile {
    file_name: String,
    file_path: String,
    file_size: u64,
    stored_path: String,
}

#[tauri::command]
pub async fn dropzone_store(app: tauri::AppHandle, file_paths: Vec<String>) -> Result<Vec<DroppedFile>, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let store_dir = data_dir.join("dropzone");
    fs::create_dir_all(&store_dir).map_err(|e| e.to_string())?;

    let mut results = Vec::new();

    for src_str in file_paths {
        let src = PathBuf::from(&src_str);
        if !src.exists() {
            continue;
        }

        let file_name = src
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        // Create unique stored name to avoid collisions
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let ext = src.extension().map(|e| format!(".{}", e.to_string_lossy()));
        let stored_name = format!("{}_{:?}{}", file_name, ts, ext.unwrap_or_default());
        let stored_path = store_dir.join(&stored_name);

        fs::copy(&src, &stored_path).map_err(|e| e.to_string())?;

        let metadata = fs::metadata(&stored_path).map_err(|e| e.to_string())?;

        results.push(DroppedFile {
            file_name,
            file_path: src_str,
            file_size: metadata.len(),
            stored_path: stored_path.to_string_lossy().to_string(),
        });
    }

    Ok(results)
}

#[tauri::command]
pub async fn dropzone_copy_out(stored_path: String, target_dir: String) -> Result<String, String> {
    let src = PathBuf::from(&stored_path);
    if !src.exists() {
        return Err("Source file not found".into());
    }

    let target = PathBuf::from(&target_dir);
    fs::create_dir_all(&target).map_err(|e| e.to_string())?;

    let file_name = src
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let dest = target.join(&file_name);
    fs::copy(&src, &dest).map_err(|e| e.to_string())?;

    Ok(dest.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn dropzone_delete(stored_path: String) -> Result<(), String> {
    let path = PathBuf::from(&stored_path);
    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}
