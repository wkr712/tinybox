<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    user, recommendPlaylists, recommendSongs, hotSearches,
    fetchRecommendPlaylists, fetchRecommendSongs, fetchHotSearches,
    fetchPlaylistTracks, playSong, currentView, currentPlaylist, searchSongs, formatDuration,
  } from "../../../stores/music";
  import SkeletonLine from "../../shared/SkeletonLine.svelte";

  let rpl = $state<any[]>([]);
  let rsongs = $state<any[]>([]);
  let hot = $state<any[]>([]);
  let contentLoading = $state(true);
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
    contentLoading = false;
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  function openPlaylist(pl: any) {
    currentPlaylist.set(pl);
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
            class="px-2 py-0.5 rounded-full bg-white/[0.04] text-[10px] text-white/40 hover:bg-accent-cyan/10 hover:text-accent-cyan active:scale-95 transition-all"
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
            class="flex flex-col gap-1 text-left active:scale-95 transition-transform"
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
            class="w-full flex items-center gap-2 px-1 py-1 rounded-lg hover:bg-white/[0.03] active:scale-[0.99] transition-all text-left"
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

  {#if contentLoading}
    <div class="space-y-4">
      <div class="flex flex-wrap gap-1.5">
        {#each { length: 6 } as _}
          <SkeletonLine width="60px" height="20px" rounded="999px" />
        {/each}
      </div>
      <div class="grid grid-cols-3 gap-2">
        {#each { length: 6 } as _}
          <div class="space-y-1">
            <SkeletonLine width="100%" height="auto" rounded="8px" style="aspect-ratio: 1;" />
            <SkeletonLine width="80%" height="9px" />
          </div>
        {/each}
      </div>
      {#each { length: 5 } as _}
        <div class="flex items-center gap-2 px-1 py-1">
          <div class="flex-1 space-y-1">
            <SkeletonLine width="60%" height="11px" />
            <SkeletonLine width="40%" height="9px" />
          </div>
        </div>
      {/each}
    </div>
  {:else if rpl.length === 0 && rsongs.length === 0 && hot.length === 0}
    <div class="flex-1 flex items-center justify-center py-12">
      <span class="text-xs text-white/15">加载中...</span>
    </div>
  {/if}
</div>
