use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub mod kugou;
pub mod ncm;
pub mod qqmusic;

#[async_trait]
pub trait MusicProvider: Send + Sync {
    #[allow(dead_code)]
    fn kind(&self) -> &str;
    async fn qr_generate(&self) -> Result<(String, String), String>;
    async fn qr_check(&self, key: &str) -> Result<Value, String>;
    async fn login_status(&self) -> Result<Value, String>;
    async fn user_playlists(&self, uid: i64) -> Result<Value, String>;
    async fn playlist_detail(&self, id: i64) -> Result<Value, String>;
    async fn song_url(&self, id: &str) -> Result<String, String>;
    async fn lyric(&self, id: &str) -> Result<Value, String>;
    async fn search(&self, kw: &str, limit: i64) -> Result<Value, String>;
    async fn personalized(&self, limit: i64) -> Result<Value, String>;
    async fn recommend_songs(&self) -> Result<Value, String>;
    async fn hot_searches(&self) -> Result<Value, String>;
    async fn logout(&self) -> Result<(), String>;
}

pub struct ProviderRegistry {
    providers: HashMap<String, Box<dyn MusicProvider>>,
    active: Mutex<String>,
    data_dir: Mutex<Option<PathBuf>>,
}

impl ProviderRegistry {
    pub fn new(data_dir: PathBuf) -> Self {
        let mut reg = Self {
            providers: HashMap::new(),
            active: Mutex::new("ncm".to_string()),
            data_dir: Mutex::new(Some(data_dir)),
        };
        reg.register_default_providers();
        reg
    }

    fn register_default_providers(&mut self) {
        let data_dir = self
            .data_dir
            .lock()
            .ok()
            .and_then(|d| d.clone());

        if let Some(ref dir) = data_dir {
            let _ = fs::create_dir_all(dir);
        }

        self.providers.insert(
            "ncm".to_string(),
            Box::new(ncm::NcmProvider::new(data_dir.clone())),
        );
        self.providers.insert(
            "qqmusic".to_string(),
            Box::new(qqmusic::QqMusicProvider::new(data_dir.clone())),
        );
        self.providers.insert(
            "kugou".to_string(),
            Box::new(kugou::KugouProvider::new(data_dir)),
        );
    }

    pub fn active_provider(&self) -> Result<&dyn MusicProvider, String> {
        let active = self.active.lock().map_err(|e| e.to_string())?;
        self.providers
            .get(&*active)
            .map(|p| p.as_ref())
            .ok_or_else(|| format!("Provider '{}' not found", *active))
    }

    pub fn set_active(&self, kind: &str) -> Result<(), String> {
        if !self.providers.contains_key(kind) {
            return Err(format!("Unknown provider: {}", kind));
        }
        let mut active = self.active.lock().map_err(|e| e.to_string())?;
        *active = kind.to_string();
        Ok(())
    }

    pub fn get_active_kind(&self) -> Result<String, String> {
        Ok(self.active.lock().map_err(|e| e.to_string())?.clone())
    }
}

pub fn persist_cookie(data_dir: &Option<PathBuf>, filename: &str, cookie: &str) {
    if let Some(ref dir) = data_dir {
        let _ = fs::create_dir_all(dir);
        let _ = fs::write(dir.join(filename), cookie);
    }
}

pub fn load_cookie(data_dir: &Option<PathBuf>, filename: &str) -> Option<String> {
    data_dir.as_ref().and_then(|dir| {
        let path = dir.join(filename);
        fs::read_to_string(path).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
    })
}
