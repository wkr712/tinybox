import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import { settings } from "../stores/settings";
import { activePanel, expandWindow, collapseWindow } from "../stores/app";
import { pauseMusic, resumeMusic, currentView, isPlaying } from "../stores/music";

let registered: string[] = [];

export async function registerHotkeys() {
  for (const key of registered) {
    try { await unregister(key); } catch { /* ignore */ }
  }
  registered = [];

  const s = getSettings();
  const hotkeys: Record<string, () => void> = {
    [s.hotkey_toggle_sidebar]: () => toggleSidebar(),
    [s.hotkey_clipboard]: () => expandWindow("clipboard"),
    [s.hotkey_new_note]: () => expandWindow("notes"),
    [s.hotkey_play_pause]: () => togglePlayPause(),
    [s.hotkey_show_lyrics]: () => showLyrics(),
  };

  for (const [shortcut, handler] of Object.entries(hotkeys)) {
    if (!shortcut) continue;
    try {
      await register(shortcut, (event) => {
        if (event.state === "Pressed") handler();
      });
      registered.push(shortcut);
    } catch (e) {
      console.warn(`Failed to register hotkey: ${shortcut}`, e);
    }
  }
}

async function toggleSidebar() {
  let panel: string | null = null;
  const unsub = activePanel.subscribe((v) => { panel = v; });
  unsub();

  if (panel) {
    await collapseWindow();
  } else {
    await expandWindow("notes");
  }
}

async function togglePlayPause() {
  let playing = false;
  const unsub = isPlaying.subscribe((v) => { playing = v; });
  unsub();

  if (playing) {
    await pauseMusic();
  } else {
    await resumeMusic();
  }
}

async function showLyrics() {
  await expandWindow("music");
  currentView.set("nowplaying");
}

function getSettings(): Record<string, string> {
  let s: any = {};
  const unsub = settings.subscribe((v) => { s = v; });
  unsub();
  return s;
}
