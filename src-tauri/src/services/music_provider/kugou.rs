use super::load_cookie;
use super::persist_cookie;
use super::MusicProvider;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct KugouProvider {
    cookie: Mutex<String>,
    data_dir: Option<PathBuf>,
    client: reqwest::Client,
}

impl KugouProvider {
    pub fn new(data_dir: Option<PathBuf>) -> Self {
        let cookie = load_cookie(&data_dir, "kugou_cookie.txt").unwrap_or_default();
        Self {
            cookie: Mutex::new(cookie),
            data_dir,
            client: reqwest::Client::new(),
        }
    }

    fn get_cookie(&self) -> Result<String, String> {
        Ok(self.cookie.lock().map_err(|e| e.to_string())?.clone())
    }

    fn update_cookie(&self, cookie_str: String) -> Result<(), String> {
        let mut guard = self.cookie.lock().map_err(|e| e.to_string())?;
        *guard = cookie_str.clone();
        drop(guard);
        persist_cookie(&self.data_dir, "kugou_cookie.txt", &cookie_str);
        Ok(())
    }

    async fn api_get(&self, url: &str) -> Result<Value, String> {
        let cookie = self.get_cookie()?;
        let mut req = self.client.get(url).header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        );
        if !cookie.is_empty() {
            req = req.header("Cookie", &cookie);
        }
        let resp = req
            .send()
            .await
            .map_err(|e| format!("Kugou request failed: {}", e))?;
        resp.json::<Value>()
            .await
            .map_err(|e| format!("Kugou parse failed: {}", e))
    }
}

#[async_trait]
impl MusicProvider for KugouProvider {
    fn kind(&self) -> &str {
        "kugou"
    }

    async fn qr_generate(&self) -> Result<(String, String), String> {
        let resp = self
            .api_get("https://login-user.kugou.com/v1/get_qr_token")
            .await?;
        let token = resp["data"]["token"]
            .as_str()
            .ok_or("Failed to get Kugou QR token")?
            .to_string();
        let qrcode = format!(
            "https://login-user.kugou.com/v1/scan_qrcode?token={}",
            token
        );
        Ok((token, qrcode))
    }

    async fn qr_check(&self, key: &str) -> Result<Value, String> {
        let url = format!("https://login-user.kugou.com/v1/scan_status?token={}", key);
        let resp = self.api_get(&url).await?;
        if resp["data"]["status"].as_i64() == Some(3) {
            if let Some(cookie) = resp["data"]["cookie"].as_str() {
                self.update_cookie(cookie.to_string())?;
            }
        }
        Ok(resp)
    }

    async fn login_status(&self) -> Result<Value, String> {
        let cookie = self.get_cookie()?;
        if cookie.is_empty() {
            return Ok(json!({"code": 301, "message": "Not logged in"}));
        }
        let resp = self
            .api_get("https://my.kugou.com/v2/user/info.php")
            .await?;
        if let Some(user) = resp["data"].as_object() {
            Ok(json!({
                "code": 200,
                "account": {
                    "id": user["user_id"].as_i64().unwrap_or(0),
                    "userName": user["nick_name"].as_str().unwrap_or("Kugou User"),
                    "avatar": user["head_pic"].as_str().unwrap_or("")
                }
            }))
        } else {
            Ok(json!({"code": 301, "message": "Not logged in"}))
        }
    }

    async fn user_playlists(&self, _uid: i64) -> Result<Value, String> {
        let resp = self
            .api_get("https://my.kugou.com/v2/get_list.php?pagesize=50&page=1")
            .await?;
        let list = resp["data"]["list"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|pl| {
                json!({
                    "id": pl["listid"].as_i64().unwrap_or(0),
                    "name": pl["listname"].as_str().unwrap_or(""),
                    "cover_img_url": pl["headimg"].as_str().unwrap_or(""),
                    "track_count": pl["song_count"].as_i64().unwrap_or(0),
                    "creator": ""
                })
            })
            .collect::<Vec<_>>();
        Ok(json!({"playlist": list}))
    }

    async fn playlist_detail(&self, id: i64) -> Result<Value, String> {
        let url = format!(
            "https://my.kugou.com/v2/get_list_song.php?listid={}&pagesize=200&page=1",
            id
        );
        let resp = self.api_get(&url).await?;
        let songs = resp["data"]["list"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|s| {
                let hash = s["hash"].as_str().unwrap_or("").to_string();
                let artists = s["singer_name"].as_str().unwrap_or("").to_string();
                json!({
                    "id": if hash.is_empty() { s["song_id"].as_str().unwrap_or("").to_string() } else { hash },
                    "name": s["song_name"].as_str().unwrap_or(""),
                    "artists": artists,
                    "album": s["album_name"].as_str().unwrap_or(""),
                    "album_id": 0,
                    "duration": s["duration"].as_i64().unwrap_or(0) * 1000,
                    "pic_url": s["cover"].as_str().unwrap_or("")
                })
            })
            .collect::<Vec<_>>();
        Ok(json!({"songs": songs}))
    }

