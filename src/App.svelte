<script lang="ts">
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import { listen } from "@tauri-apps/api/event";
  import Sidebar from "./lib/components/sidebar/Sidebar.svelte";
  import PanelHost from "./lib/components/panels/PanelHost.svelte";
  import { activePanel } from "./lib/stores/app";
  import { loadSettings } from "./lib/stores/settings";
  import { registerHotkeys } from "./lib/utils/hotkeys";

  let currentPanel = $state<string | null>(null);
  activePanel.subscribe((v) => (currentPanel = v));

  let toastMsg = $state("");
  let toastTimer: ReturnType<typeof setTimeout> | null = null;

  onMount(async () => {
    await loadSettings();
    await registerHotkeys();

    listen<{ title?: string; body: string }>("notification", (event) => {
      const msg = event.payload.title
        ? `${event.payload.title}: ${event.payload.body}`
        : event.payload.body;
      toastMsg = msg;
      if (toastTimer) clearTimeout(toastTimer);
      toastTimer = setTimeout(() => (toastMsg = ""), 3000);
    });
  });
</script>

<div class="flex h-screen glass-panel">
  <Sidebar />

  {#if currentPanel}
    <div class="flex-1 overflow-hidden" transition:fly={{ x: -16, duration: 200 }}>
      <PanelHost />
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
