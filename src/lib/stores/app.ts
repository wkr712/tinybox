import { writable, get } from "svelte/store";
import { getCurrentWindow, currentMonitor } from "@tauri-apps/api/window";
import { LogicalSize, LogicalPosition } from "@tauri-apps/api/dpi";

const SIDEBAR_WIDTH = 52;
const PANEL_WIDTH = 360;
const COLLAPSED_WIDTH = SIDEBAR_WIDTH;
const EXPANDED_WIDTH = SIDEBAR_WIDTH + PANEL_WIDTH;
const PLAYER_WIDTH = 320;
const PLAYER_HEIGHT = 80;
const EDGE_SIZE = 48;
const INITIAL_HEIGHT = 380;

export const expanded = writable(false);
export const activePanel = writable<string | null>(null);
export const minimized = writable(false);
export const minimizeMode = writable<"none" | "player" | "edge">("none");

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
  // Top-center: Y at 10% of screen height (screen top area)
  const y = topBias ? Math.round(screen.h * 0.10) : Math.round((screen.h - height) / 2);
  await Promise.all([
    win.setSize(new LogicalSize(width, height)),
    win.setPosition(new LogicalPosition(x, y)),
  ]);
  setTimeout(() => { programmaticTarget = null; }, 500);
}

let resizeCleanup: (() => void) | null = null;

export function initWindowResizeTracking() {
  const win = getCurrentWindow();
  let timer: ReturnType<typeof setTimeout>;
  let unresolves: (() => void)[] = [];

  win.onResized(async () => {
    clearTimeout(timer);
    timer = setTimeout(async () => {
      if (programmaticTarget !== null) return;
      if (isTransitioning) return;
      try {
        const physical = await win.innerSize();
        const scale = await getScaleFactor();
        const w = physical.width / scale;
        const h = physical.height / scale;

        // In minimized player mode: switch to full UI when both dimensions are large enough
        if (get(minimized) && get(minimizeMode) === "player" && w > 260 && h > 360) {
          const wasTransitioning = isTransitioning;
          isTransitioning = true;
          try {
            minimized.set(false);
            minimizeMode.set("none");
            activePanel.set("music");
            expanded.set(true);
            if (h > 100 && h < 2000) savedHeight = Math.round(h);
            // Resize to proper full UI dimensions
            programmaticTarget = savedHeight;
            const win2 = getCurrentWindow();
            await win2.setSize(new LogicalSize(EXPANDED_WIDTH, savedHeight));
            setTimeout(() => { programmaticTarget = null; }, 500);
          } finally {
            setTimeout(() => { isTransitioning = false; }, 200);
          }
          return;
        }

        // In minimized edge mode: if user drags large, restore
        if (get(minimized) && get(minimizeMode) === "edge" && (w > EDGE_SIZE + 30 || h > EDGE_SIZE + 30)) {
          minimized.set(false);
          minimizeMode.set("none");
          activePanel.set(lastPanel);
          expanded.set(true);
          if (h > 100 && h < 2000) savedHeight = Math.round(h);
          return;
        }

        // Normal mode: auto-minimize to floating player when window dragged small enough
        if (!get(minimized) && !get(expanded) && h < 100) {
          isTransitioning = true;
          try {
            const pos = await getCurrentPos();
            lastNormalPos = pos;
            activePanel.set(null);
            expanded.set(false);
            minimized.set(true);
            minimizeMode.set("player");
            await applyWindowBoundsCentered(PLAYER_WIDTH, PLAYER_HEIGHT);
          } finally {
            setTimeout(() => { isTransitioning = false; }, 200);
          }
          return;
        }

        if (get(expanded) && !get(minimized)) {
          if (h > 100 && h < 2000) savedHeight = Math.round(h);
        }
      } catch {}
    }, 300);
  }).then((unlisten) => unresolves.push(unlisten));

  win.onMoved(async () => {
    invalidateMonitorCache();
    if (!isTransitioning && !get(minimized)) {
      const pos = await getCurrentPos();
      lastNormalPos = pos;
    }
  }).then((unlisten) => unresolves.push(unlisten));

  resizeCleanup = () => {
    for (const u of unresolves) u();
    unresolves = [];
    clearTimeout(timer);
  };
}

export function destroyWindowResizeTracking() {
  if (resizeCleanup) {
    resizeCleanup();
    resizeCleanup = null;
  }
}

export async function expandWindow(panel: string) {
  if (isTransitioning) return;
  if (get(expanded) && get(activePanel)) {
    activePanel.set(panel);
    return;
  }
  isTransitioning = true;
  try {
    const win = getCurrentWindow();
    const physical = await win.innerSize();
    const scale = await getScaleFactor();
    const currentWidth = physical.width / scale;
    const width = currentWidth > EXPANDED_WIDTH ? currentWidth : EXPANDED_WIDTH;
    await applyWindowBoundsAnchored(width, savedHeight);
    activePanel.set(panel);
    expanded.set(true);
    minimized.set(false);
    minimizeMode.set("none");
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

export async function minimizeToPlayer() {
  if (isTransitioning) return;
  isTransitioning = true;
  try {
    const pos = await getCurrentPos();
    lastNormalPos = pos;
    activePanel.set(null);
    expanded.set(false);
    minimized.set(true);
    minimizeMode.set("player");
    // Position floating player at top-center of screen
    await applyWindowBoundsCentered(PLAYER_WIDTH, PLAYER_HEIGHT, true);
  } finally {
    setTimeout(() => { isTransitioning = false; }, 100);
  }
}

export async function restoreFromPlayer(retries = 0) {
  if (isTransitioning) {
    if (retries > 10) return;
    setTimeout(() => restoreFromPlayer(retries + 1), 150);
    return;
  }
  isTransitioning = true;
  try {
    minimized.set(false);
    minimizeMode.set("none");
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

export async function minimizeToEdge() {
  if (isTransitioning) return;
  isTransitioning = true;
  try {
    const pos = await getCurrentPos();
    lastNormalPos = pos;
    activePanel.set(null);
    expanded.set(false);
    minimized.set(true);
    minimizeMode.set("edge");
    const screen = await getScreenSize();
    const x = screen.w - EDGE_SIZE - 8;
    const y = Math.round((screen.h - EDGE_SIZE) / 2);
    programmaticTarget = EDGE_SIZE;
    const win = getCurrentWindow();
    await win.setSize(new LogicalSize(EDGE_SIZE, EDGE_SIZE));
    await win.setPosition(new LogicalPosition(x, y));
    setTimeout(() => { programmaticTarget = null; }, 500);
  } finally {
    setTimeout(() => { isTransitioning = false; }, 100);
  }
}

export async function restoreFromEdge(retries = 0) {
  if (isTransitioning) {
    if (retries > 10) return;
    setTimeout(() => restoreFromEdge(retries + 1), 150);
    return;
  }
  isTransitioning = true;
  try {
    minimized.set(false);
    minimizeMode.set("none");
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

export async function resetWindow() {
  if (isTransitioning) return;
  isTransitioning = true;
  try {
    savedHeight = INITIAL_HEIGHT;
    minimized.set(false);
    minimizeMode.set("none");
    activePanel.set(null);
    expanded.set(false);
    await applyWindowBoundsCentered(COLLAPSED_WIDTH, INITIAL_HEIGHT);
  } finally {
    setTimeout(() => { isTransitioning = false; }, 100);
  }
}

export function getLastPanel(): string {
  return lastPanel;
}
