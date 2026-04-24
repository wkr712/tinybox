<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { fly } from "svelte/transition";
  import { listen } from "@tauri-apps/api/event";
  import Sidebar from "./lib/components/sidebar/Sidebar.svelte";
  import PanelHost from "./lib/components/panels/PanelHost.svelte";
  import ClipboardQuick from "./lib/components/ClipboardQuick.svelte";
  import { activePanel, expanded, minimized, initWindowResizeTracking, minimizeToIsland, restoreFromCat } from "./lib/stores/app";
  import { loadSettings, settings } from "./lib/stores/settings";
  import { initAudioListener, destroyAudioListener } from "./lib/stores/music";
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
  let unsubs: (() => void)[] = [];

  let toastMsg = $state("");
  let toastTimer: ReturnType<typeof setTimeout> | null = null;

  onMount(async () => {
    unsubs.push(activePanel.subscribe((v) => (currentPanel = v)));
    unsubs.push(expanded.subscribe((v) => (isExpanded = v)));
    unsubs.push(minimized.subscribe((v) => (isMinimized = v)));
    unsubs.push(settings.subscribe((v) => (theme = v.theme || "midnight")));
    unsubs.push(clipboardQuickOpen.subscribe((v) => (quickOpen = v)));

    initWindowResizeTracking();
    requestAnimationFrame(() => { appVisible = true; });

    await loadSettings();
    await registerHotkeys();
    await initAudioListener();
    await initEdgeSnap();

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
  });
</script>

<div
  class="root-container {isMinimized ? 'island-mode' : ''} {appVisible ? 'app-enter' : 'app-hidden'}"
  data-theme={theme}
  data-tauri-drag-region
>
  <Sidebar />

  {#if currentPanel && !isMinimized}
    <div class="flex-1 overflow-hidden relative" transition:springFly={{ x: -12, duration: 280 }}>
      <PanelHost />
      {#if quickOpen}
        <ClipboardQuick />
      {/if}
    </div>
  {/if}
</div>

{#if toastMsg}
  <div
    class="fixed bottom-4 left-1/2 -translate-x-1/2 bg-white/10 backdrop-blur-md text-white/80 text-xs px-4 py-2 rounded-lg shadow-lg z-50"
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
    border-radius: 14px;
    border: 1px solid var(--color-dark-border);
    background: color-mix(in srgb, var(--color-dark-bg) 92%, transparent);
    backdrop-filter: blur(40px) saturate(200%);
    -webkit-backdrop-filter: blur(40px) saturate(200%);
    box-shadow:
      0 0 0 1px var(--color-border-subtle),
      0 12px 48px rgba(0, 0, 0, 0.5),
      0 2px 12px rgba(0, 0, 0, 0.25),
      inset 0 1px 0 var(--color-surface-5);
    transition: border-radius 0.4s cubic-bezier(0.4, 0, 0.2, 1),
                background 0.4s ease,
                box-shadow 0.4s ease,
                border-color 0.4s ease;
  }

  .island-mode {
    border-radius: 22px;
    border-color: var(--color-dark-border);
    background: color-mix(in srgb, var(--color-dark-bg) 97%, transparent);
    backdrop-filter: blur(48px) saturate(210%);
    -webkit-backdrop-filter: blur(48px) saturate(210%);
    box-shadow:
      0 0 0 1px var(--color-border-subtle),
      0 6px 32px rgba(0, 0, 0, 0.6),
      0 0 60px rgba(0, 0, 0, 0.35),
      0 0 80px color-mix(in srgb, var(--color-accent-primary) 3%, transparent),
      inset 0 1px 0 var(--color-surface-8);
  }
</style>
