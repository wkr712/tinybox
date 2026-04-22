<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    tracks, currentSong, playSong, isPlaying,
    formatDuration, currentView, currentPlaylist, lyrics,
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
    currentView.set("nowplaying");
  }
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-2 pb-3">
    <button onclick={() => currentView.set('playlists')} class="text-white/30 hover:text-white/60">
      <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
    </button>
    {#if pl}
      <div class="flex items-center gap-2">
        <img src={pl.cover_img_url + "?param=80y80"} alt="" class="w-6 h-6 rounded" />
        <span class="text-xs text-white/70 truncate max-w-[200px]">{pl.name}</span>
      </div>
    {/if}
  </div>

  <div class="flex-1 overflow-y-auto space-y-0.5">
    {#each list as track, i (track.id)}
      <button
        onclick={() => handlePlay(track)}
        class="w-full flex items-center gap-2.5 px-2 py-1.5 rounded-lg transition-colors text-left {song?.id === track.id ? 'bg-accent-cyan/10' : 'hover:bg-white/[0.03]'}"
      >
        <span class="text-[10px] text-white/15 w-5 text-right shrink-0">{i + 1}</span>
        <div class="flex-1 min-w-0">
          <div class="text-xs {song?.id === track.id ? 'text-accent-cyan' : 'text-white/75'} truncate">{track.name}</div>
          <div class="text-[10px] text-white/25 truncate">{track.artists}</div>
        </div>
        <span class="text-[10px] text-white/15 shrink-0">{formatDuration(track.duration)}</span>
      </button>
    {/each}
  </div>

  {#if song}
    <div class="pt-2 border-t border-white/5">
      <button onclick={showNowPlaying} class="w-full flex items-center gap-2.5 px-2 py-2 rounded-lg hover:bg-white/[0.04]">
        <img src={song.pic_url + "?param=80y80"} alt="" class="w-8 h-8 rounded" />
        <div class="flex-1 min-w-0">
          <div class="text-xs text-white/80 truncate">{song.name}</div>
          <div class="text-[10px] text-white/30 truncate">{song.artists}</div>
        </div>
        <div class="shrink-0">
          {#if playing}
            <svg class="w-4 h-4 text-accent-cyan" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
          {:else}
            <svg class="w-4 h-4 text-white/40" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg>
          {/if}
        </div>
      </button>
    </div>
  {/if}
</div>
