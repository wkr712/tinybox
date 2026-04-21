<script lang="ts">
  import {
    toggleFavorite, deleteItem, copyToClipboard, getPreview, timeAgo,
  } from "../../../stores/clipboard";
  import type { ClipboardItem } from "../../../types/clipboard";

  let { item }: { item: ClipboardItem } = $props();
  let copied = $state(false);
  let showActions = $state(false);

  async function handleCopy() {
    await copyToClipboard(item.content);
    copied = true;
    setTimeout(() => (copied = false), 1500);
  }

  function handleFavorite(e: MouseEvent) {
    e.stopPropagation();
    toggleFavorite(item.id, item.is_favorite);
  }

  function handleDelete(e: MouseEvent) {
    e.stopPropagation();
    deleteItem(item.id);
  }
</script>

<div
  class="group flex items-start gap-2 px-2.5 py-2 rounded-lg hover:bg-white/[0.03] transition-colors cursor-pointer"
  onclick={handleCopy}
  onmouseenter={() => (showActions = true)}
  onmouseleave={() => (showActions = false)}
  role="button"
  tabindex="0"
>
  <div class="flex-1 min-w-0">
    <div class="text-xs text-white/70 leading-relaxed break-all">{getPreview(item.content, 120)}</div>
    <div class="text-[10px] text-white/20 mt-1">{timeAgo(item.created_at)}</div>
  </div>

  <div class="flex items-center gap-0.5 shrink-0 {showActions || copied ? 'opacity-100' : 'opacity-0'} transition-opacity">
    {#if copied}
      <span class="text-[10px] text-accent-cyan px-1">已复制</span>
    {/if}
    <button
      onclick={handleFavorite}
      class="w-5 h-5 rounded flex items-center justify-center hover:bg-white/10 transition-colors"
      title={item.is_favorite ? "取消收藏" : "收藏"}
    >
      <svg class="w-3 h-3" style="color: {item.is_favorite ? '#ffd700' : 'rgba(255,255,255,0.3)'}" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill={item.is_favorite ? "currentColor" : "none"} stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon>
      </svg>
    </button>
    <button
      onclick={handleDelete}
      class="w-5 h-5 rounded flex items-center justify-center hover:bg-red-500/30 text-white/30 hover:text-red-400 transition-colors"
      title="删除"
    >
      <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <line x1="18" y1="6" x2="6" y2="18"></line>
        <line x1="6" y1="6" x2="18" y2="18"></line>
      </svg>
    </button>
  </div>
</div>
