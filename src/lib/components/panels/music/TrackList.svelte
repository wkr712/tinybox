<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    tracks, currentSong, playSong, isPlaying,
    formatDuration, currentView, currentPlaylist, previousView,
  } from "../../../stores/music";

  let list = $state<any[]>([]);
  let song = $state<any>(null);
  let playing = $state(false);
  let pl = $state<any>(null);
  let unsubs: (() => void)[] = [];

  onMount(() => {
    unsubs.push(tracks.subscribe((v) => (list = v)));
    unsubs.push(currentSong.subscribe((v) => (song = v)));
    unsubs.push(isPlaying.subscribe((v) => (playing = v)));
    unsubs.push(currentPlaylist.subscribe((v) => (pl = v)));
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  async function handlePlay(track: any) {
    await playSong(track);
  }

  function showNowPlaying() {
    previousView.set("tracks");
    currentView.set("nowplaying");
  }
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-2 pb-3">
    <button onclick={() => currentView.set('playlists')} class="text-white/25 hover:text-white/55 active:scale-90 transition-all" aria-label="返回">
      <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
    </button>
    {#if pl}
      <div class="flex items-center gap-2">
        <img src={(pl.cover_img_url || '') + "?param=80y80"} alt="" class="w-6 h-6 rounded" />
        <span class="text-xs text-white/65 truncate max-w-[200px]">{pl.name}</span>
      </div>
    {/if}
  </div>

  <div class="flex-1 overflow-y-auto space-y-0.5">
    {#each list as track, i (track.id)}
      <button
        onclick={() => handlePlay(track)}
        class="track-row {song?.id === track.id ? 'active' : ''}"
      >
        <span class="track-idx">{i + 1}</span>
        <div class="flex-1 min-w-0">
          <div class="text-xs {song?.id === track.id ? 'text-accent-primary' : 'text-white/70'} truncate">{track.name}</div>
          <div class="text-[10px] text-white/20 truncate">{track.artists}</div>
        </div>
        <span class="text-[10px] text-white/12 shrink-0">{formatDuration(track.duration)}</span>
      </button>
    {/each}
  </div>

  {#if song}
    <div class="pt-2 border-t border-white/5">
      <button onclick={showNowPlaying} class="now-playing-bar">
        <img src={(song.pic_url || '') + "?param=80y80"} alt="" class="w-7 h-7 rounded" />
        <div class="flex-1 min-w-0">
          <div class="text-xs text-white/75 truncate">{song.name}</div>
          <div class="text-[10px] text-white/25 truncate">{song.artists}</div>
        </div>
        <div class="shrink-0">
          {#if playing}
            <svg class="w-4 h-4 text-accent-primary" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
          {:else}
            <svg class="w-4 h-4 text-white/35" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg>
          {/if}
        </div>
      </button>
    </div>
  {/if}
</div>

<style>
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

  .track-row.active {
    background: color-mix(in srgb, var(--color-accent-primary) 8%, transparent);
  }

  .track-row:active {
    transform: scale(0.995);
  }

  .track-idx {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.12);
    width: 18px;
    text-align: right;
    flex-shrink: 0;
  }

  .now-playing-bar {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px;
    border-radius: 8px;
    background: transparent;
    border: none;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .now-playing-bar:hover {
    background: rgba(255, 255, 255, 0.03);
  }
</style>
