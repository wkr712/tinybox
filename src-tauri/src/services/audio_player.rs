use rodio::Source;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;

enum AudioMsg {
    Play(String),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
}

pub struct AudioState {
    tx: Mutex<Option<Sender<AudioMsg>>>,
    pub current_url: Mutex<String>,
    pub is_playing: Mutex<bool>,
}

impl AudioState {
    pub fn new() -> Self {
        let (tx, rx) = channel::<AudioMsg>();

        std::thread::spawn(move || {
            let mut _stream: Option<rodio::OutputStream> = None;
            let mut _handle: Option<rodio::OutputStreamHandle> = None;
            let mut sink: Option<rodio::Sink> = None;

            while let Ok(msg) = rx.recv() {
                match msg {
                    AudioMsg::Play(url) => {
                        // Stop previous
                        if let Some(s) = sink.take() {
                            s.stop();
                        }
                        _stream.take();
                        _handle.take();

                        if let Ok((s, h)) = rodio::OutputStream::try_default() {
                            if let Ok(new_sink) = rodio::Sink::try_new(&h) {
                                let mut ok = false;
                                if url.starts_with("http://") || url.starts_with("https://") {
                                    if let Some(bytes) = reqwest::blocking::get(&url)
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
                                } else {
                                    drop(new_sink);
                                    drop(h);
                                    drop(s);
                                }
                            } else {
                                drop(s);
                            }
                        }
                    }
                    AudioMsg::Pause => {
                        if let Some(s) = sink.as_ref() {
                            s.pause();
                        }
                    }
                    AudioMsg::Resume => {
                        if let Some(s) = sink.as_ref() {
                            s.play();
                        }
                    }
                    AudioMsg::Stop => {
                        if let Some(s) = sink.take() {
                            s.stop();
                        }
                        _stream.take();
                        _handle.take();
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
            current_url: Mutex::new(String::new()),
            is_playing: Mutex::new(false),
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
        *state.current_url.lock().map_err(|e| e.to_string())? = url.to_string();
        *state.is_playing.lock().map_err(|e| e.to_string())? = true;
        Ok(())
    }

    pub fn pause(state: &AudioState) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() {
            tx.send(AudioMsg::Pause).map_err(|e| e.to_string())?;
        }
        *state.is_playing.lock().map_err(|e| e.to_string())? = false;
        Ok(())
    }

    pub fn resume(state: &AudioState) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() {
            tx.send(AudioMsg::Resume).map_err(|e| e.to_string())?;
        }
        *state.is_playing.lock().map_err(|e| e.to_string())? = true;
        Ok(())
    }

    pub fn stop(state: &AudioState) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() {
            tx.send(AudioMsg::Stop).map_err(|e| e.to_string())?;
        }
        *state.current_url.lock().map_err(|e| e.to_string())? = String::new();
        *state.is_playing.lock().map_err(|e| e.to_string())? = false;
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
