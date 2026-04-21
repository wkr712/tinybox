<script lang="ts">
  import { searchQuery, searchResults, searchSongs, playSong, formatDuration } from "../../../stores/music";
  import { currentView } from "../../../stores/music";

  let query = $state("");
  searchQuery.subscribe((v) => (query = v));

  let results: any[] = [];
  searchResults.subscribe((v) => (results = v));

  function handleSearch() {
    if (query.trim()) {
      searchSongs(query.trim());
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") handleSearch();
  }
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-2 pb-3">
    <button onclick={() => currentView.set('playlists')} class="text-white/30 hover:text-white/60">
      <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
    </button>
    <div class="flex-1 relative">
      <input
        type="text"
        bind:value={query}
        onkeydown={handleKeydown}
        placeholder="搜索歌曲..."
        class="w-full bg-white/[0.05] text-xs text-white/80 placeholder:text-white/20 px-3 py-1.5 rounded-lg outline-none focus:ring-1 focus:ring-accent-cyan/30"
      />
      <button onclick={handleSearch} class="absolute right-2 top-1/2 -translate-y-1/2 text-white/30 hover:text-white/60">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
      </button>
    </div>
  </div>

  <div class="flex-1 overflow-y-auto space-y-0.5">
    {#each results as track (track.id)}
      <button
        onclick={() => playSong(track)}
        class="w-full flex items-center gap-2.5 px-2 py-1.5 rounded-lg hover:bg-white/[0.03] transition-colors text-left"
      >
        <div class="flex-1 min-w-0">
          <div class="text-xs text-white/75 truncate">{track.name}</div>
          <div class="text-[10px] text-white/25 truncate">{track.artists} · {track.album}</div>
        </div>
        <span class="text-[10px] text-white/15 shrink-0">{formatDuration(track.duration)}</span>
      </button>
    {/each}

    {#if results.length === 0 && query}
      <div class="text-center text-white/15 text-xs py-8">输入关键词搜索歌曲</div>
    {/if}
  </div>
</div>
