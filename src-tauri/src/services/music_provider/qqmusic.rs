use super::load_cookie;
use super::persist_cookie;
use super::MusicProvider;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Mutex;

const BASE_URL: &str = "https://u.y.qq.com/cgi-bin/musicu.fcg";

struct Inner {
    cookie: String,
    client: reqwest::Client,
}

pub struct QqMusicProvider {
    inner: Mutex<Inner>,
    data_dir: Option<PathBuf>,
}

impl QqMusicProvider {
    pub fn new(data_dir: Option<PathBuf>) -> Self {
        let cookie = load_cookie(&data_dir, "qqmusic_cookie.txt").unwrap_or_default();
        Self {
            inner: Mutex::new(Inner {
                cookie,
                client: reqwest::Client::new(),
            }),
            data_dir,
        }
    }

    fn get_cookie(&self) -> Result<String, String> {
        Ok(self.inner.lock().map_err(|e| e.to_string())?.cookie.clone())
    }

    fn update_cookie(&self, cookie_str: String) -> Result<(), String> {
        let mut guard = self.inner.lock().map_err(|e| e.to_string())?;
        guard.cookie = cookie_str.clone();
        drop(guard);
        persist_cookie(&self.data_dir, "qqmusic_cookie.txt", &cookie_str);
        Ok(())
    }

    async fn api_request(&self, body: &Value) -> Result<Value, String> {
        let cookie = self.get_cookie()?;
        let client = self.inner.lock().map_err(|e| e.to_string())?.client.clone();

        let mut req = client
            .post(BASE_URL)
            .header("Content-Type", "application/json")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36");

        if !cookie.is_empty() {
            req = req.header("Cookie", &cookie);
        }

        let resp = req
            .json(body)
            .send()
            .await
            .map_err(|e| format!("QQ Music request failed: {}", e))?;

        // Extract cookies from response headers
        if let Ok(resp_cookie) = std::str::from_utf8(
            resp.headers()
                .get_all("set-cookie")
                .iter()
                .flat_map(|v| v.to_str().ok())
                .collect::<Vec<_>>()
                .join("; ")
                .as_bytes(),
        ) {
            if !resp_cookie.is_empty() {
                let existing = self.get_cookie().unwrap_or_default();
                let merged = merge_cookies(&existing, resp_cookie);
                let _ = self.update_cookie(merged);
            }
        }

        resp.json::<Value>()
            .await
            .map_err(|e| format!("QQ Music parse failed: {}", e))
    }

    fn extract_qq_music_id(songmid: &str, songid: i64) -> String {
        if !songmid.is_empty() {
            songmid.to_string()
        } else {
            songid.to_string()
        }
    }
}

