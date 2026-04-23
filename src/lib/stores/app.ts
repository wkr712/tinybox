import { writable, get } from "svelte/store";
import { getCurrentWindow, currentMonitor } from "@tauri-apps/api/window";
import { LogicalSize, LogicalPosition } from "@tauri-apps/api/dpi";

const SIDEBAR_WIDTH = 52;
const PANEL_WIDTH = 360;
const COLLAPSED_WIDTH = SIDEBAR_WIDTH;
const EXPANDED_WIDTH = SIDEBAR_WIDTH + PANEL_WIDTH;
const ISLAND_WIDTH = 200;
const ISLAND_HEIGHT = 48;
const ISLAND_LYRICS_HEIGHT = 120;
const INITIAL_HEIGHT = 380;

export const expanded = writable(false);
export const activePanel = writable<string | null>(null);
export const minimized = writable(false);

let lastPanel = "notes";
let savedHeight = INITIAL_HEIGHT;
let isTransitioning = false;
let programmaticTarget: number | null = null;

let monitorCache: { w: number; h: number; scale: number; ts: number } | null = null;

let lastNormalPos: { x: number; y: number } | null = null;

activePanel.subscribe((v) => {
  if (v) lastPanel = v;
});

function invalidateMonitorCache() {
  monitorCache = null;
}

async function getScaleFactor(): Promise<number> {
  const cached = monitorCache;
  if (cached && Date.now() - cached.ts < 5000) return cached.scale;
  try {
    const monitor = await currentMonitor();
    return monitor?.scaleFactor ?? 1;
  } catch {
    return 1;
  }
}

async function getScreenSize(): Promise<{ w: number; h: number }> {
  const cached = monitorCache;
  if (cached && Date.now() - cached.ts < 5000) return { w: cached.w, h: cached.h };
  try {
    const monitor = await currentMonitor();
    if (monitor) {
      const s = monitor.scaleFactor;
      const size = { w: monitor.size.width / s, h: monitor.size.height / s };
      monitorCache = { w: size.w, h: size.h, scale: s, ts: Date.now() };
      return size;
    }
  } catch {}
  return { w: 1920, h: 1080 };
}

async function getCurrentPos(): Promise<{ x: number; y: number }> {
  try {
    const win = getCurrentWindow();
    const pos = await win.outerPosition();
    const scale = await getScaleFactor();
    return { x: Math.round(pos.x / scale), y: Math.round(pos.y / scale) };
  } catch {
    const screen = await getScreenSize();
    return { x: Math.round(screen.w / 2), y: Math.round(screen.h / 2) };
  }
}

function clampToScreen(x: number, y: number, w: number, h: number, screen: { w: number; h: number }): { x: number; y: number } {
  return {
    x: Math.max(-10, Math.min(x, screen.w - w + 10)),
    y: Math.max(0, Math.min(y, screen.h - h + 10)),
  };
}

async function applyWindowBoundsAnchored(width: number, height: number) {
  programmaticTarget = height;
  const win = getCurrentWindow();
  const screen = await getScreenSize();
  const pos = await getCurrentPos();
  const clamped = clampToScreen(pos.x, pos.y, width, height, screen);
  lastNormalPos = clamped;
  await Promise.all([
    win.setSize(new LogicalSize(width, height)),
    win.setPosition(new LogicalPosition(clamped.x, clamped.y)),
  ]);
  setTimeout(() => { programmaticTarget = null; }, 500);
}

async function applyWindowBoundsCentered(width: number, height: number, topBias = false) {
  programmaticTarget = height;
  const win = getCurrentWindow();
  const screen = await getScreenSize();
  const x = Math.round((screen.w - width) / 2);
  const y = topBias ? 16 : Math.round((screen.h - height) / 2);
  await Promise.all([
    win.setSize(new LogicalSize(width, height)),
    win.setPosition(new LogicalPosition(x, y)),
  ]);
  setTimeout(() => { programmaticTarget = null; }, 500);
}

