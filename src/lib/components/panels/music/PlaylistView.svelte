<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    playlists, fetchUserPlaylists, fetchPlaylistTracks,
    currentView, currentPlaylist, user,
  } from "../../../stores/music";

  let list = $state<any[]>([]);
  let u = $state<any>(null);
  let unsubs: (() => void)[] = [];

  onMount(() => {
    unsubs.push(playlists.subscribe((v) => (list = v)));
    unsubs.push(user.subscribe((v) => (u = v)));
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
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-3 pb-3">
    {#if u}
      <img src={u.avatar_url} alt="" class="w-7 h-7 rounded-full" />
      <span class="text-white/70 text-xs">{u.nickname}</span>
    {/if}
  </div>

  <div class="flex-1 overflow-y-auto space-y-1">
    {#each list as pl (pl.id)}
      <button
        onclick={() => openPlaylist(pl)}
        class="w-full flex items-center gap-2.5 px-2 py-2 rounded-lg hover:bg-white/[0.04] transition-colors text-left"
      >
        <img
          src={pl.cover_img_url + "?param=80y80"}
          alt=""
          class="w-9 h-9 rounded object-cover shrink-0"
          loading="lazy"
        />
        <div class="flex-1 min-w-0">
          <div class="text-xs text-white/80 truncate">{pl.name}</div>
          <div class="text-[10px] text-white/25 mt-0.5">{pl.track_count} 首</div>
        </div>
      </button>
    {/each}
  </div>
</div>
