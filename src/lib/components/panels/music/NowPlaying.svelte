<script lang="ts">
  import {
    currentSong, isPlaying, lyrics, pauseMusic, resumeMusic,
    stopMusic, setVolume, volume, currentView, formatDuration,
    tracks, playSong,
  } from "../../../stores/music";

  let song: any = null;
  currentSong.subscribe((v) => (song = v));

  let playing = $state(false);
  isPlaying.subscribe((v) => (playing = v));

  let lrc: any[] = [];
  lyrics.subscribe((v) => (lrc = v));

  let vol = $state(0.8);
  volume.subscribe((v) => (vol = v));

  let trackList: any[] = [];
  tracks.subscribe((v) => (trackList = v));

  let currentIdx = $derived(
    song ? trackList.findIndex((t) => t.id === song.id) : -1
  );

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
    <button onclick={() => currentView.set('tracks')} class="text-white/30 hover:text-white/60">
      <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
    </button>
    <span class="text-xs text-white/40">正在播放</span>
  </div>

  {#if song}
    <!-- Cover + Info -->
    <div class="flex flex-col items-center gap-3 pb-4">
      <div class="w-28 h-28 rounded-xl overflow-hidden shadow-lg shadow-black/40">
        <img src={song.pic_url + "?param=300y300"} alt="" class="w-full h-full object-cover" />
      </div>
      <div class="text-center">
        <div class="text-sm text-white/90">{song.name}</div>
        <div class="text-[10px] text-white/30 mt-0.5">{song.artists} · {formatDuration(song.duration)}</div>
      </div>
    </div>

    <!-- Controls -->
    <div class="flex items-center justify-center gap-4 pb-3">
      <button onclick={playPrev} class="w-8 h-8 rounded-full flex items-center justify-center text-white/40 hover:text-white/70 transition-colors">
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor"><polygon points="19 20 9 12 19 4 19 20"/><line x1="5" y1="19" x2="5" y2="5" stroke="currentColor" stroke-width="2"/></svg>
      </button>
      <button onclick={togglePlay} class="w-10 h-10 rounded-full bg-accent-cyan/20 flex items-center justify-center text-accent-cyan hover:bg-accent-cyan/30 transition-colors">
        {#if playing}
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
        {:else}
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg>
        {/if}
      </button>
      <button onclick={playNext} class="w-8 h-8 rounded-full flex items-center justify-center text-white/40 hover:text-white/70 transition-colors">
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 4 15 12 5 20 5 4"/><line x1="19" y1="5" x2="19" y2="19" stroke="currentColor" stroke-width="2"/></svg>
      </button>
      <button onclick={stopMusic} class="w-7 h-7 rounded-full flex items-center justify-center text-white/30 hover:text-red-400 transition-colors">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor"><rect x="4" y="4" width="16" height="16" rx="2"/></svg>
      </button>
    </div>

    <!-- Volume -->
    <div class="flex items-center gap-2 px-4 pb-3">
      <svg class="w-3 h-3 text-white/20 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M19.07 4.93a10 10 0 010 14.14"/></svg>
      <input
        type="range"
        min="0"
        max="1"
        step="0.05"
        value={vol}
        oninput={handleVolumeChange}
        class="flex-1 h-1 appearance-none bg-white/10 rounded-full outline-none accent-accent-cyan [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-2.5 [&::-webkit-slider-thumb]:h-2.5 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-accent-cyan"
      />
    </div>

    <!-- Lyrics -->
    {#if lrc.length > 0}
      <div class="flex-1 overflow-y-auto px-2">
        <div class="text-[10px] text-white/20 mb-2">歌词</div>
        {#each lrc as line (line.time)}
          <div class="text-[11px] text-white/30 py-0.5 leading-relaxed">
            {line.text}
          </div>
        {/each}
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
