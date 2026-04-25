use rodio::Source;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::Emitter;

const SAMPLE_BUF_SIZE: usize = 2048;
const NUM_BANDS: usize = 32;
const WAVEFORM_SIZE: usize = 128;

enum AudioMsg {
    Play(String),
    PlayBytes(Vec<u8>),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
    Seek(Duration),
}

#[derive(Clone, serde::Serialize)]
pub struct SpectrumData {
    pub bands: Vec<f32>,
    pub bass: f32,
    pub mid: f32,
    pub treble: f32,
    pub energy: f32,
    pub beat: bool,
    pub waveform: Vec<f32>,
}

pub struct AudioState {
    tx: Mutex<Option<Sender<AudioMsg>>>,
    #[allow(dead_code)]
    sample_buf: Arc<Mutex<Vec<f32>>>,
    spectrum: Arc<Mutex<SpectrumData>>,
}

impl AudioState {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        let (tx, rx) = channel::<AudioMsg>();
        let tx_clone = tx.clone();
        let sample_buf = Arc::new(Mutex::new(Vec::with_capacity(SAMPLE_BUF_SIZE)));
        let spectrum = Arc::new(Mutex::new(SpectrumData {
            bands: vec![0.0; NUM_BANDS],
            bass: 0.0,
            mid: 0.0,
            treble: 0.0,
            energy: 0.0,
            beat: false,
            waveform: vec![0.0; WAVEFORM_SIZE],
        }));
        let buf_for_thread = sample_buf.clone();
        let spec_for_thread = spectrum.clone();

        // Beat detection state
        let prev_energy: Arc<Mutex<f32>> = Arc::new(Mutex::new(0.0));
        let energy_hist: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
        let prev_for_beat = prev_energy.clone();
        let hist_for_beat = energy_hist.clone();

