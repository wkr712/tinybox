<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { ask } from "@tauri-apps/plugin-dialog";
  import {
    clipboardHistory, initClipboardMonitor,
    clearHistory, getFilteredItems,
  } from "../../../stores/clipboard";
  import ClipboardItem from "./ClipboardItem.svelte";
  import type { ClipboardItem as CI } from "../../../types/clipboard";

  let items = $state<CI[]>([]);
  let query = $state("");
  let unsubs: (() => void)[] = [];

  let filtered = $derived(getFilteredItems(items, query));
  let favorites = $derived(filtered.filter((i) => i.is_favorite));
  let recent = $derived(filtered.filter((i) => !i.is_favorite));

  onMount(() => {
    unsubs.push(clipboardHistory.subscribe((v) => (items = v)));

    initClipboardMonitor();
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  async function handleClear() {
    const yes = await ask("清空所有非收藏的剪贴板记录？", { title: "确认", kind: "warning", okLabel: "清空", cancelLabel: "取消" });
    if (yes) {
      clearHistory();
    }
  }
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-2 mb-3">
    <div class="flex-1 search-box">
      <svg class="search-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
      <input
        type="text"
        placeholder="搜索剪贴板..."
        bind:value={query}
        class="search-input"
      />
    </div>
    <button
      onclick={handleClear}
      class="text-[10px] text-white/25 hover:text-red-400 px-2 py-1.5 rounded-lg hover:bg-white/[0.04] active:scale-95 transition-all"
    >
      清空
    </button>
  </div>

  <div class="flex-1 overflow-y-auto space-y-1.5">
    {#if favorites.length > 0}
      <div class="section-label">收藏</div>
      {#each favorites as item (item.id)}
        <ClipboardItem {item} />
      {/each}
      <div class="divider"></div>
    {/if}

    {#each recent as item (item.id)}
      <ClipboardItem {item} />
    {/each}

    {#if filtered.length === 0 && items.length > 0}
      <div class="empty-state">
        <span>没有匹配的记录</span>
      </div>
    {:else if items.length === 0}
      <div class="empty-state">
        <span>复制内容后会自动记录到这里</span>
      </div>
    {/if}
  </div>
</div>

<style>
  .search-box {
    position: relative;
  }

  .search-icon {
    position: absolute;
    left: 8px;
    top: 50%;
    transform: translateY(-50%);
    width: 14px;
    height: 14px;
    color: rgba(255, 255, 255, 0.25);
  }

  .search-input {
    width: 100%;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    padding: 6px 12px 6px 28px;
    font-size: 12px;
    color: rgba(255, 255, 255, 0.8);
    outline: none;
    transition: all 0.2s ease;
  }

  .search-input::placeholder {
    color: rgba(255, 255, 255, 0.25);
  }

  .search-input:focus {
    border-color: color-mix(in srgb, var(--color-accent-primary) 30%, transparent);
    background: rgba(255, 255, 255, 0.06);
  }

  .section-label {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.25);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    margin-bottom: 4px;
  }

  .divider {
    height: 1px;
    background: var(--color-border-subtle);
    margin: 8px 0;
  }

  .empty-state {
    text-align: center;
    padding: 32px 0;
    color: rgba(255, 255, 255, 0.25);
    font-size: 12px;
  }
</style>
