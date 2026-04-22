use std::sync::mpsc;
use std::time::Duration;
use tauri::Emitter;

pub struct ClipboardMonitor {
    _stop: mpsc::Sender<()>,
}

impl ClipboardMonitor {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        let (tx, rx) = mpsc::channel::<()>();

        std::thread::spawn(move || {
            let mut clipboard = match arboard::Clipboard::new() {
                Ok(c) => c,
                Err(_) => return,
            };
            let mut last_text = String::new();

            loop {
                if rx.try_recv().is_ok() {
                    break;
                }

                if let Ok(text) = clipboard.get_text() {
                    if !text.is_empty() && text != last_text {
                        last_text = text.clone();
                        let _ = app_handle.emit("clipboard-changed", &text);
                    }
                }

                std::thread::sleep(Duration::from_millis(500));
            }
        });

        ClipboardMonitor { _stop: tx }
    }
}
