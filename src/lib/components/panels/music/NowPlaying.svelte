<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    currentSong, isPlaying, lyrics, playProgress, pauseMusic, resumeMusic,
    stopMusic, setVolume, volume, currentView, previousView, formatDuration,
    tracks, playSong, seekTo,
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
  let progressTrackEl: HTMLDivElement | undefined = $state();
  let progressDragging = $state(false);

  let currentIdx = $derived(
    song ? trackList.findIndex((t) => t.id === song.id) : -1
  );

  let activeLyricIdx = $derived.by(() => {
    for (let i = lrc.length - 1; i >= 0; i--) {
      if (progress >= lrc[i].time) return i;
    }
    return -1;
  });

  let lastScrolledIdx = $state(-1);
  $effect(() => {
    if (activeLyricIdx >= 0 && activeLyricIdx !== lastScrolledIdx && lyricsContainer) {
      lastScrolledIdx = activeLyricIdx;
      const lineEl = lyricsContainer.querySelector(`[data-idx="${activeLyricIdx}"]`);
      if (lineEl) {
        lineEl.scrollIntoView({ behavior: "smooth", block: "center" });
      }
    }
  });

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
    cleanupProgressListeners();
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

  // Progress bar seek
  function seekFromEvent(e: MouseEvent) {
    if (!progressTrackEl || !song || !song.duration) return;
    const rect = progressTrackEl.getBoundingClientRect();
    const pct = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    const targetSeconds = pct * (song.duration / 1000);
    seekTo(targetSeconds);
  }

  function handleProgressDown(e: MouseEvent) {
    e.preventDefault();
    progressDragging = true;
    seekFromEvent(e);
    window.addEventListener("mousemove", handleProgressMove);
    window.addEventListener("mouseup", handleProgressUp);
  }

  function handleProgressMove(e: MouseEvent) {
    if (progressDragging) seekFromEvent(e);
  }

  function handleProgressUp() {
    progressDragging = false;
    cleanupProgressListeners();
  }

  function cleanupProgressListeners() {
    window.removeEventListener("mousemove", handleProgressMove);
    window.removeEventListener("mouseup", handleProgressUp);
  }
</script>

<div class="h-full flex flex-col">
  <!-- Header -->
  <div class="flex items-center gap-2 pb-3">
    <button onclick={() => currentView.set(get(previousView) as any || 'tracks')} class="text-white/30 hover:text-white/60" aria-label="返回">
      <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
    </button>
    <span class="text-xs text-white/40">正在播放</span>
  </div>

  {#if song}
    <!-- Compact cover + info -->
    <div class="flex items-center gap-3 pb-3">
      <div class="w-12 h-12 rounded-full overflow-hidden shrink-0 shadow-md shadow-black/30">
        <img
          src={(song.pic_url || '') + '?param=200y200'}
          alt=""
          class="w-full h-full object-cover album-spin {playing ? '' : 'paused'}"
        />
      </div>
      <div class="flex-1 min-w-0">
        <div class="text-sm text-white/90 truncate">{song.name}</div>
        <div class="text-[10px] text-white/30 mt-0.5">{song.artists}</div>
      </div>
    </div>

    <!-- Progress bar (interactive) -->
    <div class="px-1 pb-2">
      <div
        bind:this={progressTrackEl}
        class="progress-track"
        role="slider"
        tabindex="0"
        aria-label="播放进度"
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={Math.round(progressPercent)}
        onmousedown={handleProgressDown}
      >
        <div class="h-full bg-accent-primary/60 rounded-full {playing && !progressDragging ? 'progress-playing' : ''}" style="width: {progressPercent}%"></div>
      </div>
      <div class="flex justify-between mt-1">
        <span class="text-[9px] text-white/20">{formatDuration(progress * 1000)}</span>
        <span class="text-[9px] text-white/20">{formatDuration(song.duration)}</span>
      </div>
    </div>

    <!-- Controls -->
    <div class="flex items-center justify-center gap-3 pb-2">
      <button onclick={playPrev} class="w-7 h-7 rounded-full flex items-center justify-center text-white/40 hover:text-white/70 transition-all active:scale-90" aria-label="上一首">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor"><polygon points="19 20 9 12 19 4 19 20"/><line x1="5" y1="19" x2="5" y2="5" stroke="currentColor" stroke-width="2"/></svg>
      </button>
      <button onclick={togglePlay} class="w-9 h-9 rounded-full bg-accent-primary/20 flex items-center justify-center text-accent-primary hover:bg-accent-primary/30 transition-all active:scale-90" aria-label={playing ? "暂停" : "播放"}>
        {#if playing}
          <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
        {:else}
          <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg>
        {/if}
      </button>
      <button onclick={playNext} class="w-7 h-7 rounded-full flex items-center justify-center text-white/40 hover:text-white/70 transition-all active:scale-90" aria-label="下一首">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 4 15 12 5 20 5 4"/><line x1="19" y1="5" x2="19" y2="19" stroke="currentColor" stroke-width="2"/></svg>
      </button>
      <button onclick={stopMusic} class="w-6 h-6 rounded-full flex items-center justify-center text-white/25 hover:text-red-400 transition-all active:scale-90 ml-1" aria-label="停止">
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
        class="flex-1 h-1 appearance-none bg-white/10 rounded-full outline-none accent-accent-primary [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-2 [&::-webkit-slider-thumb]:h-2 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-accent-primary"
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
              class="text-[11px] py-1 leading-relaxed transition-all duration-500 {i === activeLyricIdx
                ? 'text-accent-primary scale-[1.02] origin-left'
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

<style>
  .progress-track {
    height: 4px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 9999px;
    overflow: hidden;
    cursor: pointer;
    transition: height 0.15s ease;
  }

  .progress-track:hover {
    height: 6px;
  }

  .progress-playing {
    animation: pulse-glow 2s ease-in-out infinite;
  }

  .album-spin {
    animation: subtle-spin 20s linear infinite;
  }

  .album-spin.paused {
    animation-play-state: paused;
  }

  @keyframes pulse-glow {
    0%, 100% { box-shadow: none; }
    50% { box-shadow: 0 0 8px color-mix(in srgb, var(--color-accent-primary) 30%, transparent); }
  }

  @keyframes subtle-spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
</style>
