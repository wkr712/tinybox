use rodio::Source;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::time::Duration;
use tauri::Emitter;

enum AudioMsg {
    Play(String),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
}

pub struct AudioState {
    tx: Mutex<Option<Sender<AudioMsg>>>,
}

impl AudioState {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        let (tx, rx) = channel::<AudioMsg>();

        std::thread::spawn(move || {
            let mut _stream: Option<rodio::OutputStream> = None;
            let mut _handle: Option<rodio::OutputStreamHandle> = None;
            let mut sink: Option<rodio::Sink> = None;

            let http_client = reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| reqwest::blocking::Client::new());

            while let Ok(msg) = rx.recv() {
                match msg {
                    AudioMsg::Play(url) => {
                        if let Some(s) = sink.take() {
                            s.stop();
                        }
                        _stream.take();
                        _handle.take();

                        if let Ok((s, h)) = rodio::OutputStream::try_default() {
                            if let Ok(new_sink) = rodio::Sink::try_new(&h) {
                                let mut ok = false;
                                if url.starts_with("http://") || url.starts_with("https://") {
                                    if let Some(bytes) = http_client
                                        .get(&url)
                                        .send()
                                        .ok()
                                        .and_then(|r| r.bytes().ok())
                                    {
                                        if let Ok(source) =
                                            rodio::Decoder::new(std::io::Cursor::new(bytes))
                                        {
                                            new_sink.append(source.convert_samples::<f32>());
                                            ok = true;
                                        }
                                    }
                                } else if let Ok(file) = std::fs::File::open(&url) {
                                    if let Ok(source) =
                                        rodio::Decoder::new(std::io::BufReader::new(file))
                                    {
                                        new_sink.append(source.convert_samples::<f32>());
                                        ok = true;
                                    }
                                }

                                if ok {
                                    new_sink.play();
                                    sink = Some(new_sink);
                                    _handle = Some(h);
                                    _stream = Some(s);
                                    let _ = app_handle.emit(
                                        "audio-state-changed",
                                        serde_json::json!({"playing": true}),
                                    );
                                } else {
                                    drop(new_sink);
                                    drop(h);
                                    drop(s);
                                    let _ = app_handle.emit(
                                        "audio-state-changed",
                                        serde_json::json!({"playing": false, "error": "failed to decode audio"}),
                                    );
                                }
                            } else {
                                drop(s);
                                let _ = app_handle.emit(
                                    "audio-state-changed",
                                    serde_json::json!({"playing": false, "error": "failed to create audio sink"}),
                                );
                            }
                        } else {
                            let _ = app_handle.emit(
                                "audio-state-changed",
                                serde_json::json!({"playing": false, "error": "no audio output device"}),
                            );
                        }
                    }
                    AudioMsg::Pause => {
                        if let Some(s) = sink.as_ref() {
                            s.pause();
                            let _ = app_handle
                                .emit("audio-state-changed", serde_json::json!({"playing": false}));
                        }
                    }
                    AudioMsg::Resume => {
                        if let Some(s) = sink.as_ref() {
                            s.play();
                            let _ = app_handle
                                .emit("audio-state-changed", serde_json::json!({"playing": true}));
                        }
                    }
                    AudioMsg::Stop => {
                        if let Some(s) = sink.take() {
                            s.stop();
                        }
                        _stream.take();
                        _handle.take();
                        let _ = app_handle
                            .emit("audio-state-changed", serde_json::json!({"playing": false}));
                    }
                    AudioMsg::SetVolume(vol) => {
                        if let Some(s) = sink.as_ref() {
                            s.set_volume(vol);
                        }
                    }
                }
            }
        });

        Self {
            tx: Mutex::new(Some(tx)),
        }
    }
}

pub struct AudioPlayer;

impl AudioPlayer {
    pub fn play(state: &AudioState, url: &str) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() {
            tx.send(AudioMsg::Play(url.to_string()))
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn pause(state: &AudioState) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() {
            tx.send(AudioMsg::Pause).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn resume(state: &AudioState) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() {
            tx.send(AudioMsg::Resume).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn stop(state: &AudioState) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() {
            tx.send(AudioMsg::Stop).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn set_volume(state: &AudioState, vol: f32) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() {
            tx.send(AudioMsg::SetVolume(vol))
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
