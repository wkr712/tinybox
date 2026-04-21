<script lang="ts">
  import Sidebar from "./lib/components/sidebar/Sidebar.svelte";
  import PanelHost from "./lib/components/panels/PanelHost.svelte";
  import { expanded, activePanel } from "./lib/stores/app";

  let currentPanel = $state<string | null>(null);

  activePanel.subscribe((v) => (currentPanel = v));
</script>

<div class="flex h-screen glass-panel">
  <Sidebar />

  {#if currentPanel}
    <div class="flex-1 overflow-hidden animate-in">
      <PanelHost />
    </div>
  {/if}
</div>

<style>
  .animate-in {
    animation: slideIn 0.25s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateX(-12px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }
</style>
