<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    user, recommendPlaylists, recommendSongs, hotSearches,
    fetchRecommendPlaylists, fetchRecommendSongs, fetchHotSearches,
    fetchPlaylistTracks, playSong, currentView, searchSongs, formatDuration,
  } from "../../../stores/music";

  let rpl = $state<any[]>([]);
  let rsongs = $state<any[]>([]);
  let hot = $state<any[]>([]);
  let unsubs: (() => void)[] = [];

  onMount(async () => {
    unsubs.push(recommendPlaylists.subscribe((v) => (rpl = v)));
    unsubs.push(recommendSongs.subscribe((v) => (rsongs = v)));
    unsubs.push(hotSearches.subscribe((v) => (hot = v)));

    await Promise.all([
      fetchRecommendPlaylists(),
      fetchRecommendSongs(),
      fetchHotSearches(),
    ]);
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  function openPlaylist(pl: any) {
    fetchPlaylistTracks(pl.id);
    currentView.set("tracks");
  }

  function handleHotSearch(word: string) {
    searchSongs(word);
    currentView.set("search");
  }
</script>

<div class="h-full flex flex-col gap-4 overflow-y-auto">
  <!-- Hot searches -->
  {#if hot.length > 0}
    <div>
      <div class="text-[10px] text-white/20 mb-2 px-1">热搜</div>
      <div class="flex flex-wrap gap-1.5">
        {#each hot.slice(0, 10) as h, i (h.search_word)}
          <button
            onclick={() => handleHotSearch(h.search_word)}
            class="px-2 py-0.5 rounded-full bg-white/[0.04] text-[10px] text-white/40 hover:bg-accent-cyan/10 hover:text-accent-cyan transition-colors"
          >
            <span class="text-white/15 mr-1">{i + 1}</span>{h.search_word}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Recommend playlists -->
  {#if rpl.length > 0}
    <div>
      <div class="text-[10px] text-white/20 mb-2 px-1">推荐歌单</div>
      <div class="grid grid-cols-3 gap-2">
        {#each rpl.slice(0, 6) as pl (pl.id)}
          <button
            onclick={() => openPlaylist(pl)}
            class="flex flex-col gap-1 text-left"
          >
            <div class="aspect-square rounded-lg overflow-hidden">
              <img
                src={pl.cover_img_url + "?param=200y200"}
                alt=""
                class="w-full h-full object-cover"
                loading="lazy"
              />
            </div>
            <div class="text-[9px] text-white/40 leading-tight line-clamp-2">{pl.name}</div>
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Daily recommend songs -->
  {#if rsongs.length > 0}
    <div>
      <div class="text-[10px] text-white/20 mb-2 px-1">每日推荐</div>
      <div class="space-y-0.5">
        {#each rsongs.slice(0, 12) as track (track.id)}
          <button
            onclick={() => playSong(track)}
            class="w-full flex items-center gap-2 px-1 py-1 rounded-lg hover:bg-white/[0.03] transition-colors text-left"
          >
            <div class="flex-1 min-w-0">
              <div class="text-[11px] text-white/70 truncate">{track.name}</div>
              <div class="text-[9px] text-white/25 truncate">{track.artists}</div>
            </div>
            <span class="text-[9px] text-white/15 shrink-0">{formatDuration(track.duration)}</span>
          </button>
        {/each}
      </div>
    </div>
  {/if}

  {#if rpl.length === 0 && rsongs.length === 0 && hot.length === 0}
    <div class="flex-1 flex items-center justify-center py-12">
      <span class="text-xs text-white/15">加载中...</span>
    </div>
  {/if}
</div>
