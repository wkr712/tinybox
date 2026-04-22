<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    clipboardHistory, initClipboardMonitor,
    clearHistory, getFilteredItems, getPreview, timeAgo,
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

  function handleClear() {
    if (confirm("清空所有非收藏的剪贴板记录？")) {
      clearHistory();
    }
  }
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-2 mb-3">
    <div class="flex-1 relative">
      <svg class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-white/30" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
      <input
        type="text"
        placeholder="搜索剪贴板..."
        bind:value={query}
        class="w-full bg-white/5 border border-white/10 rounded-lg pl-8 pr-3 py-1.5 text-xs text-white/80 placeholder-white/30 outline-none focus:border-white/20 transition-colors"
      />
    </div>
    <button
      onclick={handleClear}
      class="text-[10px] text-white/30 hover:text-red-400 px-2 py-1 rounded-lg hover:bg-white/5 transition-colors"
    >
      清空
    </button>
  </div>

  <div class="flex-1 overflow-y-auto space-y-1">
    {#if favorites.length > 0}
      <div class="text-[10px] text-white/30 uppercase tracking-wider mb-1 px-1">收藏</div>
      {#each favorites as item (item.id)}
        <ClipboardItem {item} />
      {/each}
      <div class="h-px bg-white/5 my-2"></div>
    {/if}

    {#each recent as item (item.id)}
      <ClipboardItem {item} />
    {/each}

    {#if filtered.length === 0 && items.length > 0}
      <div class="text-white/30 text-xs text-center py-8">没有匹配的记录</div>
    {:else if items.length === 0}
      <div class="text-white/30 text-xs text-center py-8">复制内容后会自动记录到这里</div>
    {/if}
  </div>
</div>
