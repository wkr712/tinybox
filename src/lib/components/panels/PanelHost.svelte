<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { activePanel, collapseWindow, minimized, minimizeToIsland } from "../../stores/app";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import NotesPanel from "./notes/NotesPanel.svelte";
  import TodoPanel from "./todo/TodoPanel.svelte";
  import ClipboardPanel from "./clipboard/ClipboardPanel.svelte";
  import DropZonePanel from "./dropzone/DropzonePanel.svelte";
  import MusicPanel from "./music/MusicPanel.svelte";
  import SettingsPanel from "./settings/SettingsPanel.svelte";

  let current = $state<string | null>(null);
  let unsubs: (() => void)[] = [];

  onMount(() => {
    unsubs.push(activePanel.subscribe((v) => (current = v)));
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  const panelTitles: Record<string, string> = {
    notes: "便签",
    todo: "待办计时",
    clipboard: "剪贴板历史",
    dropzone: "文件暂存",
    music: "网易云音乐",
    settings: "设置",
  };
</script>

<div class="h-full flex flex-col">
  <!-- macOS-style title bar with traffic lights -->
  <div class="panel-header" data-tauri-drag-region>
    <div class="traffic-lights">
      <button
        class="traffic-dot close"
        onclick={() => getCurrentWindow().close()}
        title="关闭"
        aria-label="关闭窗口"
      >
        <svg width="8" height="8" viewBox="0 0 12 12">
          <line x1="2" y1="2" x2="10" y2="10" stroke="rgba(80,0,0,0.8)" stroke-width="1.5" stroke-linecap="round"/>
          <line x1="10" y1="2" x2="2" y2="10" stroke="rgba(80,0,0,0.8)" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </button>
      <button
        class="traffic-dot minimize"
        onclick={minimizeToIsland}
        title="最小化"
        aria-label="最小化到岛"
      >
        <svg width="8" height="8" viewBox="0 0 12 12">
          <line x1="2" y1="6" x2="10" y2="6" stroke="rgba(80,60,0,0.8)" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </button>
      <button
        class="traffic-dot collapse"
        onclick={collapseWindow}
        title="收起"
        aria-label="收起面板"
      >
        <svg width="8" height="8" viewBox="0 0 12 12">
          <polyline points="3 8 6 4 9 8" stroke="rgba(0,60,0,0.8)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
        </svg>
      </button>
    </div>
    <span class="panel-title" data-tauri-drag-region>
      {current ? panelTitles[current] || "" : ""}
    </span>
    <div class="traffic-spacer"></div>
  </div>

  <div class="flex-1 overflow-hidden px-4 pb-4">
    {#if current === "notes"}
      <div class="panel-content"><NotesPanel /></div>
    {:else if current === "todo"}
      <div class="panel-content"><TodoPanel /></div>
    {:else if current === "clipboard"}
      <div class="panel-content"><ClipboardPanel /></div>
    {:else if current === "dropzone"}
      <div class="panel-content"><DropZonePanel /></div>
    {:else if current === "music"}
      <div class="panel-content"><MusicPanel /></div>
    {:else if current === "settings"}
      <div class="panel-content"><SettingsPanel /></div>
    {/if}
  </div>
</div>

<style>
  .panel-header {
    display: flex;
    align-items: center;
    padding: 10px 14px 6px;
    gap: 8px;
    user-select: none;
    position: relative;
  }

  .panel-header::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 14px;
    right: 14px;
    height: 1px;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.06), transparent);
  }

  .panel-title {
    flex: 1;
    text-align: center;
    font-size: 11px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.45);
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .traffic-spacer {
    width: 58px;
    flex-shrink: 0;
  }

  .traffic-lights {
    display: flex;
    gap: 7px;
    flex-shrink: 0;
  }

  .traffic-dot {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    background: transparent;
    transition: transform 0.15s cubic-bezier(0.175, 0.885, 0.32, 1.275),
                filter 0.15s ease;
  }

  .traffic-dot::after {
    content: '';
    position: absolute;
    width: 12px;
    height: 12px;
    border-radius: 50%;
  }

  .traffic-dot:active {
    transform: scale(0.85);
  }

  .traffic-dot svg {
    opacity: 0;
    transition: opacity 0.12s;
    position: relative;
    z-index: 1;
  }

  .traffic-lights:hover .traffic-dot svg {
    opacity: 1;
  }

  .traffic-dot.close::after {
    background: #ff5f57;
    box-shadow: 0 0 4px rgba(255, 95, 87, 0.3);
  }

  .traffic-dot.minimize::after {
    background: #febc2e;
    box-shadow: 0 0 4px rgba(254, 188, 46, 0.3);
  }

  .traffic-dot.collapse::after {
    background: #28c840;
    box-shadow: 0 0 4px rgba(40, 200, 64, 0.3);
  }

  .traffic-dot:hover {
    transform: scale(1.2);
    filter: brightness(1.15);
  }

  .panel-content {
    animation: panel-enter 0.25s cubic-bezier(0.16, 1, 0.3, 1) both;
  }

  @keyframes panel-enter {
    from {
      opacity: 0;
      transform: translateY(6px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
