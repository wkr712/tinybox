use ncm_api_rs::{create_client, ApiClient, Query};
use serde_json::Value;
use std::sync::Mutex;

pub struct NcmState {
    pub client: Mutex<ApiClient>,
    pub cookie: Mutex<String>,
}

impl NcmState {
    pub fn new() -> Self {
        Self {
            client: Mutex::new(create_client(None)),
            cookie: Mutex::new(String::new()),
        }
    }

    fn get_client(&self) -> Result<ApiClient, String> {
        let guard = self.client.lock().map_err(|e| e.to_string())?;
        Ok(guard.clone())
    }

    fn get_cookie(&self) -> Result<String, String> {
        let guard = self.cookie.lock().map_err(|e| e.to_string())?;
        Ok(guard.clone())
    }

    fn update_cookie(&self, cookie_str: String) -> Result<(), String> {
        let mut stored = self.cookie.lock().map_err(|e| e.to_string())?;
        *stored = cookie_str.clone();
        let mut c = self.client.lock().map_err(|e| e.to_string())?;
        *c = create_client(Some(cookie_str));
        Ok(())
    }
}

pub struct NcmService;

impl NcmService {
    pub async fn qr_key(state: &NcmState) -> Result<Value, String> {
        let client = state.get_client()?;
        let query = Query::new();
        let resp = client.login_qr_key(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn qr_create(state: &NcmState, key: &str) -> Result<Value, String> {
        let client = state.get_client()?;
        let query = Query::new().param("key", key);
        let resp = client.login_qr_create(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn qr_check(state: &NcmState, key: &str) -> Result<Value, String> {
        let client = state.get_client()?;
        let query = Query::new().param("key", key);
        let resp = client.login_qr_check(&query).await.map_err(|e| e.to_string())?;

        if resp.body["code"].as_i64() == Some(803) {
            let cookie_str = resp.cookie.join("; ");
            if !cookie_str.is_empty() {
                state.update_cookie(cookie_str)?;
            }
        }

        Ok(resp.body)
    }

    pub async fn login_status(state: &NcmState) -> Result<Value, String> {
        let cookie = state.get_cookie()?;
        let client = state.get_client()?;
        let query = Query::new().cookie(&cookie);
        let resp = client.login_status(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn user_playlist(state: &NcmState, uid: i64) -> Result<Value, String> {
        let cookie = state.get_cookie()?;
        let client = state.get_client()?;
        let query = Query::new().param("uid", &uid.to_string()).cookie(&cookie);
        let resp = client.user_playlist(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn playlist_detail(state: &NcmState, id: i64) -> Result<Value, String> {
        let cookie = state.get_cookie()?;
        let client = state.get_client()?;
        let query = Query::new().param("id", &id.to_string()).cookie(&cookie);
        let resp = client.playlist_detail(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn song_url(state: &NcmState, id: i64) -> Result<Value, String> {
        let cookie = state.get_cookie()?;
        let client = state.get_client()?;
        let query = Query::new()
            .param("id", &id.to_string())
            .param("level", "standard")
            .cookie(&cookie);
        let resp = client.song_url_v1(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn song_detail(state: &NcmState, ids: &str) -> Result<Value, String> {
        let cookie = state.get_cookie()?;
        let client = state.get_client()?;
        let query = Query::new().param("ids", ids).cookie(&cookie);
        let resp = client.song_detail(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn lyric(state: &NcmState, id: i64) -> Result<Value, String> {
        let cookie = state.get_cookie()?;
        let client = state.get_client()?;
        let query = Query::new().param("id", &id.to_string()).cookie(&cookie);
        let resp = client.lyric(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn search(state: &NcmState, keywords: &str) -> Result<Value, String> {
        let cookie = state.get_cookie()?;
        let client = state.get_client()?;
        let query = Query::new()
            .param("keywords", keywords)
            .param("type", "1")
            .param("limit", "30")
            .cookie(&cookie);
        let resp = client.cloudsearch(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn personalized(state: &NcmState, limit: i64) -> Result<Value, String> {
        let cookie = state.get_cookie()?;
        let client = state.get_client()?;
        let query = Query::new().param("limit", &limit.to_string()).cookie(&cookie);
        let resp = client.personalized(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn personalized_newsong(state: &NcmState, limit: i64) -> Result<Value, String> {
        let cookie = state.get_cookie()?;
        let client = state.get_client()?;
        let query = Query::new().param("limit", &limit.to_string()).cookie(&cookie);
        let resp = client.personalized_newsong(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn recommend_songs(state: &NcmState) -> Result<Value, String> {
        let cookie = state.get_cookie()?;
        let client = state.get_client()?;
        let query = Query::new().cookie(&cookie);
        let resp = client.recommend_songs(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }

    pub async fn search_hot_detail(state: &NcmState) -> Result<Value, String> {
        let cookie = state.get_cookie()?;
        let client = state.get_client()?;
        let query = Query::new().cookie(&cookie);
        let resp = client.search_hot_detail(&query).await.map_err(|e| e.to_string())?;
        Ok(resp.body)
    }
}
