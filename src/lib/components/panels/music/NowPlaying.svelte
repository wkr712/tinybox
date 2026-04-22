<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    currentSong, isPlaying, lyrics, playProgress, pauseMusic, resumeMusic,
    stopMusic, setVolume, volume, currentView, previousView, formatDuration,
    tracks, playSong,
  } from "../../../stores/music";
  import { get } from "svelte/store";

  let song = $state<any>(null);
  let playing = $state(false);
  let lrc = $state<any[]>([]);
  let progress = $state(0);
  let vol = $state(0.8);
  let trackList = $state<any[]>([]);
  let unsubs: (() => void)[] = [];

  let lyricsContainer: HTMLDivElement | undefined = $state();

  let currentIdx = $derived(
    song ? trackList.findIndex((t) => t.id === song.id) : -1
  );

  // Find the current lyric line index based on progress
  let activeLyricIdx = $derived.by(() => {
    for (let i = lrc.length - 1; i >= 0; i--) {
      if (progress >= lrc[i].time) return i;
    }
    return -1;
  });

  // Auto-scroll lyrics to active line
  $effect(() => {
    if (activeLyricIdx >= 0 && lyricsContainer) {
      const lineEl = lyricsContainer.querySelector(`[data-idx="${activeLyricIdx}"]`);
      if (lineEl) {
        lineEl.scrollIntoView({ behavior: "smooth", block: "center" });
      }
    }
  });

  // Progress bar as percentage of duration
  let progressPercent = $derived(
    song && song.duration > 0 ? Math.min((progress * 1000) / song.duration * 100, 100) : 0
  );

  onMount(() => {
    unsubs.push(currentSong.subscribe((v) => (song = v)));
    unsubs.push(isPlaying.subscribe((v) => (playing = v)));
    unsubs.push(lyrics.subscribe((v) => (lrc = v)));
    unsubs.push(playProgress.subscribe((v) => (progress = v)));
    unsubs.push(volume.subscribe((v) => (vol = v)));
    unsubs.push(tracks.subscribe((v) => (trackList = v)));
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  async function togglePlay() {
    if (playing) {
      await pauseMusic();
    } else {
      await resumeMusic();
    }
  }

  async function playNext() {
    if (currentIdx < trackList.length - 1) {
      await playSong(trackList[currentIdx + 1]);
    }
  }

  async function playPrev() {
    if (currentIdx > 0) {
      await playSong(trackList[currentIdx - 1]);
    }
  }

  function handleVolumeChange(e: Event) {
    const target = e.target as HTMLInputElement;
    const v = parseFloat(target.value);
    setVolume(v);
  }
</script>

<div class="h-full flex flex-col">
  <!-- Header -->
  <div class="flex items-center gap-2 pb-3">
    <button onclick={() => currentView.set(get(previousView) as any || 'tracks')} class="text-white/30 hover:text-white/60">
      <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
    </button>
    <span class="text-xs text-white/40">正在播放</span>
  </div>

  {#if song}
    <!-- Compact cover + controls row -->
    <div class="flex items-center gap-3 pb-3">
      <div class="w-12 h-12 rounded-lg overflow-hidden shrink-0 shadow-md shadow-black/30">
        <img src={song.pic_url + '?param=200y200'} alt="" class="w-full h-full object-cover" />
      </div>
      <div class="flex-1 min-w-0">
        <div class="text-sm text-white/90 truncate">{song.name}</div>
        <div class="text-[10px] text-white/30 mt-0.5">{song.artists}</div>
      </div>
    </div>

    <!-- Progress bar -->
    <div class="px-1 pb-2">
      <div class="h-0.5 bg-white/10 rounded-full overflow-hidden">
        <div class="h-full bg-accent-cyan/60 rounded-full transition-all duration-200" style="width: {progressPercent}%"></div>
      </div>
      <div class="flex justify-between mt-1">
        <span class="text-[9px] text-white/20">{formatDuration(progress * 1000)}</span>
        <span class="text-[9px] text-white/20">{formatDuration(song.duration)}</span>
      </div>
    </div>

    <!-- Controls -->
    <div class="flex items-center justify-center gap-3 pb-2">
      <button onclick={playPrev} class="w-7 h-7 rounded-full flex items-center justify-center text-white/40 hover:text-white/70 transition-colors">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor"><polygon points="19 20 9 12 19 4 19 20"/><line x1="5" y1="19" x2="5" y2="5" stroke="currentColor" stroke-width="2"/></svg>
      </button>
      <button onclick={togglePlay} class="w-9 h-9 rounded-full bg-accent-cyan/20 flex items-center justify-center text-accent-cyan hover:bg-accent-cyan/30 transition-colors">
        {#if playing}
          <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
        {:else}
          <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg>
        {/if}
      </button>
      <button onclick={playNext} class="w-7 h-7 rounded-full flex items-center justify-center text-white/40 hover:text-white/70 transition-colors">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 4 15 12 5 20 5 4"/><line x1="19" y1="5" x2="19" y2="19" stroke="currentColor" stroke-width="2"/></svg>
      </button>
      <button onclick={stopMusic} class="w-6 h-6 rounded-full flex items-center justify-center text-white/25 hover:text-red-400 transition-colors ml-1">
        <svg class="w-3 h-3" viewBox="0 0 24 24" fill="currentColor"><rect x="4" y="4" width="16" height="16" rx="2"/></svg>
      </button>
    </div>

    <!-- Volume -->
    <div class="flex items-center gap-2 px-3 pb-2">
      <svg class="w-3 h-3 text-white/15 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M19.07 4.93a10 10 0 010 14.14"/></svg>
      <input
        type="range"
        min="0"
        max="1"
        step="0.05"
        value={vol}
        oninput={handleVolumeChange}
        class="flex-1 h-1 appearance-none bg-white/10 rounded-full outline-none accent-accent-cyan [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-2 [&::-webkit-slider-thumb]:h-2 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-accent-cyan"
      />
    </div>

    <!-- Synced Lyrics -->
    {#if lrc.length > 0}
      <div class="text-[9px] text-white/15 px-1 pb-1">歌词</div>
      <div bind:this={lyricsContainer} class="flex-1 overflow-y-auto px-1 scroll-smooth" style="mask-image: linear-gradient(transparent 0%, black 15%, black 85%, transparent 100%);">
        <div class="py-8">
          {#each lrc as line, i (line.time)}
            <div
              data-idx={i}
              class="text-[11px] py-1 leading-relaxed transition-all duration-300 {i === activeLyricIdx
                ? 'text-accent-cyan scale-[1.02] origin-left'
                : i < activeLyricIdx
                  ? 'text-white/15'
                  : 'text-white/35'}"
            >
              {line.text}
            </div>
          {/each}
        </div>
      </div>
    {:else}
      <div class="flex-1 flex items-center justify-center">
        <span class="text-[10px] text-white/15">暂无歌词</span>
      </div>
    {/if}
  {:else}
    <div class="flex-1 flex flex-col items-center justify-center gap-2">
      <svg class="w-8 h-8 text-white/10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><polygon points="10 8 16 12 10 16 10 8"/></svg>
      <span class="text-xs text-white/20">未在播放</span>
    </div>
  {/if}
</div>