fn merge_cookies(existing: &str, new: &str) -> String {
    let mut map = std::collections::HashMap::new();
    for part in existing.split(';').chain(new.split(';')) {
        let part = part.trim();
        if let Some((k, v)) = part.split_once('=') {
            map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    map.into_iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("; ")
}

#[async_trait]
impl MusicProvider for QqMusicProvider {
    fn kind(&self) -> &str {
        "qqmusic"
    }

    async fn qr_generate(&self) -> Result<(String, String), String> {
        let body = json!({
            "music.client.QRCodeLogin.Server": {
                "module": "music.client.QRCodeLogin.Server",
                "method": "GetQQLoginQRCode",
                "param": {}
            }
        });
        let resp = self.api_request(&body).await?;
        let data = resp
            .get("music.client.QRCodeLogin.Server")
            .ok_or("Failed to get QQ Music QR code")?;
        let token = data["data"]["token"]
            .as_str()
            .ok_or("No QR token")?
            .to_string();
        let qrcode = data["data"]["qrcode"]
            .as_str()
            .ok_or("No QR code image")?
            .to_string();
        Ok((token, format!("https://qrcode.qq.com/{}", qrcode)))
    }

    async fn qr_check(&self, key: &str) -> Result<Value, String> {
        let body = json!({
            "music.client.QRCodeLogin.Server": {
                "module": "music.client.QRCodeLogin.Server",
                "method": "CheckQQLoginQRCode",
                "param": {
                    "token": key
                }
            }
        });
        self.api_request(&body).await
    }

    async fn login_status(&self) -> Result<Value, String> {
        let cookie = self.get_cookie()?;
        if cookie.is_empty() {
            return Ok(json!({"code": 301, "message": "Not logged in"}));
        }
        let body = json!({
            "music.UnifiedHomepage.UnifiedHomepageSvr": {
                "module": "music.UnifiedHomepage.UnifiedHomepageSvr",
                "method": "GetHomepageHeader",
                "param": {}
            }
        });
        let resp = self.api_request(&body).await?;
        let data = resp
            .get("music.UnifiedHomepage.UnifiedHomepageSvr")
            .cloned()
            .unwrap_or(json!({}));
        if data["code"].as_i64() == Some(0) {
            let nick = data["data"]["nick"]
                .as_str()
                .unwrap_or("QQ Music User")
                .to_string();
            let avatar = data["data"]["headpic"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let uid = data["data"]["uin"]
                .as_i64()
                .unwrap_or(0);
            Ok(json!({
                "code": 200,
                "account": {
                    "id": uid,
                    "userName": nick,
                    "avatar": avatar,
                }
            }))
        } else {
            Ok(json!({"code": 301, "message": "Not logged in"}))
        }
    }

    async fn user_playlists(&self, uid: i64) -> Result<Value, String> {
        let body = json!({
            "music.srfDissInfo.DissInfo": {
                "module": "music.srfDissInfo.DissInfo",
                "method": "CgiGetUserOwnDiss",
                "param": {
                    "uin": uid,
                    "start": 0,
                    "num": 50
                }
            }
        });
        let resp = self.api_request(&body).await?;
        let data = resp
            .get("music.srfDissInfo.DissInfo")
            .cloned()
            .unwrap_or(json!({}));
        let list = data["data"]["disslist"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|pl| {
                json!({
                    "id": pl["tid"].as_i64().unwrap_or(0),
                    "name": pl["title"].as_str().unwrap_or(""),
                    "cover_img_url": pl["picurl"].as_str().unwrap_or(""),
                    "track_count": pl["song_cnt"].as_i64().unwrap_or(0),
                    "creator": ""
                })
            })
            .collect::<Vec<_>>();
        Ok(json!({"playlist": list}))
    }

    async fn playlist_detail(&self, id: i64) -> Result<Value, String> {
        let body = json!({
            "music.srfDissInfo.DissInfo": {
                "module": "music.srfDissInfo.DissInfo",
                "method": "CgiGetDiss",
                "param": {
                    "disstid": id,
                    "onlysonglist": 0,
                    "song_begin": 0,
                    "song_num": 200
                }
            }
        });
        let resp = self.api_request(&body).await?;
        let data = resp
            .get("music.srfDissInfo.DissInfo")
            .cloned()
            .unwrap_or(json!({}));
        let songs = data["data"]["songlist"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|s| {
                let songmid = s["songmid"].as_str().unwrap_or("").to_string();
                let songid = s["songid"].as_i64().unwrap_or(0);
                let artists = s["singer"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|a| a["name"].as_str().map(|n| n.to_string()))
                            .collect::<Vec<_>>()
                            .join(" / ")
                    })
                    .unwrap_or_default();
                let album = s["albumname"].as_str().unwrap_or("").to_string();
                let duration = s["interval"].as_i64().unwrap_or(0) * 1000;
                json!({
                    "id": Self::extract_qq_music_id(&songmid, songid),
                    "name": s["songname"].as_str().unwrap_or(""),
                    "artists": artists,
                    "album": album,
                    "album_id": s["albumid"].as_i64().unwrap_or(0),
                    "duration": duration,
                    "pic_url": format!("https://y.gtimg.cn/music/photo_new/T002R300x300M000{}.jpg", s["albummid"].as_str().unwrap_or(""))
                })
            })
            .collect::<Vec<_>>();
        Ok(json!({"songs": songs}))
    }

    async fn song_url(&self, id: &str) -> Result<String, String> {
        let body = json!({
            "music.vkey.GetMusicVkey": {
                "module": "music.vkey.GetMusicVkey",
                "method": "CgiGetMusicVkey",
                "param": {
                    "songmid": id,
                    "songtype": 1,
                    "uin": "0",
                    "loginflag": 0,
                    "platform": "20"
                }
            }
        });
        let resp = self.api_request(&body).await?;
        let data = resp
            .get("music.vkey.GetMusicVkey")
            .ok_or("Failed to get song URL")?;
        let midurlinfo = data["data"]["midurlinfo"]
            .as_array()
            .and_then(|arr| arr.first())
            .ok_or("No midurlinfo")?;
        let purl = midurlinfo["purl"].as_str().unwrap_or("");
        let sip = data["data"]["sip"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|s| s.as_str())
            .unwrap_or("");
        if purl.is_empty() {
            return Err("Song URL not available (may require VIP)".to_string());
        }
        Ok(format!("{}{}", sip, purl))
    }

    async fn lyric(&self, id: &str) -> Result<Value, String> {
        let body = json!({
            "music.musichallSong.PlayLyricInfo": {
                "module": "music.musichallSong.PlayLyricInfo",
                "method": "GetPlayLyricInfo",
                "param": {
                    "songMID": id,
                    "songID": 0
                }
            }
        });
        let resp = self.api_request(&body).await?;
        let data = resp
            .get("music.musichallSong.PlayLyricInfo")
            .cloned()
            .unwrap_or(json!({}));
        let lyric_b64 = data["data"]["lyric"]
            .as_str()
            .unwrap_or("");
        let lyric = standard_base64_decode(lyric_b64);
        Ok(json!({"lrc": {"lyric": lyric}}))
    }

    async fn search(&self, kw: &str, limit: i64) -> Result<Value, String> {
        let body = json!({
            "music.search.SearchCgiService": {
                "module": "music.search.SearchCgiService",
                "method": "DoSearchForClass",
                "param": {
                    "query": kw,
                    "num_per_page": limit,
                    "page_num": 1,
                    "search_type": 0
                }
            }
        });
        let resp = self.api_request(&body).await?;
        let data = resp
            .get("music.search.SearchCgiService")
            .cloned()
            .unwrap_or(json!({}));
        let songs = data["data"]["body"]["song"]["list"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|s| {
                let songmid = s["songmid"].as_str().unwrap_or("").to_string();
                let songid = s["songid"].as_i64().unwrap_or(0);
                let artists = s["singer"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|a| a["name"].as_str().map(|n| n.to_string()))
                            .collect::<Vec<_>>()
                            .join(" / ")
                    })
                    .unwrap_or_default();
                let album = s["albumname"].as_str().unwrap_or("").to_string();
                let duration = s["interval"].as_i64().unwrap_or(0) * 1000;
                json!({
                    "id": Self::extract_qq_music_id(&songmid, songid),
                    "name": s["songname"].as_str().unwrap_or(""),
                    "artists": artists,
                    "album": album,
                    "album_id": s["albumid"].as_i64().unwrap_or(0),
                    "duration": duration,
                    "pic_url": format!("https://y.gtimg.cn/music/photo_new/T002R300x300M000{}.jpg", s["albummid"].as_str().unwrap_or(""))
                })
            })
            .collect::<Vec<_>>();
        Ok(json!({"result": {"songs": songs}}))
    }

    async fn personalized(&self, _limit: i64) -> Result<Value, String> {
        let body = json!({
            "music.recommend.RecommendFeed": {
                "module": "music.recommend.RecommendFeed",
                "method": "GetRecommendFeed",
                "param": {
                    "cmd": 6,
                    "qc_flag": 2
                }
            }
        });
        let resp = self.api_request(&body).await?;
        let data = resp
            .get("music.recommend.RecommendFeed")
            .cloned()
            .unwrap_or(json!({}));
        let list = data["data"]["track_list"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .take(6)
            .map(|pl| {
                json!({
                    "id": pl["tid"].as_i64().unwrap_or(0),
                    "name": pl["title"].as_str().unwrap_or(""),
                    "cover_img_url": pl["pic"].as_str().unwrap_or(""),
                    "track_count": 0,
                    "creator": ""
                })
            })
            .collect::<Vec<_>>();
        Ok(json!({"result": list}))
    }

    async fn recommend_songs(&self) -> Result<Value, String> {
        let body = json!({
            "music.recommend.RecommendSong": {
                "module": "music.recommend.RecommendSong",
                "method": "GetRecommendSong",
                "param": {}
            }
        });
        let resp = self.api_request(&body).await?;
        let data = resp
            .get("music.recommend.RecommendSong")
            .cloned()
            .unwrap_or(json!({}));
        let songs = data["data"]["track_list"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|s| {
                let songmid = s["songmid"].as_str().unwrap_or("").to_string();
                let songid = s["songid"].as_i64().unwrap_or(0);
                let artists = s["singer"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|a| a["name"].as_str().map(|n| n.to_string()))
                            .collect::<Vec<_>>()
                            .join(" / ")
                    })
                    .unwrap_or_default();
                json!({
                    "id": Self::extract_qq_music_id(&songmid, songid),
                    "name": s["songname"].as_str().unwrap_or(""),
                    "artists": artists,
                    "album": s["albumname"].as_str().unwrap_or(""),
                    "album_id": s["albumid"].as_i64().unwrap_or(0),
                    "duration": s["interval"].as_i64().unwrap_or(0) * 1000,
                    "pic_url": format!("https://y.gtimg.cn/music/photo_new/T002R300x300M000{}.jpg", s["albummid"].as_str().unwrap_or(""))
                })
            })
            .collect::<Vec<_>>();
        Ok(json!({"data": {"daily_songs": songs}}))
    }

    async fn hot_searches(&self) -> Result<Value, String> {
        let body = json!({
            "music.search.HotkeyService": {
                "module": "music.search.HotkeyService",
                "method": "GetHotkeyForQQ",
                "param": {}
            }
        });
        let resp = self.api_request(&body).await?;
        let data = resp
            .get("music.search.HotkeyService")
            .cloned()
            .unwrap_or(json!({}));
        let list = data["data"]["vec_hotkey"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|h| {
                json!({
                    "search_word": h["query"].as_str().unwrap_or(""),
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

fn standard_base64_decode(input: &str) -> String {
    // QQ Music uses URL-safe base64, convert to standard
    let standard = input.replace('-', "+").replace('_', "/");
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(standard)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .unwrap_or_default()
}
