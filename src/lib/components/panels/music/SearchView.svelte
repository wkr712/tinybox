<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    searchResults, searchSongs, playSong, formatDuration,
    currentView, previousView, hotSearches, fetchHotSearches,
  } from "../../../stores/music";
  import type { MusicView } from "../../../types/music";

  let query = $state("");
  let results = $state<any[]>([]);
  let hot = $state<any[]>([]);
  let prev = $state<MusicView>("playlists");
  let unsubs: (() => void)[] = [];

  onMount(() => {
    unsubs.push(searchResults.subscribe((v) => (results = v)));
    unsubs.push(hotSearches.subscribe((v) => (hot = v)));
    unsubs.push(previousView.subscribe((v) => (prev = v)));

    if (hot.length === 0) fetchHotSearches();
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  function handleSearch() {
    if (query.trim()) {
      searchSongs(query.trim());
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") handleSearch();
  }

  function handleHotClick(word: string) {
    query = word;
    searchSongs(word);
  }
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-2 pb-3">
    <button onclick={() => currentView.set(prev || 'playlists')} class="text-white/25 hover:text-white/55 active:scale-90 transition-all" aria-label="返回">
      <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
    </button>
    <div class="flex-1 search-box">
      <input
        type="text"
        bind:value={query}
        onkeydown={handleKeydown}
        placeholder="搜索歌曲..."
        class="search-input"
      />
      <button onclick={handleSearch} class="search-btn" aria-label="搜索">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
      </button>
    </div>
  </div>

  <div class="flex-1 overflow-y-auto">
    {#if results.length > 0}
      <div class="space-y-0.5">
        {#each results as track (track.id)}
          <button
            onclick={() => playSong(track)}
            class="track-row"
          >
            <div class="flex-1 min-w-0">
              <div class="text-xs text-white/70 truncate">{track.name}</div>
              <div class="text-[10px] text-white/20 truncate">{track.artists} · {track.album}</div>
            </div>
            <span class="text-[10px] text-white/12 shrink-0">{formatDuration(track.duration)}</span>
          </button>
        {/each}
      </div>
    {:else if hot.length > 0}
      <div class="px-1">
        <div class="text-[10px] text-white/18 mb-2">热搜榜</div>
        <div class="space-y-0.5">
          {#each hot.slice(0, 15) as h, i (h.search_word)}
            <button
              onclick={() => handleHotClick(h.search_word)}
              class="track-row"
            >
              <span class="text-[10px] w-5 text-right shrink-0 {i < 3 ? 'text-accent-primary/70' : 'text-white/12'}">{i + 1}</span>
              <div class="flex-1 min-w-0">
                <div class="text-[11px] text-white/55 truncate">{h.search_word}</div>
                {#if h.content}
                  <div class="text-[9px] text-white/12 truncate">{h.content}</div>
                {/if}
              </div>
            </button>
          {/each}
        </div>
      </div>
    {:else}
      <div class="text-center text-white/12 text-xs py-8">输入关键词搜索歌曲</div>
    {/if}
  </div>
</div>

<style>
  .search-box {
    position: relative;
  }

  .search-input {
    width: 100%;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    font-size: 12px;
    color: rgba(255, 255, 255, 0.8);
    padding: 6px 32px 6px 12px;
    border-radius: 8px;
    outline: none;
    transition: all 0.2s ease;
  }

  .search-input::placeholder {
    color: rgba(255, 255, 255, 0.2);
  }

  .search-input:focus {
    border-color: color-mix(in srgb, var(--color-accent-primary) 30%, transparent);
    background: rgba(255, 255, 255, 0.06);
  }

  .search-btn {
    position: absolute;
    right: 8px;
    top: 50%;
    transform: translateY(-50%);
    color: rgba(255, 255, 255, 0.25);
    transition: color 0.15s ease;
    background: none;
    border: none;
    cursor: pointer;
  }

  .search-btn:hover {
    color: rgba(255, 255, 255, 0.55);
  }

  .track-row {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 8px;
    border-radius: 6px;
    text-align: left;
    background: transparent;
    border: none;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .track-row:hover {
    background: rgba(255, 255, 255, 0.03);
  }

  .track-row:active {
    transform: scale(0.995);
  }
</style>
