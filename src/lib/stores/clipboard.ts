import { writable } from "svelte/store";
import { select, execute } from "../utils/db";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import type { ClipboardItem } from "../types/clipboard";
import { getSetting } from "./settings";

export const clipboardHistory = writable<ClipboardItem[]>([]);
export const searchQuery = writable("");
export let maxHistory = 100;

export function setMaxHistory(val: number) {
  maxHistory = val;
}

let initialized = false;

export async function initClipboardMonitor() {
  if (initialized) return;
  initialized = true;

  await loadHistory();

  await listen<string>("clipboard-changed", async (event) => {
    const enabled = getSetting("clipboard_monitor_enabled");
    if (enabled !== "true") return;

    const text = event.payload;
    if (!text.trim()) return;

    // Deduplicate: skip if last entry is the same
    const current = await getCurrentHistory();
    if (current.length > 0 && current[0].content === text) return;

    await execute(
      "INSERT INTO clipboard_history (content, content_type) VALUES (?, 'text')",
      [text]
    );

    // Auto-cleanup: keep only maxHistory records
    const count = await getCount();
    if (count > maxHistory) {
      await execute(
        `DELETE FROM clipboard_history WHERE id NOT IN (
          SELECT id FROM clipboard_history ORDER BY created_at DESC LIMIT ?
        ) AND is_favorite = 0`,
        [maxHistory]
      );
    }

    await loadHistory();
  });
}

async function getCurrentHistory(): Promise<ClipboardItem[]> {
  return await select<ClipboardItem>(
    "SELECT * FROM clipboard_history ORDER BY created_at DESC LIMIT 1"
  );
}

async function getCount(): Promise<number> {
  const result = await select<{ cnt: number }>(
    "SELECT COUNT(*) as cnt FROM clipboard_history"
  );
  return result[0]?.cnt ?? 0;
}

export async function loadHistory() {
  const result = await select<ClipboardItem>(
    "SELECT * FROM clipboard_history ORDER BY is_favorite DESC, created_at DESC"
  );
  clipboardHistory.set(result);
}

export async function toggleFavorite(id: number, current: number) {
  await execute("UPDATE clipboard_history SET is_favorite = ? WHERE id = ?", [
    current ? 0 : 1,
    id,
  ]);
  await loadHistory();
}

export async function deleteItem(id: number) {
  await execute("DELETE FROM clipboard_history WHERE id = ?", [id]);
  await loadHistory();
}

export async function clearHistory() {
  await execute("DELETE FROM clipboard_history WHERE is_favorite = 0");
  await loadHistory();
}

export async function copyToClipboard(text: string) {
  await invoke("clipboard_write_text", { text });
}

export function getFilteredItems(
  items: ClipboardItem[],
  query: string
): ClipboardItem[] {
  if (!query.trim()) return items;
  const q = query.toLowerCase();
  return items.filter((item) => item.content.toLowerCase().includes(q));
}

export function getPreview(text: string, maxLen = 80): string {
  const single = text.replace(/\n/g, " ").trim();
  return single.length > maxLen ? single.slice(0, maxLen) + "..." : single;
}

export function timeAgo(dateStr: string): string {
  const now = new Date();
  const d = new Date(dateStr);
  const diff = Math.floor((now.getTime() - d.getTime()) / 1000);
  if (diff < 60) return "刚刚";
  if (diff < 3600) return `${Math.floor(diff / 60)}分钟前`;
  if (diff < 86400) return `${Math.floor(diff / 3600)}小时前`;
  return `${Math.floor(diff / 86400)}天前`;
}
