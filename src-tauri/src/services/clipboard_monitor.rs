use std::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::Emitter;

pub struct ClipboardMonitor {
    _stop: mpsc::Sender<()>,
    paused: Arc<AtomicBool>,
}

impl ClipboardMonitor {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        let (tx, rx) = mpsc::channel::<()>();
        let paused = Arc::new(AtomicBool::new(false));
        let paused_flag = paused.clone();

        std::thread::spawn(move || {
            let mut clipboard = match arboard::Clipboard::new() {
                Ok(c) => c,
                Err(_) => return,
            };
            let mut last_text = String::new();

            loop {
                match rx.try_recv() {
                    Ok(_) | Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
                    Err(std::sync::mpsc::TryRecvError::Empty) => {}
                }

                if !paused_flag.load(Ordering::Relaxed) {
                    if let Ok(text) = clipboard.get_text() {
                        if !text.is_empty() && text != last_text {
                            last_text = text.clone();
                            let _ = app_handle.emit("clipboard-changed", &text);
                        }
                    }
                }

                std::thread::sleep(Duration::from_millis(500));
            }
        });

        ClipboardMonitor { _stop: tx, paused }
    }

    pub fn set_paused(&self, paused: bool) {
        self.paused.store(paused, Ordering::Relaxed);
    }
}