export function initWindowResizeTracking() {
  const win = getCurrentWindow();
  let timer: ReturnType<typeof setTimeout>;
  win.onResized(async () => {
    clearTimeout(timer);
    timer = setTimeout(async () => {
      if (programmaticTarget !== null) return;
      if (!isTransitioning && get(expanded)) {
        try {
          const physical = await win.innerSize();
          const scale = await getScaleFactor();
          const h = physical.height / scale;
          if (h > 100 && h < 2000) savedHeight = Math.round(h);
        } catch {}
      }
    }, 300);
  });
  win.onMoved(async () => {
    invalidateMonitorCache();
    if (!isTransitioning && !get(minimized)) {
      const pos = await getCurrentPos();
      lastNormalPos = pos;
    }
  });
}

export async function expandWindow(panel: string) {
  if (isTransitioning) return;

  if (get(expanded) && get(activePanel)) {
    activePanel.set(panel);
    return;
  }

  isTransitioning = true;
  try {
    await applyWindowBoundsAnchored(EXPANDED_WIDTH, savedHeight);
    activePanel.set(panel);
    expanded.set(true);
    minimized.set(false);
  } finally {
    setTimeout(() => { isTransitioning = false; }, 100);
  }
}

export async function collapseWindow() {
  if (isTransitioning) return;
  isTransitioning = true;
  try {
    activePanel.set(null);
    expanded.set(false);
    await applyWindowBoundsAnchored(COLLAPSED_WIDTH, savedHeight);
  } finally {
    setTimeout(() => { isTransitioning = false; }, 100);
  }
}

export async function minimizeToIsland() {
  if (isTransitioning) return;
  isTransitioning = true;
  try {
    const pos = await getCurrentPos();
    lastNormalPos = pos;
    activePanel.set(null);
    expanded.set(false);
    minimized.set(true);
    await applyWindowBoundsCentered(ISLAND_WIDTH, ISLAND_HEIGHT, true);
  } finally {
    setTimeout(() => { isTransitioning = false; }, 100);
  }
}

export function getLastPanel(): string {
  return lastPanel;
}

export async function restoreFromIsland() {
  if (isTransitioning) return;
  isTransitioning = true;
  try {
    minimized.set(false);
    if (lastNormalPos) {
      programmaticTarget = savedHeight;
      const win = getCurrentWindow();
      await win.setSize(new LogicalSize(COLLAPSED_WIDTH, savedHeight));
      await win.setPosition(new LogicalPosition(lastNormalPos.x, lastNormalPos.y));
      setTimeout(() => { programmaticTarget = null; }, 500);
    } else {
      await applyWindowBoundsCentered(COLLAPSED_WIDTH, savedHeight);
    }
  } finally {
    setTimeout(() => { isTransitioning = false; }, 100);
  }
}

export async function expandIslandForLyrics() {
  if (!get(minimized) || isTransitioning) return;
  isTransitioning = true;
  try {
    const win = getCurrentWindow();
    const pos = await getCurrentPos();
    programmaticTarget = ISLAND_LYRICS_HEIGHT;
    await win.setSize(new LogicalSize(ISLAND_WIDTH, ISLAND_LYRICS_HEIGHT));
    await win.setPosition(new LogicalPosition(pos.x, pos.y));
    setTimeout(() => { programmaticTarget = null; }, 500);
  } finally {
    setTimeout(() => { isTransitioning = false; }, 100);
  }
}

export async function shrinkIslandFromLyrics() {
  if (!get(minimized) || isTransitioning) return;
  isTransitioning = true;
  try {
    const win = getCurrentWindow();
    const pos = await getCurrentPos();
    programmaticTarget = ISLAND_HEIGHT;
    await win.setSize(new LogicalSize(ISLAND_WIDTH, ISLAND_HEIGHT));
    await win.setPosition(new LogicalPosition(pos.x, pos.y));
    setTimeout(() => { programmaticTarget = null; }, 500);
  } finally {
    setTimeout(() => { isTransitioning = false; }, 100);
  }
}