    async fn song_url(&self, id: &str) -> Result<String, String> {
        let url = format!(
            "https://wwwapi.kugou.com/yy/index.php?r=play/getdata&hash={}&album_id=0",
            id
        );
        let resp = self.api_get(&url).await?;
        let play_url = resp["data"]["play_url"].as_str().unwrap_or("").to_string();
        if play_url.is_empty() {
            return Err("Song URL not available".to_string());
        }
        Ok(play_url)
    }

    async fn lyric(&self, id: &str) -> Result<Value, String> {
        // First get song info for the lyric hash
        let url = format!(
            "https://wwwapi.kugou.com/yy/index.php?r=play/getdata&hash={}&album_id=0",
            id
        );
        let resp = self.api_get(&url).await?;
        let lyrics = resp["data"]["lyrics"].as_str().unwrap_or("").to_string();
        Ok(json!({"lrc": {"lyric": lyrics}}))
    }

    async fn search(&self, kw: &str, limit: i64) -> Result<Value, String> {
        let url = format!(
            "https://songsearch.kugou.com/song_search_v2?keyword={}&page=1&pagesize={}&platform=WebFilter",
            urlencoding(kw),
            limit
        );
        let resp = self.api_get(&url).await?;
        let songs = resp["data"]["lists"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|s| {
                let hash = s["FileHash"].as_str().unwrap_or("").to_string();
                let artists = s["SingerName"].as_str().unwrap_or("").to_string();
                // Kugou returns HTML-encoded strings, decode common entities
                let artists = artists
                    .replace("&amp;", "&")
                    .replace("&#39;", "'")
                    .replace("&quot;", "\"")
                    .replace("&lt;", "<")
                    .replace("&gt;", ">");
                let name = s["SongName"]
                    .as_str()
                    .unwrap_or("")
                    .replace("&amp;", "&")
                    .replace("&#39;", "'")
                    .replace("&quot;", "\"");
                let album = s["AlbumName"]
                    .as_str()
                    .unwrap_or("")
                    .replace("&amp;", "&")
                    .replace("&#39;", "'");
                let duration = s["Duration"].as_i64().unwrap_or(0) * 1000;
                json!({
                    "id": hash,
                    "name": name,
                    "artists": artists,
                    "album": album,
                    "album_id": 0,
                    "duration": duration,
                    "pic_url": ""
                })
            })
            .collect::<Vec<_>>();
        Ok(json!({"result": {"songs": songs}}))
    }

    async fn personalized(&self, _limit: i64) -> Result<Value, String> {
        let resp = self
            .api_get("https://everydayrec.kugou.com/recommendList/v1?apiver=2&plat=0")
            .await?;
        let list = resp["data"]["list"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .take(6)
            .map(|pl| {
                json!({
                    "id": pl["listid"].as_i64().unwrap_or(0),
                    "name": pl["listname"].as_str().unwrap_or(""),
                    "cover_img_url": pl["headimg"].as_str().unwrap_or(""),
                    "track_count": 0,
                    "creator": ""
                })
            })
            .collect::<Vec<_>>();
        Ok(json!({"result": list}))
    }

    async fn recommend_songs(&self) -> Result<Value, String> {
        let resp = self
            .api_get("https://everydayrec.kugou.com/everydayrec/v1?apiver=2&plat=0")
            .await?;
        let songs = resp["data"]["song_list"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|s| {
                let hash = s["hash"].as_str().unwrap_or("").to_string();
                let artists = s["singer_name"]
                    .as_str()
                    .unwrap_or("")
                    .replace("&amp;", "&")
                    .replace("&#39;", "'");
                json!({
                    "id": hash,
                    "name": s["song_name"].as_str().unwrap_or("").replace("&amp;", "&").replace("&#39;", "'"),
                    "artists": artists,
                    "album": s["album_name"].as_str().unwrap_or("").replace("&amp;", "&"),
                    "album_id": 0,
                    "duration": s["duration"].as_i64().unwrap_or(0) * 1000,
                    "pic_url": ""
                })
            })
            .collect::<Vec<_>>();
        Ok(json!({"data": {"daily_songs": songs}}))
    }

    async fn hot_searches(&self) -> Result<Value, String> {
        let resp = self
            .api_get("https://mobileservice.kugou.com/api/v3/search/hot?plat=0&count=20")
            .await?;
        let list = resp["data"]["info"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|h| {
                json!({
                    "search_word": h["keyword"].as_str().unwrap_or(""),
                    "content": "",
                    "icon_type": 0
                })
            })
            .collect::<Vec<_>>();
        Ok(json!({"result": {"hots": list}}))
    }

    async fn logout(&self) -> Result<(), String> {
        self.update_cookie(String::new())?;
        Ok(())
    }
}

fn urlencoding(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for byte in s.as_bytes() {
        if byte.is_ascii_alphanumeric() || matches!(*byte, b'-' | b'_' | b'.' | b'~') {
            result.push(*byte as char);
        } else {
            result.push_str(&format!("%{:02X}", byte));
        }
    }
    result
}
