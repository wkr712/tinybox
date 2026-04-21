import { writable } from "svelte/store";

export const expanded = writable(false);
export const activePanel = writable<string | null>(null);
export const alwaysOnTop = writable(true);
export const sidebarEdge = writable<"left" | "right">("left");
