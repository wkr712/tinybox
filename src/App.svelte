<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { fly } from "svelte/transition";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import Sidebar from "./lib/components/sidebar/Sidebar.svelte";
  import PanelHost from "./lib/components/panels/PanelHost.svelte";
  import ClipboardQuick from "./lib/components/ClipboardQuick.svelte";
  import FloatingPlayer from "./lib/components/sidebar/FloatingPlayer.svelte";
  import { activePanel, expanded, minimized, minimizeMode, initWindowResizeTracking, destroyWindowResizeTracking, restoreFromEdge } from "./lib/stores/app";
  import { loadSettings, settings } from "./lib/stores/settings";
  import { initAudioListener, destroyAudioListener, currentSong, isPlaying } from "./lib/stores/music";
  import { clipboardQuickOpen } from "./lib/stores/clipboard";
  import { registerHotkeys } from "./lib/utils/hotkeys";
  import { initEdgeSnap, destroyEdgeSnap } from "./lib/utils/edgeSnap";
  import { springFly } from "./lib/utils/transitions";

  let currentPanel = $state<string | null>(null);
  let isExpanded = $state(false);
  let isMinimized = $state(false);
  let appVisible = $state(false);
  let theme = $state("midnight");
  let quickOpen = $state(false);
  let miniMode = $state<"none" | "player" | "edge">("none");
  let song = $state<any>(null);
  let playing = $state(false);
  let unsubs: (() => void)[] = [];

  let toastMsg = $state("");
  let toastTimer: ReturnType<typeof setTimeout> | null = null;

  onMount(async () => {
    unsubs.push(activePanel.subscribe((v) => (currentPanel = v)));
    unsubs.push(expanded.subscribe((v) => (isExpanded = v)));
    unsubs.push(minimized.subscribe((v) => (isMinimized = v)));
    unsubs.push(minimizeMode.subscribe((v) => (miniMode = v)));
    unsubs.push(settings.subscribe((v) => (theme = v.theme || "midnight")));
    unsubs.push(clipboardQuickOpen.subscribe((v) => (quickOpen = v)));
    unsubs.push(currentSong.subscribe((v) => (song = v)));
    unsubs.push(isPlaying.subscribe((v) => (playing = v)));

    initWindowResizeTracking();

    // Load settings, register hotkeys, init audio — all before showing window
    await loadSettings();
    await registerHotkeys();
    await initAudioListener();
    await initEdgeSnap();

    // Signal Rust that UI is ready → show window, then fade in
    await invoke("window_ready");
    requestAnimationFrame(() => { appVisible = true; });

    const unlisten = await listen<{ title?: string; body: string }>("notification", (event) => {
      const msg = event.payload.title
        ? `${event.payload.title}: ${event.payload.body}`
        : event.payload.body;
      toastMsg = msg;
      if (toastTimer) clearTimeout(toastTimer);
      toastTimer = setTimeout(() => (toastMsg = ""), 3000);
    });
    unsubs.push(unlisten);
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    if (toastTimer) clearTimeout(toastTimer);
    destroyAudioListener();
    destroyEdgeSnap();
    destroyWindowResizeTracking();
  });

  let modeClass = $derived(
    isMinimized
      ? (miniMode === 'player' ? 'player-mode' : 'edge-mode')
      : ''
  );
</script>

<div
  class="root-container {modeClass} {appVisible ? 'app-enter' : 'app-hidden'}"
  data-theme={theme}
  data-tauri-drag-region
>
  {#if isMinimized}
    <!-- Minimized: only show mini content, no sidebar/panel -->
    {#if miniMode === "player"}
      <FloatingPlayer />
    {:else}
      <!-- edge mode: small icon -->
      <div class="edge-wrap" onclick={() => restoreFromEdge()} onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); restoreFromEdge(); } }} role="button" tabindex="0">
        <svg viewBox="0 0 100 100" width="28" height="28" xmlns="http://www.w3.org/2000/svg">
          <path d="M32 30 L28 10 L42 26" fill="currentColor" opacity="0.45" />
          <path d="M68 30 L72 10 L58 26" fill="currentColor" opacity="0.45" />
          <circle cx="50" cy="44" r="22" fill="currentColor" opacity="0.35" />
          <ellipse cx="42" cy="42" rx="3.5" ry="4" fill="currentColor" opacity="0.7" />
          <ellipse cx="58" cy="42" rx="3.5" ry="4" fill="currentColor" opacity="0.7" />
        </svg>
      </div>
    {/if}
  {:else}
    <!-- Normal mode: sidebar + panel -->
    <Sidebar />

    {#if currentPanel}
      <div class="flex-1 overflow-hidden relative" transition:springFly={{ x: -12, duration: 280 }}>
        <PanelHost />
        {#if quickOpen}
          <ClipboardQuick />
        {/if}
      </div>
    {/if}
  {/if}
</div>

{#if toastMsg}
  <div
    class="fixed bottom-4 left-1/2 -translate-x-1/2 bg-white/10 text-white/80 text-xs px-4 py-2 rounded-lg shadow-lg z-50"
    transition:fly={{ y: 20, duration: 200 }}
  >
    {toastMsg}
  </div>
{/if}

<style>
  .root-container {
    display: flex;
    height: 100vh;
    position: relative;
    overflow: hidden;
    border-radius: 16px;
    border: none;
    background: var(--color-dark-bg);
    box-shadow:
      0 16px 64px rgba(0, 0, 0, 0.5),
      0 4px 16px rgba(0, 0, 0, 0.25);
    transition: border-radius 0.4s cubic-bezier(0.4, 0, 0.2, 1),
                box-shadow 0.4s ease;
  }

  .player-mode {
    border-radius: 24px;
    align-items: center;
    justify-content: center;
    box-shadow:
      0 8px 32px rgba(0, 0, 0, 0.5),
      0 0 40px color-mix(in srgb, var(--color-accent-primary) 2%, transparent);
    animation: player-breathe 4s ease-in-out infinite;
  }

  .edge-mode {
    border-radius: 14px;
    align-items: center;
    justify-content: center;
    box-shadow:
      0 4px 20px rgba(0, 0, 0, 0.4);
  }

  .edge-wrap {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-accent-primary);
    cursor: pointer;
  }

  @keyframes player-breathe {
    0%, 100% {
      box-shadow:
        0 8px 32px rgba(0, 0, 0, 0.5),
        0 0 40px color-mix(in srgb, var(--color-accent-primary) 2%, transparent);
    }
    50% {
      box-shadow:
        0 8px 32px rgba(0, 0, 0, 0.5),
        0 0 50px color-mix(in srgb, var(--color-accent-primary) 3%, transparent);
    }
  }
</style>
