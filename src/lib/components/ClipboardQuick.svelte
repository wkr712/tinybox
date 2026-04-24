<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { clipboardHistory, clipboardQuickOpen, copyToClipboard, getPreview, timeAgo } from "../stores/clipboard";
  import { get } from "svelte/store";
  import { fly } from "svelte/transition";
  import type { ClipboardItem } from "../types/clipboard";

  let items = $state<ClipboardItem[]>([]);
  let selectedIdx = $state(0);
  let unsubs: (() => void)[] = [];

  onMount(() => {
    unsubs.push(clipboardHistory.subscribe((v) => {
      items = v.slice(0, 15);
    }));
    unsubs.push(clipboardQuickOpen.subscribe((open) => {
      if (open) {
        selectedIdx = 0;
        items = get(clipboardHistory).slice(0, 15);
      }
    }));
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  function selectItem(idx: number) {
    if (idx >= 0 && idx < items.length) {
      copyToClipboard(items[idx].content);
      clipboardQuickOpen.set(false);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      selectedIdx = Math.min(selectedIdx + 1, items.length - 1);
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      selectedIdx = Math.max(selectedIdx - 1, 0);
    } else if (e.key === "Enter") {
      e.preventDefault();
      selectItem(selectedIdx);
    } else if (e.key === "Escape") {
      e.preventDefault();
      clipboardQuickOpen.set(false);
    } else if (e.key >= "1" && e.key <= "9") {
      e.preventDefault();
      selectItem(parseInt(e.key) - 1);
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if items.length > 0}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="cq-overlay" transition:fly={{ y: 8, duration: 150 }}>
    <div class="text-[9px] text-white/15 px-3 py-1.5 border-b border-white/5">剪贴板历史</div>
    <div class="cq-list">
      {#each items as item, i (item.id)}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div
          class="cq-item {i === selectedIdx ? 'active' : ''}"
          onclick={() => selectItem(i)}
          onmouseenter={() => (selectedIdx = i)}
          tabindex="-1"
          role="option"
          aria-selected={i === selectedIdx}
        >
          <span class="cq-idx">{i < 9 ? i + 1 : ''}</span>
          <div class="flex-1 min-w-0">
            <div class="text-[11px] text-white/70 truncate">{getPreview(item.content, 60)}</div>
          </div>
          <span class="cq-time">{timeAgo(item.created_at)}</span>
        </div>
      {/each}
    </div>
    <div class="cq-footer">
      ↑↓ 切换 · Enter 确认 · 1-9 快选 · Esc 关闭
    </div>
  </div>
{/if}

<style>
  .cq-overlay {
    position: absolute;
    inset: 0;
    z-index: 100;
    display: flex;
    flex-direction: column;
    background: rgba(14, 14, 24, 0.95);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border-radius: 12px;
    overflow: hidden;
  }

  .cq-list {
    flex: 1;
    overflow-y: auto;
    padding: 2px 0;
  }

  .cq-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 12px;
    cursor: pointer;
    transition: background 0.1s ease;
  }

  .cq-item.active {
    background: color-mix(in srgb, var(--color-accent-primary) 12%, transparent);
  }

  .cq-item.active .cq-idx {
    color: var(--color-accent-primary);
  }

  .cq-idx {
    width: 16px;
    text-align: center;
    font-size: 9px;
    color: rgba(255, 255, 255, 0.15);
    flex-shrink: 0;
    font-weight: 500;
  }

  .cq-time {
    font-size: 9px;
    color: rgba(255, 255, 255, 0.15);
    flex-shrink: 0;
  }

  .cq-footer {
    padding: 4px 12px;
    border-top: 1px solid rgba(255, 255, 255, 0.05);
    text-align: center;
    font-size: 9px;
    color: rgba(255, 255, 255, 0.12);
  }
</style>
