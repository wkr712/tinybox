import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import { settings } from "../stores/settings";
import { activePanel, expanded } from "../stores/app";

let registered: string[] = [];

export async function registerHotkeys() {
  // Unregister all existing
  for (const key of registered) {
    try { await unregister(key); } catch { /* ignore */ }
  }
  registered = [];

  const s = getSnapshot();
  const hotkeys: Record<string, () => void> = {
    [s.hotkey_toggle_sidebar]: () => toggleSidebar(),
    [s.hotkey_clipboard]: () => showPanel("clipboard"),
    [s.hotkey_new_note]: () => showPanel("notes"),
    [s.hotkey_play_pause]: () => showPanel("music"),
    [s.hotkey_show_lyrics]: () => showPanel("music"),
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

function toggleSidebar() {
  expanded.update((v) => !v);
  if (!getSnapshot_expanded()) {
    // If collapsing, clear panel
  }
}

function showPanel(panel: string) {
  expanded.set(true);
  activePanel.set(panel);
}

function getSnapshot(): ReturnType<typeof getSettingsSnapshot> {
  let s: any = {};
  const unsub = settings.subscribe((v) => { s = v; });
  unsub();
  return s;
}

function getSettingsSnapshot() {
  let s: any = {};
  const unsub = settings.subscribe((v) => { s = v; });
  unsub();
  return s;
}

function getSnapshot_expanded(): boolean {
  let v = false;
  const unsub = expanded.subscribe((val) => { v = val; });
  unsub();
  return v;
}
