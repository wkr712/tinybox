use super::load_cookie;
use super::persist_cookie;
use super::MusicProvider;
use async_trait::async_trait;
use ncm_api_rs::{create_client, ApiClient, Query};
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Mutex;

struct Inner {
    client: ApiClient,
    cookie: String,
}

pub struct NcmProvider {
    inner: Mutex<Inner>,
    data_dir: Option<PathBuf>,
}

impl NcmProvider {
    pub fn new(data_dir: Option<PathBuf>) -> Self {
        let cookie = load_cookie(&data_dir, "ncm_cookie.txt").unwrap_or_default();
        let client = if cookie.is_empty() {
            create_client(None)
        } else {
            create_client(Some(cookie.clone()))
        };
        Self {
            inner: Mutex::new(Inner { client, cookie }),
            data_dir,
        }
    }

    fn get_client_and_cookie(&self) -> Result<(ApiClient, String), String> {
        let guard = self.inner.lock().map_err(|e| e.to_string())?;
        Ok((guard.client.clone(), guard.cookie.clone()))
    }

    fn update_cookie(&self, cookie_str: String) -> Result<(), String> {
        let mut guard = self.inner.lock().map_err(|e| e.to_string())?;
        guard.cookie = cookie_str.clone();
        guard.client = create_client(Some(cookie_str.clone()));
        drop(guard);
        persist_cookie(&self.data_dir, "ncm_cookie.txt", &cookie_str);
        Ok(())
    }
}

#[async_trait]
impl MusicProvider for NcmProvider {
    fn kind(&self) -> &str {
        "ncm"
    }

    async fn qr_generate(&self) -> Result<(String, String), String> {
        let (client, _) = self.get_client_and_cookie()?;
        let query = Query::new();
        let resp = client
            .login_qr_key(&query)
            .await
            .map_err(|e| e.to_string())?;
        let key = resp.body["unikey"]
            .as_str()
            .ok_or("Failed to get QR key")?
            .to_string();

        let query = Query::new().param("key", &key);
        let resp = client
            .login_qr_create(&query)
            .await
            .map_err(|e| e.to_string())?;
        let qrurl = resp.body["data"]["qrurl"]
            .as_str()
            .ok_or("Failed to get QR url")?
            .to_string();

        Ok((key, qrurl))
    }

    async fn qr_check(&self, key: &str) -> Result<Value, String> {
        let (client, _) = self.get_client_and_cookie()?;
        let query = Query::new().param("key", key);
        let resp = client
            .login_qr_check(&query)
            .await
            .map_err(|e| e.to_string())?;

        if resp.body["code"].as_i64() == Some(803) {
            let cookie_str = resp.cookie.join("; ");
            if !cookie_str.is_empty() {
                self.update_cookie(cookie_str)?;
            }
        }

        Ok(resp.body)
    }

    async fn login_status(&self) -> Result<Value, String> {
        let (client, cookie) = self.get_client_and_cookie()?;
        let query = Query::new().cookie(&cookie);
        let resp = client
            .login_status(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    async fn user_playlists(&self, uid: i64) -> Result<Value, String> {
        let (client, cookie) = self.get_client_and_cookie()?;
        let query = Query::new().param("uid", &uid.to_string()).cookie(&cookie);
        let resp = client
            .user_playlist(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    async fn playlist_detail(&self, id: i64) -> Result<Value, String> {
        let (client, cookie) = self.get_client_and_cookie()?;
        let query = Query::new().param("id", &id.to_string()).cookie(&cookie);
        let resp = client
            .playlist_detail(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    async fn song_url(&self, id: &str) -> Result<String, String> {
        let (client, cookie) = self.get_client_and_cookie()?;
        let query = Query::new()
            .param("id", id)
            .param("level", "standard")
            .cookie(&cookie);
        let resp = client
            .song_url_v1(&query)
            .await
            .map_err(|e| e.to_string())?;
        let url = resp.body["data"][0]["url"]
            .as_str()
            .ok_or("Song URL not available (may require VIP or region-restricted)")?;
        if url.is_empty() {
            return Err("Song URL not available (may require VIP)".to_string());
        }
        Ok(url.to_string())
    }

    async fn lyric(&self, id: &str) -> Result<Value, String> {
        let (client, cookie) = self.get_client_and_cookie()?;
        let query = Query::new().param("id", id).cookie(&cookie);
        let resp = client.lyric(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    async fn search(&self, kw: &str, limit: i64) -> Result<Value, String> {
        let (client, cookie) = self.get_client_and_cookie()?;
        let query = Query::new()
            .param("keywords", kw)
            .param("type", "1")
            .param("limit", &limit.to_string())
            .cookie(&cookie);
        let resp = client
            .cloudsearch(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    async fn personalized(&self, limit: i64) -> Result<Value, String> {
        let (client, cookie) = self.get_client_and_cookie()?;
        let query = Query::new()
            .param("limit", &limit.to_string())
            .cookie(&cookie);
        let resp = client
            .personalized(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    async fn recommend_songs(&self) -> Result<Value, String> {
        let (client, cookie) = self.get_client_and_cookie()?;
        let query = Query::new().cookie(&cookie);
        let resp = client
            .recommend_songs(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    async fn hot_searches(&self) -> Result<Value, String> {
        let (client, cookie) = self.get_client_and_cookie()?;
        let query = Query::new().cookie(&cookie);
        let resp = client
            .search_hot_detail(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    async fn logout(&self) -> Result<(), String> {
        self.update_cookie(String::new())?;
        Ok(())
    }
}
