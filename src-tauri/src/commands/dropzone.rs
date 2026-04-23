use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::Manager;

#[derive(Serialize)]
pub struct DroppedFile {
    file_name: String,
    file_path: String,
    file_size: u64,
    stored_path: String,
}

fn get_dropzone_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(data_dir.join("dropzone"))
}

fn is_within_dropzone(path: &Path, dropzone_dir: &Path) -> Result<(), String> {
    let canonical = path
        .canonicalize()
        .map_err(|e| format!("Invalid path: {}", e))?;
    let canonical_dir = dropzone_dir
        .canonicalize()
        .map_err(|e| format!("Invalid dropzone dir: {}", e))?;
    if !canonical.starts_with(&canonical_dir) {
        return Err("Path is outside dropzone directory".into());
    }
    Ok(())
}

fn is_sensitive_path(path: &Path) -> bool {
    let lower = path.to_string_lossy().to_lowercase();
    let forbidden = [
        r"\windows\",
        r"\program files\",
        r"\programdata\",
        r"\appdata\",
        r"\.ssh\",
        r"\.gnupg\",
        r"\ntuser.dat",
    ];
    forbidden.iter().any(|f| lower.contains(f))
}

#[tauri::command]
pub async fn dropzone_store(
    app: tauri::AppHandle,
    file_paths: Vec<String>,
) -> Result<Vec<DroppedFile>, String> {
    let store_dir = get_dropzone_dir(&app)?;
    fs::create_dir_all(&store_dir).map_err(|e| e.to_string())?;

    let mut results = Vec::new();

    for src_str in file_paths {
        let src = PathBuf::from(&src_str);
        if !src.exists() || is_sensitive_path(&src) {
            continue;
        }

        let file_name = src
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let ext = src.extension().map(|e| format!(".{}", e.to_string_lossy()));
        let stored_name = format!("{}_{}{}", file_name, ts, ext.unwrap_or_default());
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
pub async fn dropzone_copy_out(
    app: tauri::AppHandle,
    stored_path: String,
    target_dir: String,
) -> Result<String, String> {
    let dropzone_dir = get_dropzone_dir(&app)?;
    let src = PathBuf::from(&stored_path);
    if !src.exists() {
        return Err("Source file not found".into());
    }
    is_within_dropzone(&src, &dropzone_dir)?;

    let target = PathBuf::from(&target_dir);

    // Validate target directory exists and is actually a directory
    if !target.is_dir() {
        return Err("Target path is not a directory".into());
    }

    // Prevent writing to sensitive system directories
    let target_canonical = target
        .canonicalize()
        .map_err(|e| format!("Invalid target directory: {}", e))?;
    let target_str = target_canonical.to_string_lossy().to_lowercase();
    let forbidden = [
        r"\windows\system32",
        r"\windows\syswow64",
        r"\program files",
        r"\programdata",
    ];
    for f in &forbidden {
        if target_str.contains(f) {
            return Err("Cannot export to system directories".into());
        }
    }

    let file_name = src
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let dest = target_canonical.join(&file_name);
    fs::copy(&src, &dest).map_err(|e| e.to_string())?;

    Ok(dest.to_string_lossy().to_string())
}

fn is_expected_stored_name(file_name: &str) -> bool {
    // Expected pattern: {original_name}_{nanosecond_timestamp}.{ext}
    // Strip extension, then check stem ends with _{10+ digits}
    let stem = file_name
        .rfind('.')
        .map_or(file_name, |pos| &file_name[..pos]);
    if let Some(pos) = stem.rfind('_') {
        let after = &stem[pos + 1..];
        return after.len() >= 10 && after.chars().all(|c| c.is_ascii_digit());
    }
    false
}

#[tauri::command]
pub async fn dropzone_delete(app: tauri::AppHandle, stored_path: String) -> Result<(), String> {
    let dropzone_dir = get_dropzone_dir(&app)?;
    let path = PathBuf::from(&stored_path);
    if path.exists() {
        is_within_dropzone(&path, &dropzone_dir)?;
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        if !is_expected_stored_name(&file_name) {
            return Err("Invalid stored file name".into());
        }
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}
