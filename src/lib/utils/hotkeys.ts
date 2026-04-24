import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import { get } from "svelte/store";
import { settings } from "../stores/settings";
import { activePanel, expandWindow, collapseWindow, getLastPanel } from "../stores/app";
import { pauseMusic, resumeMusic, currentView, isPlaying } from "../stores/music";
import { clipboardQuickOpen } from "../stores/clipboard";

let registered: string[] = [];

export async function registerHotkeys() {
  for (const key of registered) {
    try { await unregister(key); } catch { /* ignore */ }
  }
  registered = [];

  const s = getSettings();
  const hotkeys: Record<string, () => void> = {
    [s.hotkey_toggle_sidebar]: () => toggleSidebar(),
    [s.hotkey_clipboard]: () => openClipboardQuick(),
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
  const panel = get(activePanel);
  if (panel) {
    await collapseWindow();
  } else {
    await expandWindow(getLastPanel());
  }
}

async function openClipboardQuick() {
  await expandWindow("clipboard");
  clipboardQuickOpen.set(true);
}

async function togglePlayPause() {
  if (get(isPlaying)) {
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
  return get(settings) as unknown as Record<string, string>;
}
