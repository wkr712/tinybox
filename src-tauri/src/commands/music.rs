use crate::services::audio_player::{AudioPlayer, AudioState};
use crate::services::music_provider::ProviderRegistry;
use serde::Serialize;
use std::time::Duration;
use tauri::State;

#[derive(Serialize)]
pub struct QrResult {
    key: String,
    qrurl: String,
}

#[tauri::command]
pub async fn music_qr_generate(registry: State<'_, ProviderRegistry>) -> Result<QrResult, String> {
    let provider = registry.active_provider()?;
    let (key, qrurl) = provider.qr_generate().await?;
    Ok(QrResult { key, qrurl })
}

#[tauri::command]
pub async fn music_qr_check(
    registry: State<'_, ProviderRegistry>,
    key: String,
) -> Result<serde_json::Value, String> {
    let provider = registry.active_provider()?;
    provider.qr_check(&key).await
}

#[tauri::command]
pub async fn music_login_status(
    registry: State<'_, ProviderRegistry>,
) -> Result<serde_json::Value, String> {
    let provider = registry.active_provider()?;
    provider.login_status().await
}

#[tauri::command]
pub async fn music_user_playlist(
    registry: State<'_, ProviderRegistry>,
    uid: i64,
) -> Result<serde_json::Value, String> {
    let provider = registry.active_provider()?;
    provider.user_playlists(uid).await
}

#[tauri::command]
pub async fn music_playlist_detail(
    registry: State<'_, ProviderRegistry>,
    id: i64,
) -> Result<serde_json::Value, String> {
    let provider = registry.active_provider()?;
    provider.playlist_detail(id).await
}

#[tauri::command]
pub async fn music_song_url(
    registry: State<'_, ProviderRegistry>,
    id: String,
) -> Result<String, String> {
    let provider = registry.active_provider()?;
    provider.song_url(&id).await
}

#[tauri::command]
pub async fn music_lyric(
    registry: State<'_, ProviderRegistry>,
    id: String,
) -> Result<serde_json::Value, String> {
    let provider = registry.active_provider()?;
    provider.lyric(&id).await
}

#[tauri::command]
pub async fn music_search(
    registry: State<'_, ProviderRegistry>,
    keywords: String,
) -> Result<serde_json::Value, String> {
    let provider = registry.active_provider()?;
    provider.search(&keywords, 30).await
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
pub fn music_seek(state: State<'_, AudioState>, position_ms: u64) -> Result<(), String> {
    AudioPlayer::seek(&state, Duration::from_millis(position_ms))
}

#[tauri::command]
pub async fn music_personalized(
    registry: State<'_, ProviderRegistry>,
    limit: i64,
) -> Result<serde_json::Value, String> {
    let provider = registry.active_provider()?;
    provider.personalized(limit).await
}

#[tauri::command]
pub async fn music_recommend_songs(
    registry: State<'_, ProviderRegistry>,
) -> Result<serde_json::Value, String> {
    let provider = registry.active_provider()?;
    provider.recommend_songs().await
}

#[tauri::command]
pub async fn music_search_hot(
    registry: State<'_, ProviderRegistry>,
) -> Result<serde_json::Value, String> {
    let provider = registry.active_provider()?;
    provider.hot_searches().await
}

#[tauri::command]
pub async fn music_set_provider(
    registry: State<'_, ProviderRegistry>,
    provider: String,
) -> Result<(), String> {
    registry.set_active(&provider)
}

#[tauri::command]
pub async fn music_get_provider(registry: State<'_, ProviderRegistry>) -> Result<String, String> {
    registry.get_active_kind()
}

#[tauri::command]
pub async fn music_logout(registry: State<'_, ProviderRegistry>) -> Result<(), String> {
    let provider = registry.active_provider()?;
    provider.logout().await
}
