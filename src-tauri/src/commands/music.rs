use crate::services::audio_player::{AudioPlayer, AudioState};
use crate::services::ncm::{NcmService, NcmState};
use serde::Serialize;
use tauri::State;

#[derive(Serialize)]
pub struct QrResult {
    key: String,
    qrurl: String,
}

#[tauri::command]
pub async fn music_qr_generate(state: State<'_, NcmState>) -> Result<QrResult, String> {
    let key_body = NcmService::qr_key(&state).await?;
    let key = key_body["unikey"]
        .as_str()
        .ok_or("Failed to get QR key")?
        .to_string();

    let qr_body = NcmService::qr_create(&state, &key).await?;
    let qrurl = qr_body["data"]["qrurl"]
        .as_str()
        .ok_or("Failed to get QR url")?
        .to_string();

    Ok(QrResult { key, qrurl })
}

#[tauri::command]
pub async fn music_qr_check(state: State<'_, NcmState>, key: String) -> Result<serde_json::Value, String> {
    NcmService::qr_check(&state, &key).await
}

#[tauri::command]
pub async fn music_login_status(state: State<'_, NcmState>) -> Result<serde_json::Value, String> {
    NcmService::login_status(&state).await
}

#[tauri::command]
pub async fn music_user_playlist(state: State<'_, NcmState>, uid: i64) -> Result<serde_json::Value, String> {
    NcmService::user_playlist(&state, uid).await
}

#[tauri::command]
pub async fn music_playlist_detail(state: State<'_, NcmState>, id: i64) -> Result<serde_json::Value, String> {
    NcmService::playlist_detail(&state, id).await
}

#[tauri::command]
pub async fn music_song_url(state: State<'_, NcmState>, id: i64) -> Result<serde_json::Value, String> {
    NcmService::song_url(&state, id).await
}

#[tauri::command]
pub async fn music_song_detail(state: State<'_, NcmState>, ids: String) -> Result<serde_json::Value, String> {
    NcmService::song_detail(&state, &ids).await
}

#[tauri::command]
pub async fn music_lyric(state: State<'_, NcmState>, id: i64) -> Result<serde_json::Value, String> {
    NcmService::lyric(&state, id).await
}

#[tauri::command]
pub async fn music_search(state: State<'_, NcmState>, keywords: String) -> Result<serde_json::Value, String> {
    NcmService::search(&state, &keywords).await
}

#[tauri::command]
pub fn music_play(state: State<'_, AudioState>, url: String) -> Result<(), String> {
    AudioPlayer::play(&state, &url)
}

#[tauri::command]
pub fn music_pause(state: State<'_, AudioState>) -> Result<(), String> {
    AudioPlayer::pause(&state)
}

#[tauri::command]
pub fn music_resume(state: State<'_, AudioState>) -> Result<(), String> {
    AudioPlayer::resume(&state)
}

#[tauri::command]
pub fn music_stop(state: State<'_, AudioState>) -> Result<(), String> {
    AudioPlayer::stop(&state)
}

#[tauri::command]
pub fn music_set_volume(state: State<'_, AudioState>, volume: f32) -> Result<(), String> {
    AudioPlayer::set_volume(&state, volume)
}

#[tauri::command]
pub async fn music_personalized(state: State<'_, NcmState>, limit: i64) -> Result<serde_json::Value, String> {
    NcmService::personalized(&state, limit).await
}

#[tauri::command]
pub async fn music_personalized_newsong(state: State<'_, NcmState>, limit: i64) -> Result<serde_json::Value, String> {
    NcmService::personalized_newsong(&state, limit).await
}

#[tauri::command]
pub async fn music_recommend_songs(state: State<'_, NcmState>) -> Result<serde_json::Value, String> {
    NcmService::recommend_songs(&state).await
}

#[tauri::command]
pub async fn music_search_hot(state: State<'_, NcmState>) -> Result<serde_json::Value, String> {
    NcmService::search_hot_detail(&state).await
}
