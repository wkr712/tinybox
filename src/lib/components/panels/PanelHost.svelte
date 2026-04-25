<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { activePanel, collapseWindow } from "../../stores/app";
  import { activeProvider } from "../../stores/music";
  import NotesPanel from "./notes/NotesPanel.svelte";
  import TodoPanel from "./todo/TodoPanel.svelte";
  import ClipboardPanel from "./clipboard/ClipboardPanel.svelte";
  import DropZonePanel from "./dropzone/DropzonePanel.svelte";
  import MusicPanel from "./music/MusicPanel.svelte";
  import SettingsPanel from "./settings/SettingsPanel.svelte";

  let current = $state<string | null>(null);
  let provider = $state<string>("ncm");
  let unsubs: (() => void)[] = [];

  onMount(() => {
    unsubs.push(activePanel.subscribe((v) => (current = v)));
    unsubs.push(activeProvider.subscribe((v) => (provider = v)));
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  const baseTitles: Record<string, Record<string, string>> = {
    music: {
      ncm: "网易云音乐",
      qqmusic: "QQ音乐",
      kugou: "酷狗音乐",
    },
  };
  const staticTitles: Record<string, string> = {
    notes: "便签",
    todo: "待办计时",
    clipboard: "剪贴板历史",
    dropzone: "文件暂存",
    settings: "设置",
  };

  let panelTitle = $derived(
    current
      ? (baseTitles[current]?.[provider] ?? staticTitles[current] ?? "")
      : ""
  );
</script>

<div class="h-full flex flex-col">
  <div class="panel-header" data-tauri-drag-region>
    <span class="panel-title" data-tauri-drag-region>
      {panelTitle}
    </span>
    <button class="close-btn" onclick={collapseWindow} title="关闭面板" aria-label="关闭面板">
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
        <line x1="6" y1="6" x2="18" y2="18"/>
        <line x1="18" y1="6" x2="6" y2="18"/>
      </svg>
    </button>
  </div>

  <div class="flex-1 overflow-hidden px-4 pb-4 pt-2">
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
    background: var(--color-border-subtle);
  }

  .panel-title {
    flex: 1;
    text-align: center;
    font-size: 11px;
    font-weight: 600;
    color: var(--color-text-secondary);
    letter-spacing: 0.06em;
  }

  .close-btn {
    width: 24px;
    height: 24px;
    border-radius: 6px;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    color: var(--color-text-muted);
    transition: all 0.15s ease;
    flex-shrink: 0;
  }

  .close-btn:hover {
    color: var(--color-text-primary);
    background: var(--color-border-subtle);
  }

  .close-btn:active {
    transform: scale(0.9);
  }

  .panel-content {
    height: 100%;
    animation: panel-enter 0.2s ease both;
  }

  @keyframes panel-enter {
    from { opacity: 0; transform: translateY(4px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
