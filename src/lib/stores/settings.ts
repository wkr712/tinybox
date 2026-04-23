import { writable, get } from "svelte/store";
import { select, execute } from "../utils/db";

interface SettingsMap {
  sidebar_edge: string;
  always_on_top: string;
  clipboard_max_history: string;
  clipboard_monitor_enabled: string;
  pomodoro_work_duration: string;
  pomodoro_break_duration: string;
  music_volume: string;
  hotkey_toggle_sidebar: string;
  hotkey_clipboard: string;
  hotkey_new_note: string;
  hotkey_play_pause: string;
  hotkey_show_lyrics: string;
}

const defaults: SettingsMap = {
  sidebar_edge: "left",
  always_on_top: "true",
  clipboard_max_history: "100",
  clipboard_monitor_enabled: "true",
  pomodoro_work_duration: "25",
  pomodoro_break_duration: "5",
  music_volume: "80",
  hotkey_toggle_sidebar: "Alt+Space",
  hotkey_clipboard: "Control+Shift+V",
  hotkey_new_note: "Control+Shift+N",
  hotkey_play_pause: "Control+Shift+M",
  hotkey_show_lyrics: "Control+Shift+L",
};

export const settings = writable<SettingsMap>({ ... defaults });

export async function loadSettings() {
  const rows = await select<{ key: string; value: string }>("SELECT key, value FROM settings");
  const map: Record<string, string> = {};
  for (const row of rows) {
    map[row.key] = row.value;
  }
  settings.set({ ...defaults, ...map } as SettingsMap);
}

export async function saveSetting(key: string, value: string) {
  await execute("INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now', 'localtime'))", [key, value]);
  settings.update((s) => ({ ...s, [key]: value }));
}

export function getSetting(key: keyof SettingsMap): string | undefined {
  return get(settings)[key];
}
