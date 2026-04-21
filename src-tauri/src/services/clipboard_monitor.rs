use std::sync::mpsc;
use std::time::Duration;
use tauri::Emitter;

pub struct ClipboardMonitor {
    running: mpsc::Sender<()>,
}

impl ClipboardMonitor {
    pub fn new(app_handle: tauri::AppHandle, max_history: usize) -> Self {
        let (tx, rx) = mpsc::channel::<()>();

        std::thread::spawn(move || {
            let mut clipboard = match arboard::Clipboard::new() {
                Ok(c) => c,
                Err(_) => return,
            };
            let mut last_text = String::new();
            let mut count: usize = 0;

            loop {
                if rx.try_recv().is_ok() {
                    break;
                }

                if let Ok(text) = clipboard.get_text() {
                    if !text.is_empty() && text != last_text {
                        last_text = text.clone();
                        let _ = app_handle.emit("clipboard-changed", &text);
                        count += 1;

                        // Auto-cleanup hint every 50 items
                        if count % 50 == 0 {
                            let _ =
                                app_handle.emit("clipboard-cleanup", serde_json::json!(max_history));
                        }
                    }
                }

                std::thread::sleep(Duration::from_millis(500));
            }
        });

        ClipboardMonitor { running: tx }
    }

    pub fn stop(&self) {
        let _ = self.running.send(());
    }
}