        std::thread::spawn(move || {
            let mut _stream: Option<rodio::OutputStream> = None;
            let mut _handle: Option<rodio::OutputStreamHandle> = None;
            let mut sink: Option<rodio::Sink> = None;

            let http_client = reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| reqwest::blocking::Client::new());

            loop {
                match rx.recv_timeout(Duration::from_millis(50)) {
                    Ok(msg) => match msg {
                        AudioMsg::Play(url) => {
                            if let Some(s) = sink.take() { s.stop(); }
                            _stream.take(); _handle.take();
                            let tx_c = tx_clone.clone();
                            if url.starts_with("http://") || url.starts_with("https://") {
                                let client = http_client.clone();
                                let emit_h = app_handle.clone();
                                std::thread::spawn(move || {
                                    let bytes = client.get(&url).send().ok()
                                        .and_then(|r| r.bytes().ok()).map(|b| b.to_vec());
                                    if let Some(data) = bytes {
                                        let _ = tx_c.send(AudioMsg::PlayBytes(data));
                                    } else {
                                        let _ = emit_h.emit("audio-state-changed",
                                            serde_json::json!({"playing": false, "reason": "error"}));
                                    }
                                });
                            } else if let Ok(file) = std::fs::File::open(&url) {
                                if let Ok(source) = rodio::Decoder::new(std::io::BufReader::new(file)) {
                                    if let Ok((s, h)) = rodio::OutputStream::try_default() {
                                        if let Ok(new_sink) = rodio::Sink::try_new(&h) {
                                            new_sink.append(SampleTap::new(source.convert_samples::<f32>(), buf_for_thread.clone()));
                                            new_sink.play();
                                            sink = Some(new_sink); _handle = Some(h); _stream = Some(s);
                                            let _ = app_handle.emit("audio-state-changed", serde_json::json!({"playing": true}));
                                        }
                                    }
                                }
                            }
                        }
                        AudioMsg::PlayBytes(data) => {
                            if let Ok((s, h)) = rodio::OutputStream::try_default() {
                                if let Ok(new_sink) = rodio::Sink::try_new(&h) {
                                    if let Ok(source) = rodio::Decoder::new(std::io::Cursor::new(data)) {
                                        new_sink.append(SampleTap::new(source.convert_samples::<f32>(), buf_for_thread.clone()));
                                        new_sink.play();
                                        sink = Some(new_sink); _handle = Some(h); _stream = Some(s);
                                        let _ = app_handle.emit("audio-state-changed", serde_json::json!({"playing": true}));
                                    } else {
                                        drop(new_sink); drop(h); drop(s);
                                        let _ = app_handle.emit("audio-state-changed", serde_json::json!({"playing": false, "error": "decode"}));
                                    }
                                }
                            }
                        }
                        AudioMsg::Pause => {
                            if let Some(s) = sink.as_ref() { s.pause(); }
                            let _ = app_handle.emit("audio-state-changed", serde_json::json!({"playing": false, "reason": "paused"}));
                        }
                        AudioMsg::Resume => {
                            if let Some(s) = sink.as_ref() { s.play(); }
                            let _ = app_handle.emit("audio-state-changed", serde_json::json!({"playing": true}));
                        }
                        AudioMsg::Stop => {
                            if let Some(s) = sink.take() { s.stop(); }
                            _stream.take(); _handle.take();
                            if let Ok(mut b) = buf_for_thread.lock() { b.clear(); }
                            if let Ok(mut sp) = spec_for_thread.lock() {
                                sp.bands.fill(0.0); sp.bass = 0.0; sp.mid = 0.0;
                                sp.treble = 0.0; sp.energy = 0.0; sp.beat = false;
                                sp.waveform.fill(0.0);
                            }
                            let _ = app_handle.emit("audio-state-changed", serde_json::json!({"playing": false, "reason": "stopped"}));
                        }
                        AudioMsg::SetVolume(vol) => {
                            if let Some(s) = sink.as_ref() { s.set_volume(vol.clamp(0.0, 2.0)); }
                        }
                        AudioMsg::Seek(pos) => {
                            if let Some(s) = sink.as_ref() { let _ = s.try_seek(pos); }
                        }
                    },
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        if let Some(ref s) = sink {
                            if s.empty() {
                                sink.take(); _stream.take(); _handle.take();
                                if let Ok(mut b) = buf_for_thread.lock() { b.clear(); }
                                if let Ok(mut sp) = spec_for_thread.lock() {
                                    sp.bands.fill(0.0); sp.bass = 0.0; sp.mid = 0.0;
                                    sp.treble = 0.0; sp.energy = 0.0; sp.beat = false;
                                    sp.waveform.fill(0.0);
                                }
                                let _ = app_handle.emit("audio-state-changed", serde_json::json!({"playing": false, "reason": "ended"}));
                            }
                        }
                        // Compute spectrum
                        if let Ok(buf) = buf_for_thread.lock() {
                            let (bands, bass, mid, treble, energy) = compute_bands(&buf, NUM_BANDS);
                            let waveform = extract_waveform(&buf, WAVEFORM_SIZE);
                            // Beat detection
                            let beat = detect_beat(energy, &prev_for_beat, &hist_for_beat);
                            if let Ok(mut sp) = spec_for_thread.lock() {
                                sp.bands = bands;
                                sp.bass = bass;
                                sp.mid = mid;
                                sp.treble = treble;
                                sp.energy = energy;
                                sp.beat = beat;
                                sp.waveform = waveform;
                            }
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        });

        Self { tx: Mutex::new(Some(tx)), sample_buf, spectrum }
    }

    pub fn get_spectrum(&self) -> SpectrumData {
        match self.spectrum.lock() {
            Ok(sp) => sp.clone(),
            Err(_) => SpectrumData {
                bands: vec![0.0; NUM_BANDS], bass: 0.0, mid: 0.0, treble: 0.0,
                energy: 0.0, beat: false, waveform: vec![0.0; WAVEFORM_SIZE],
            },
        }
    }
}

fn compute_bands(samples: &[f32], num_bands: usize) -> (Vec<f32>, f32, f32, f32, f32) {
    let n = samples.len();
    if n < 64 {
        return (vec![0.0; num_bands], 0.0, 0.0, 0.0, 0.0);
    }

    let mut bands = vec![0.0f32; num_bands];
    let half = n / 2;
    let band_size = (half / num_bands).max(1);
    let time_step = (n / 256).max(1);
    let total_energy;

    // Compute each band's magnitude
    for b in 0..num_bands {
        let start = b * band_size;
        let end = if b == num_bands - 1 { half } else { start + band_size };
        let step = ((end - start) / 6).max(1);
        let mut sum = 0.0f32;
        let mut count = 0u32;

        for k in (start..end).step_by(step) {
            let mut re = 0.0f32;
            let mut im = 0.0f32;
            for t in (0..n).step_by(time_step) {
                let angle = 2.0 * std::f32::consts::PI * (k as f32) * (t as f32) / (n as f32);
                re += samples[t] * angle.cos();
                im -= samples[t] * angle.sin();
            }
            sum += (re * re + im * im).sqrt();
            count += 1;
        }
        bands[b] = if count > 0 { (sum / count as f32).min(1.0) } else { 0.0 };
    }

    // Bass: first 4 bands (low freq), Mid: bands 4-16, Treble: bands 16+
    let bass_count = (num_bands as f32 * 0.15) as usize;
    let mid_end = (num_bands as f32 * 0.55) as usize;
    let bass: f32 = bands[..bass_count.max(1)].iter().sum::<f32>() / bass_count.max(1) as f32;
    let mid: f32 = bands[bass_count.max(1)..mid_end].iter().sum::<f32>() / (mid_end - bass_count.max(1)).max(1) as f32;
    let treble: f32 = bands[mid_end..].iter().sum::<f32>() / (num_bands - mid_end).max(1) as f32;
    total_energy = bands.iter().sum::<f32>() / num_bands as f32;

    (bands, bass, mid, treble, total_energy)
}

fn extract_waveform(samples: &[f32], size: usize) -> Vec<f32> {
    let n = samples.len();
    if n < size {
        return vec![0.0; size];
    }
    let step = n / size;
    (0..size).map(|i| samples[i * step]).collect()
}

fn detect_beat(energy: f32, prev: &Arc<Mutex<f32>>, hist: &Arc<Mutex<Vec<f32>>>) -> bool {
    let prev_e = *prev.lock().unwrap_or_else(|e| e.into_inner());
    *prev.lock().unwrap_or_else(|e| e.into_inner()) = energy;

    let mut h = hist.lock().unwrap_or_else(|e| e.into_inner());
    h.push(energy);
    if h.len() > 43 { h.remove(0); } // ~2s at 50ms intervals

    let avg = h.iter().sum::<f32>() / h.len().max(1) as f32;
    let threshold = avg * 1.4 + 0.02;
    energy > threshold && energy > prev_e && energy > 0.05
}

// SampleTap: captures samples for analysis
struct SampleTap<S> { inner: S, buf: Arc<Mutex<Vec<f32>>> }
impl<S> SampleTap<S> {
    fn new(inner: S, buf: Arc<Mutex<Vec<f32>>>) -> Self { Self { inner, buf } }
}
impl<S: Source<Item = f32>> Iterator for SampleTap<S> {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.inner.next()?;
        if let Ok(mut buf) = self.buf.lock() {
            buf.push(sample);
            let excess = buf.len().saturating_sub(SAMPLE_BUF_SIZE);
            if excess > 0 { buf.drain(0..excess); }
        }
        Some(sample)
    }
}
impl<S: Source<Item = f32>> Source for SampleTap<S> {
    fn current_frame_len(&self) -> Option<usize> { self.inner.current_frame_len() }
    fn channels(&self) -> u16 { self.inner.channels() }
    fn sample_rate(&self) -> u32 { self.inner.sample_rate() }
    fn total_duration(&self) -> Option<Duration> { self.inner.total_duration() }
}

pub struct AudioPlayer;
impl AudioPlayer {
    pub fn play(state: &AudioState, url: &str) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() { tx.send(AudioMsg::Play(url.to_string())).map_err(|e| e.to_string())?; }
        Ok(())
    }
    pub fn pause(state: &AudioState) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() { tx.send(AudioMsg::Pause).map_err(|e| e.to_string())?; }
        Ok(())
    }
    pub fn resume(state: &AudioState) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() { tx.send(AudioMsg::Resume).map_err(|e| e.to_string())?; }
        Ok(())
    }
    pub fn stop(state: &AudioState) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() { tx.send(AudioMsg::Stop).map_err(|e| e.to_string())?; }
        Ok(())
    }
    pub fn set_volume(state: &AudioState, vol: f32) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() { tx.send(AudioMsg::SetVolume(vol)).map_err(|e| e.to_string())?; }
        Ok(())
    }
    pub fn seek(state: &AudioState, pos: Duration) -> Result<(), String> {
        let tx = state.tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = tx.as_ref() { tx.send(AudioMsg::Seek(pos)).map_err(|e| e.to_string())?; }
        Ok(())
    }
}
