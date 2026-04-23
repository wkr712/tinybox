<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { get } from "svelte/store";
  import { activePanel, expandWindow, collapseWindow, minimized, restoreFromIsland, minimizeToIsland, expandIslandForLyrics, shrinkIslandFromLyrics } from "../../stores/app";
  import { currentSong, isPlaying, pauseMusic, resumeMusic, tracks, playSong, lyrics } from "../../stores/music";
  import IslandLyrics from "./IslandLyrics.svelte";

  const panels = [
    {
      id: "notes", label: "便签",
      svg: `<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>`
    },
    {
      id: "todo", label: "待办",
      svg: `<path d="M9 5H7a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2"/><rect x="9" y="3" width="6" height="4" rx="2"/><path d="M9 14l2 2 4-4"/>`
    },
    {
      id: "clipboard", label: "剪贴板",
      svg: `<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><rect x="8" y="2" width="8" height="4" rx="1" ry="1"/>`
    },
    {
      id: "dropzone", label: "文件暂存",
      svg: `<path d="M20 17v-5a8 8 0 1 0-16 0v5"/><line x1="12" y1="22" x2="12" y2="10"/><polyline points="8 14 12 10 16 14"/>`
    },
    {
      id: "music", label: "音乐",
      svg: `<path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/>`
    },
    {
      id: "settings", label: "设置",
      svg: `<circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>`
    },
  ];

  let current = $state<string | null>(null);
  let isMinimized = $state(false);
  let song = $state<any>(null);
  let playing = $state(false);
  let lrc = $state<any[]>([]);
  let mounted = $state(false);
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
    if (isMinimized && song && playing && lrc.length > 0) {
      expandIslandForLyrics();
    } else if (isMinimized) {
      shrinkIslandFromLyrics();
    }
  });

  async function selectPanel(id: string) {
    if (current === id) {
      current = null;
      await collapseWindow();
    } else {
      current = id;
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
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="sidebar-root"
  class:minimized={isMinimized}
  data-tauri-drag-region
>
  {#if isMinimized}
    <div class="island-content" data-tauri-drag-region>
      {#if song}
        <div class="island-player" data-tauri-drag-region>
          <img src={(song.pic_url || '') + '?param=80y80'} alt="" class="island-cover" />
          <div class="island-info" data-tauri-drag-region>
            <div class="island-title">{song.name}</div>
          </div>
          <button class="island-btn-sm" onclick={islandPrev} aria-label="上一首">
            <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor"><polygon points="19 20 9 12 19 4"/><line x1="5" y1="19" x2="5" y2="5" stroke="currentColor" stroke-width="3"/></svg>
          </button>
          <button class="island-btn" onclick={togglePlay} aria-label="播放暂停">
            {#if playing}
              <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
            {:else}
              <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg>
            {/if}
          </button>
          <button class="island-btn-sm" onclick={islandNext} aria-label="下一首">
            <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 4 15 12 5 20"/><line x1="19" y1="5" x2="19" y2="19" stroke="currentColor" stroke-width="3"/></svg>
          </button>
          <button class="island-btn-sm" onclick={restoreFromIsland} aria-label="恢复">
            <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round"><polyline points="15 18 9 12 15 6"/></svg>
          </button>
        </div>
        {#if playing && lrc.length > 0}
          <IslandLyrics />
        {/if}
      {:else}
        <button class="island-restore" onclick={restoreFromIsland} data-tauri-drag-region aria-label="恢复窗口">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="15 18 9 12 15 6"></polyline>
          </svg>
          <span>TinyBox</span>
        </button>
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
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
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
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
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

      <!-- Minimize to island -->
      <button
        class="sidebar-minimize-btn"
        onclick={() => minimizeToIsland()}
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
  }

  .sidebar-root.minimized {
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
    background: linear-gradient(90deg, var(--color-accent-cyan), var(--color-accent-purple));
    margin-bottom: 10px;
    opacity: 0.6;
    box-shadow: 0 0 6px rgba(0, 229, 255, 0.15);
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
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.08), transparent);
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
    color: rgba(255, 255, 255, 0.15);
    cursor: pointer;
    transition: all 0.2s ease;
    margin-bottom: 2px;
  }

  .sidebar-minimize-btn:hover {
    color: rgba(255, 255, 255, 0.45);
    background: rgba(255, 255, 255, 0.04);
  }

  .sidebar-minimize-btn:active {
    transform: scale(0.85);
  }

  /* Dynamic Island inner */
  .island-content {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 0 6px;
    gap: 4px;
  }

  .island-player {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    flex-shrink: 0;
  }

  .island-cover {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    object-fit: cover;
    flex-shrink: 0;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  }

  .island-info {
    flex: 1;
    min-width: 0;
  }

  .island-title {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.8);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .island-btn {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: rgba(255, 255, 255, 0.7);
    background: rgba(255, 255, 255, 0.08);
    border: none;
    cursor: pointer;
    flex-shrink: 0;
    transition: all 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .island-btn:hover {
    background: rgba(0, 229, 255, 0.15);
    color: rgba(0, 229, 255, 0.9);
    box-shadow: 0 0 8px rgba(0, 229, 255, 0.15);
  }

  .island-btn:active {
    transform: scale(0.85);
  }

  .island-btn-sm {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: rgba(255, 255, 255, 0.35);
    background: transparent;
    border: none;
    cursor: pointer;
    flex-shrink: 0;
    transition: all 0.2s;
  }

  .island-btn-sm:hover {
    color: rgba(255, 255, 255, 0.85);
    background: rgba(255, 255, 255, 0.08);
  }

  .island-btn-sm:active {
    transform: scale(0.85);
  }

  .island-restore {
    display: flex;
    align-items: center;
    gap: 6px;
    background: none;
    border: none;
    color: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    padding: 4px 10px;
    border-radius: 16px;
    transition: all 0.15s;
  }

  .island-restore:hover {
    color: rgba(0, 229, 255, 0.8);
    background: rgba(0, 229, 255, 0.06);
  }

  .island-restore:active {
    transform: scale(0.95);
  }

  .island-restore span {
    font-size: 11px;
    font-weight: 500;
  }

  /* Mini player in sidebar */
  .mini-player {
    position: relative;
    width: 38px;
    height: 38px;
    border-radius: 12px;
    overflow: hidden;
    flex-shrink: 0;
    border: 1px solid rgba(0, 229, 255, 0.15);
    box-shadow: 0 0 10px rgba(0, 229, 255, 0.08);
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .mini-player:hover {
    border-color: rgba(0, 229, 255, 0.35);
    box-shadow: 0 0 16px rgba(0, 229, 255, 0.2);
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
