import { writable } from "svelte/store";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { LogicalSize } from "@tauri-apps/api/dpi";

const SIDEBAR_WIDTH = 52;
const PANEL_WIDTH = 360;
const COLLAPSED_WIDTH = SIDEBAR_WIDTH;
const EXPANDED_WIDTH = SIDEBAR_WIDTH + PANEL_WIDTH;

export const expanded = writable(false);
export const activePanel = writable<string | null>(null);

export async function expandWindow(panel: string) {
  activePanel.set(panel);
  expanded.set(true);
  const win = getCurrentWindow();
  await win.setSize(new LogicalSize(EXPANDED_WIDTH, await getWindowHeight()));
}

export async function collapseWindow() {
  activePanel.set(null);
  expanded.set(false);
  const win = getCurrentWindow();
  await win.setSize(new LogicalSize(COLLAPSED_WIDTH, await getWindowHeight()));
}

async function getWindowHeight(): Promise<number> {
  const win = getCurrentWindow();
  const size = await win.innerSize();
  return size.height;
}
