<script lang="ts">
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import Sidebar from "./lib/components/sidebar/Sidebar.svelte";
  import PanelHost from "./lib/components/panels/PanelHost.svelte";
  import { activePanel } from "./lib/stores/app";
  import { loadSettings } from "./lib/stores/settings";
  import { registerHotkeys } from "./lib/utils/hotkeys";

  let currentPanel = $state<string | null>(null);
  activePanel.subscribe((v) => (currentPanel = v));

  onMount(async () => {
    await loadSettings();
    await registerHotkeys();
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
