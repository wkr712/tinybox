<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { get } from "svelte/store";
  import { activePanel, expandWindow, collapseWindow, minimized, restoreFromIsland, minimizeToIsland, minimizeToCat, restoreFromCat, expandIslandForLyrics, shrinkIslandFromLyrics } from "../../stores/app";
  import { currentSong, isPlaying, pauseMusic, resumeMusic, tracks, playSong, lyrics } from "../../stores/music";
  import IslandLyrics from "./IslandLyrics.svelte";
  import CatMascot from "./CatMascot.svelte";

  const panels = [
    {
      id: "notes", label: "便签",
      svg: `<path d="M20 2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h4l4 4 4-4h4c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-2 12H6v-2h12v2zm0-3H6V9h12v2zm0-3H6V6h12v2z"/>`
    },
    {
      id: "todo", label: "待办",
      svg: `<path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-9 14l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>`
    },
    {
      id: "clipboard", label: "剪贴板",
      svg: `<path d="M19 3h-2.18C16.4 1.84 15.3 1 14 1h-4c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm2 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z"/>`
    },
    {
      id: "dropzone", label: "文件暂存",
      svg: `<path d="M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM14 13v4h-4v-4H7l5-5 5 5h-3z"/>`
    },
    {
      id: "music", label: "音乐",
      svg: `<path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/>`
    },
    {
      id: "settings", label: "设置",
      svg: `<path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.07.62-.07.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/>`
    },
  ];

  let current = $state<string | null>(null);
  let isMinimized = $state(false);
  let song = $state<any>(null);
  let playing = $state(false);
  let lrc = $state<any[]>([]);
  let mounted = $state(false);
  let catClicked = $state(0);
  let unsubs: (() => void)[] = [];

  onMount(() => {
    unsubs.push(activePanel.subscribe((v) => (current = v)));
    unsubs.push(currentSong.subscribe((v) => (song = v)));
    unsubs.push(isPlaying.subscribe((v) => (playing = v)));
    unsubs.push(minimized.subscribe((v) => (isMinimized = v)));
    unsubs.push(lyrics.subscribe((v) => (lrc = v)));
    requestAnimationFrame(() => { mounted = true; });
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  $effect(() => {
    if (!mounted) return;
    if (isMinimized && song && playing && lrc.length > 0) {
      expandIslandForLyrics();
    } else if (isMinimized) {
      shrinkIslandFromLyrics();
    }
  });

  async function selectPanel(id: string) {
    if (current === id) {
      await collapseWindow();
    } else {
      await expandWindow(id);
    }
  }

  async function togglePlay(e: MouseEvent) {
    e.stopPropagation();
    if (playing) {
      await pauseMusic();
    } else {
      await resumeMusic();
    }
  }

  function openMusicPanel(e: MouseEvent) {
    e.stopPropagation();
    selectPanel("music");
  }

  async function islandPrev(e: MouseEvent) {
    e.stopPropagation();
    const s = get(currentSong);
    const t = get(tracks);
    if (!s || !t.length) return;
    const idx = t.findIndex((tr: any) => tr.id === s.id);
    if (idx > 0) await playSong(t[idx - 1]);
  }

  async function islandNext(e: MouseEvent) {
    e.stopPropagation();
    const s = get(currentSong);
    const t = get(tracks);
    if (!s || !t.length) return;
    const idx = t.findIndex((tr: any) => tr.id === s.id);
    if (idx < t.length - 1) await playSong(t[idx + 1]);
  }

  function handleCatClick() {
    catClicked++;
    if (catClicked >= 5) {
      catClicked = 0;
      minimizeToIsland();
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="sidebar-root {isMinimized ? 'cat-mode' : ''}"
  data-tauri-drag-region
>
  {#if isMinimized}
    <!-- Cat mascot minimized state -->
    <div class="cat-container" data-tauri-drag-region>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div class="cat-mascot-wrap" onclick={handleCatClick} role="button" tabindex="0">
        <CatMascot />
      </div>
      {#if song}
        <div class="cat-mini-controls">
          <button class="cat-ctrl-btn" onclick={islandPrev} aria-label="上一首">
            <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor"><polygon points="19 20 9 12 19 4"/><line x1="5" y1="19" x2="5" y2="5" stroke="currentColor" stroke-width="3"/></svg>
          </button>
          <button class="cat-ctrl-btn play" onclick={togglePlay} aria-label="播放暂停">
            {#if playing}
              <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
            {:else}
              <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg>
            {/if}
          </button>
          <button class="cat-ctrl-btn" onclick={islandNext} aria-label="下一首">
            <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 4 15 12 5 20"/><line x1="19" y1="5" x2="19" y2="19" stroke="currentColor" stroke-width="3"/></svg>
          </button>
        </div>
        <div class="cat-song-name">{song.name}</div>
      {/if}
      <button class="cat-restore" onclick={() => restoreFromCat()} aria-label="恢复">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
          <polyline points="15 18 9 12 15 6"></polyline>
        </svg>
      </button>
      {#if catClicked > 0}
        <div class="cat-click-hint">{5 - catClicked} 次点击进入小岛模式</div>
      {/if}
    </div>
  {:else}
    <div class="sidebar-inner">
      <!-- Top accent line -->
      <div class="sidebar-accent" data-tauri-drag-region></div>

      <!-- Main icons -->
      <div class="sidebar-icons">
        {#each panels.slice(0, 4) as panel, i (panel.id)}
          <button
            class="sidebar-icon {current === panel.id ? 'active' : ''} {mounted ? 'icon-enter' : ''}"
            style="transition-delay: {i * 50}ms"
            onclick={() => selectPanel(panel.id)}
            title={panel.label}
            aria-label={panel.label}
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
              {@html panel.svg}
            </svg>
          </button>
        {/each}
      </div>

      <div class="sidebar-divider"></div>

      <!-- Bottom icons (music + settings) -->
      <div class="sidebar-icons-bottom">
        {#each panels.slice(4) as panel, i (panel.id)}
          <button
            class="sidebar-icon {current === panel.id ? 'active' : ''} {mounted ? 'icon-enter' : ''}"
            style="transition-delay: {(i + 4) * 50}ms"
            onclick={() => selectPanel(panel.id)}
            title={panel.label}
            aria-label={panel.label}
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
              {@html panel.svg}
            </svg>
          </button>
        {/each}

        {#if song && current !== "music"}
          <button
            onclick={openMusicPanel}
            class="mini-player {mounted ? 'icon-enter' : ''}"
            title="{song.name} - {song.artists}"
            style="transition-delay: 350ms"
            aria-label="打开音乐面板"
          >
            <img src={(song.pic_url || '') + '?param=80y80'} alt="" class="mini-cover" />
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <div class="mini-overlay" role="button" tabindex="-1" onclick={togglePlay} aria-label="播放暂停">
              {#if playing}
                <svg class="w-3 h-3 text-white" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
              {:else}
                <svg class="w-3 h-3 text-white" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg>
              {/if}
            </div>
          </button>
        {/if}
      </div>

      <!-- Spacer pushes minimize to bottom -->
      <div class="sidebar-spacer" data-tauri-drag-region></div>

      <!-- Minimize to cat -->
      <button
        class="sidebar-minimize-btn"
        onclick={() => minimizeToCat()}
        title="最小化"
        aria-label="最小化"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
          <line x1="5" y1="12" x2="19" y2="12"/>
        </svg>
      </button>
    </div>
  {/if}
</div>

<style>
  .sidebar-root {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 0;
    width: 52px;
    flex-shrink: 0;
    height: 100%;
    position: relative;
    z-index: 2;
    transition: width 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .sidebar-root.cat-mode {
    width: 100%;
    height: 100%;
    padding: 0;
    justify-content: center;
    align-items: center;
  }

  .sidebar-inner {
    display: flex;
    flex-direction: column;
    align-items: center;
    height: 100%;
    width: 100%;
    padding: 10px 0 8px;
  }

  .sidebar-accent {
    width: 22px;
    height: 2px;
    border-radius: 1px;
    background: linear-gradient(90deg, var(--color-accent-primary), var(--color-accent-secondary));
    margin-bottom: 10px;
    opacity: 0.6;
    box-shadow: 0 0 6px color-mix(in srgb, var(--color-accent-primary) 15%, transparent);
  }

  .sidebar-icons {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

  .sidebar-divider {
    width: 20px;
    height: 1px;
    margin: 6px 0;
    background: linear-gradient(90deg, transparent, var(--color-border-default), transparent);
  }

  .sidebar-icons-bottom {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

  .sidebar-spacer {
    flex: 1;
    min-height: 8px;
  }

  .sidebar-minimize-btn {
    width: 28px;
    height: 28px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: var(--color-text-muted);
    cursor: pointer;
    transition: all 0.2s ease;
    margin-bottom: 2px;
  }

  .sidebar-minimize-btn:hover {
    color: var(--color-text-tertiary);
    background: var(--color-border-subtle);
  }

  .sidebar-minimize-btn:active {
    transform: scale(0.85);
  }

  /* Cat mascot minimized state */
  .cat-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    gap: 4px;
    padding: 8px 4px;
    animation: cat-appear 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  @keyframes cat-appear {
    0% { opacity: 0; transform: scale(0.6) rotate(-10deg); }
    60% { transform: scale(1.08) rotate(2deg); }
    100% { opacity: 1; transform: scale(1) rotate(0deg); }
  }

  .cat-mascot-wrap {
    width: 40px;
    height: 40px;
    flex-shrink: 0;
    cursor: pointer;
    transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .cat-mascot-wrap:hover {
    transform: scale(1.12);
  }

  .cat-mascot-wrap:active {
    transform: scale(0.9);
  }

  .cat-mini-controls {
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .cat-ctrl-btn {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: rgba(255, 255, 255, 0.3);
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all 0.2s;
  }

  .cat-ctrl-btn.play {
    width: 18px;
    height: 18px;
    color: rgba(255, 255, 255, 0.5);
    background: rgba(255, 255, 255, 0.06);
  }

  .cat-ctrl-btn:hover {
    color: color-mix(in srgb, var(--color-accent-primary) 80%, transparent);
    background: color-mix(in srgb, var(--color-accent-primary) 10%, transparent);
  }

  .cat-ctrl-btn:active {
    transform: scale(0.8);
  }

  .cat-song-name {
    font-size: 8px;
    color: rgba(255, 255, 255, 0.3);
    text-align: center;
    max-width: 48px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .cat-restore {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: rgba(255, 255, 255, 0.2);
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all 0.2s;
  }

  .cat-restore:hover {
    color: color-mix(in srgb, var(--color-accent-primary) 70%, transparent);
    background: color-mix(in srgb, var(--color-accent-primary) 6%, transparent);
  }

  .cat-restore:active {
    transform: scale(0.85);
  }

  .cat-click-hint {
    font-size: 7px;
    color: rgba(255, 255, 255, 0.15);
    position: absolute;
    bottom: 4px;
    text-align: center;
    animation: hint-fade 2s ease forwards;
  }

  @keyframes hint-fade {
    0% { opacity: 0; }
    20% { opacity: 1; }
    80% { opacity: 1; }
    100% { opacity: 0; }
  }

  /* Mini player in sidebar */
  .mini-player {
    position: relative;
    width: 38px;
    height: 38px;
    border-radius: 12px;
    overflow: hidden;
    flex-shrink: 0;
    border: 1px solid color-mix(in srgb, var(--color-accent-primary) 15%, transparent);
    box-shadow: 0 0 10px color-mix(in srgb, var(--color-accent-primary) 8%, transparent);
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .mini-player:hover {
    border-color: color-mix(in srgb, var(--color-accent-primary) 35%, transparent);
    box-shadow: 0 0 16px color-mix(in srgb, var(--color-accent-primary) 20%, transparent);
    transform: scale(1.05);
  }

  .mini-player:active {
    transform: scale(0.95);
  }

  .mini-cover {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .mini-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.35);
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: opacity 0.15s;
  }

  .mini-player:hover .mini-overlay {
    opacity: 1;
  }
</style>
