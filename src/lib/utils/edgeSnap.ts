import { getCurrentWindow, currentMonitor } from "@tauri-apps/api/window";
import { LogicalPosition } from "@tauri-apps/api/dpi";

const SNAP_THRESHOLD = 16;

interface MonitorInfo {
  x: number;
  y: number;
  w: number;
  h: number;
  scaleFactor: number;
}

async function getMonitorInfo(): Promise<MonitorInfo | null> {
  try {
    const mon = await currentMonitor();
    if (!mon) return null;
    const s = mon.scaleFactor;
    return {
      x: mon.position.x / s,
      y: mon.position.y / s,
      w: mon.size.width / s,
      h: mon.size.height / s,
      scaleFactor: s,
    };
  } catch {
    return null;
  }
}

interface SnapResult {
  x: number;
  y: number;
}

async function checkSnap(
  winX: number, winY: number, winW: number, winH: number
): Promise<SnapResult | null> {
  const mon = await getMonitorInfo();
  if (!mon) return null;

  const screenLeft = mon.x;
  const screenRight = mon.x + mon.w;
  const screenTop = mon.y;

  const leftDist = winX - screenLeft;
  const rightDist = screenRight - (winX + winW);
  const topDist = winY - screenTop;

  // Snap left: flush with left edge
  if (leftDist >= -5 && leftDist < SNAP_THRESHOLD) {
    return { x: screenLeft, y: winY };
  }
  // Snap right: flush with right edge
  if (rightDist >= -5 && rightDist < SNAP_THRESHOLD) {
    return { x: screenRight - winW, y: winY };
  }
  // Snap top
  if (topDist >= -5 && topDist < SNAP_THRESHOLD) {
    return { x: winX, y: screenTop };
  }
  return null;
}

let unlisten: (() => void) | null = null;
let debounceTimer: ReturnType<typeof setTimeout> | null = null;
let isSnapping = false;

export async function initEdgeSnap() {
  const win = getCurrentWindow();
  unlisten = await win.onMoved(async (event) => {
    if (isSnapping) return;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(async () => {
      try {
        const physSize = await win.innerSize();
        const mon = await getMonitorInfo();
        if (!mon) return;
        const s = mon.scaleFactor;
        const w = physSize.width / s;
        const h = physSize.height / s;
        const x = event.payload.x / s;
        const y = event.payload.y / s;
        const snap = await checkSnap(x, y, w, h);
        if (snap) {
          isSnapping = true;
          await win.setPosition(new LogicalPosition(snap.x, snap.y));
          setTimeout(() => { isSnapping = false; }, 200);
        }
      } catch {}
    }, 80);
  });
}

export function destroyEdgeSnap() {
  if (unlisten) {
    unlisten();
    unlisten = null;
  }
  if (debounceTimer) {
    clearTimeout(debounceTimer);
    debounceTimer = null;
  }
}
