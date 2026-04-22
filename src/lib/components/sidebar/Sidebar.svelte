<script lang="ts">
  import { activePanel, expandWindow, collapseWindow } from "../../stores/app";
  import { currentSong, isPlaying, pauseMusic, resumeMusic } from "../../stores/music";

  const panels = [
    { id: "notes", icon: "M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z", label: "便签" },
    { id: "todo", icon: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4", label: "待办" },
    { id: "clipboard", icon: "M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M8 5a2 2 0 002 2h4a2 2 0 002-2M8 5a2 2 0 012-2h4a2 2 0 012 2", label: "剪贴板" },
    { id: "dropzone", icon: "M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12", label: "文件暂存" },
    { id: "music", icon: "M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2z", label: "音乐" },
  ];

  let current = $state<string | null>(null);
  activePanel.subscribe((v) => (current = v));

  let song: any = null;
  currentSong.subscribe((v) => (song = v));

  let playing = $state(false);
  isPlaying.subscribe((v) => (playing = v));

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
</script>

<div class="flex flex-col items-center py-4 gap-2 w-[52px] shrink-0" data-tauri-drag-region>
  {#each panels as panel}
    <button
      class="sidebar-icon {current === panel.id ? 'active' : ''}"
      onclick={() => selectPanel(panel.id)}
      title={panel.label}
    >
      <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        {@html panel.icon}
      </svg>
    </button>
  {/each}

  <!-- Mini player: shows when collapsed + music playing -->
  {#if song && current !== "music"}
    <button
      onclick={() => selectPanel('music')}
      class="w-9 h-9 rounded-lg overflow-hidden shrink-0 ring-1 ring-white/10 hover:ring-accent-cyan/30 transition-all relative"
      title="{song.name} - {song.artists}"
    >
      <img src={song.pic_url + '?param=80y80'} alt="" class="w-full h-full object-cover" />
      <div class="absolute inset-0 bg-black/30 flex items-center justify-center" onclick={togglePlay}>
        {#if playing}
          <svg class="w-3 h-3 text-white/80" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
        {:else}
          <svg class="w-3 h-3 text-white/80" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg>
        {/if}
      </div>
    </button>
  {/if}

  <div class="flex-1"></div>

  <button class="sidebar-icon {current === 'settings' ? 'active' : ''}" onclick={() => selectPanel('settings')} title="设置">
    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="12" cy="12" r="3"></circle>
      <path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-2 2 2 2 0 01-2-2v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06A1.65 1.65 0 004.68 15a1.65 1.65 0 00-1.51-1H3a2 2 0 01-2-2 2 2 0 012-2h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06A1.65 1.65 0 009 4.68a1.65 1.65 0 001-1.51V3a2 2 0 012-2 2 2 0 012 2v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 012 2 2 2 0 01-2 2h-.09a1.65 1.65 0 00-1.51 1z"></path>
    </svg>
  </button>
</div>
