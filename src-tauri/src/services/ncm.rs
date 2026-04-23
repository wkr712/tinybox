use ncm_api_rs::{create_client, ApiClient, Query};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

struct InnerState {
    client: ApiClient,
    cookie: String,
}

pub struct NcmState {
    inner: Mutex<InnerState>,
    data_dir: Mutex<Option<PathBuf>>,
}

impl NcmState {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(InnerState {
                client: create_client(None),
                cookie: String::new(),
            }),
            data_dir: Mutex::new(None),
        }
    }

    pub fn init_data_dir(&self, dir: PathBuf) {
        let cookie_path = dir.join("ncm_cookie.txt");
        if let Ok(cookie_str) = fs::read_to_string(&cookie_path) {
            if !cookie_str.trim().is_empty() {
                let cookie = cookie_str.trim().to_string();
                if let Ok(mut guard) = self.inner.lock() {
                    *guard = InnerState {
                        client: create_client(Some(cookie)),
                        cookie: cookie_str.trim().to_string(),
                    };
                }
            }
        }
        if let Ok(mut d) = self.data_dir.lock() {
            *d = Some(dir);
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
        self.persist_cookie(&cookie_str);
        Ok(())
    }

    fn persist_cookie(&self, cookie: &str) {
        if let Ok(guard) = self.data_dir.lock() {
            if let Some(ref dir) = *guard {
                let _ = fs::create_dir_all(dir);
                let _ = fs::write(dir.join("ncm_cookie.txt"), cookie);
            }
        }
    }
}

pub struct NcmService;

impl NcmService {
    pub async fn qr_key(state: &NcmState) -> Result<Value, String> {
        let (client, _) = state.get_client_and_cookie()?;
        let query = Query::new();
        let resp = client
            .login_qr_key(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn qr_create(state: &NcmState, key: &str) -> Result<Value, String> {
        let (client, _) = state.get_client_and_cookie()?;
        let query = Query::new().param("key", key);
        let resp = client
            .login_qr_create(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn qr_check(state: &NcmState, key: &str) -> Result<Value, String> {
        let (client, _) = state.get_client_and_cookie()?;
        let query = Query::new().param("key", key);
        let resp = client
            .login_qr_check(&query)
            .await
            .map_err(|e| e.to_string())?;

        if resp.body["code"].as_i64() == Some(803) {
            let cookie_str = resp.cookie.join("; ");
            if !cookie_str.is_empty() {
                state.update_cookie(cookie_str)?;
            }
        }

        Ok(resp.body)
    }

    pub async fn login_status(state: &NcmState) -> Result<Value, String> {
        let (client, cookie) = state.get_client_and_cookie()?;
        let query = Query::new().cookie(&cookie);
        let resp = client
            .login_status(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn user_playlist(state: &NcmState, uid: i64) -> Result<Value, String> {
        let (client, cookie) = state.get_client_and_cookie()?;
        let query = Query::new().param("uid", &uid.to_string()).cookie(&cookie);
        let resp = client
            .user_playlist(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn playlist_detail(state: &NcmState, id: i64) -> Result<Value, String> {
        let (client, cookie) = state.get_client_and_cookie()?;
        let query = Query::new().param("id", &id.to_string()).cookie(&cookie);
        let resp = client
            .playlist_detail(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn song_url(state: &NcmState, id: i64) -> Result<Value, String> {
        let (client, cookie) = state.get_client_and_cookie()?;
        let query = Query::new()
            .param("id", &id.to_string())
            .param("level", "standard")
            .cookie(&cookie);
        let resp = client
            .song_url_v1(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn song_detail(state: &NcmState, ids: &str) -> Result<Value, String> {
        let (client, cookie) = state.get_client_and_cookie()?;
        let query = Query::new().param("ids", ids).cookie(&cookie);
        let resp = client
            .song_detail(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn lyric(state: &NcmState, id: i64) -> Result<Value, String> {
        let (client, cookie) = state.get_client_and_cookie()?;
        let query = Query::new().param("id", &id.to_string()).cookie(&cookie);
        let resp = client.lyric(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn search(state: &NcmState, keywords: &str) -> Result<Value, String> {
        let (client, cookie) = state.get_client_and_cookie()?;
        let query = Query::new()
            .param("keywords", keywords)
            .param("type", "1")
            .param("limit", "30")
            .cookie(&cookie);
        let resp = client
            .cloudsearch(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn personalized(state: &NcmState, limit: i64) -> Result<Value, String> {
        let (client, cookie) = state.get_client_and_cookie()?;
        let query = Query::new()
            .param("limit", &limit.to_string())
            .cookie(&cookie);
        let resp = client
            .personalized(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn recommend_songs(state: &NcmState) -> Result<Value, String> {
        let (client, cookie) = state.get_client_and_cookie()?;
        let query = Query::new().cookie(&cookie);
        let resp = client
            .recommend_songs(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn search_hot_detail(state: &NcmState) -> Result<Value, String> {
        let (client, cookie) = state.get_client_and_cookie()?;
        let query = Query::new().cookie(&cookie);
        let resp = client
            .search_hot_detail(&query)
            .await
            .map_err(|e| e.to_string())?;
        Ok(resp.body)
    }
}
