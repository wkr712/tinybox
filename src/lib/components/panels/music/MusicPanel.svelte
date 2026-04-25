<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    user, currentView, previousView, fetchLoginStatus, fetchUserPlaylists,
    currentSong, isPlaying, activeProvider, switchProvider, logout, getActiveProviderKind,
  } from "../../../stores/music";
  import { get } from "svelte/store";
  import type { MusicProviderKind } from "../../../types/music";
  import QrLogin from "./QrLogin.svelte";
  import PlaylistView from "./PlaylistView.svelte";
  import TrackList from "./TrackList.svelte";
  import NowPlaying from "./NowPlaying.svelte";
  import SearchView from "./SearchView.svelte";
  import DiscoverView from "./DiscoverView.svelte";

  let u = $state<any>(null);
  let view = $state<any>("login");
  let song = $state<any>(null);
  let playing = $state(false);
  let provider = $state<MusicProviderKind>("ncm");
  let showProviderMenu = $state(false);
  let unsubs: (() => void)[] = [];

  function handleClickOutside(e: MouseEvent) {
    if (showProviderMenu) showProviderMenu = false;
  }

  const providers: { id: MusicProviderKind; label: string }[] = [
    { id: "ncm", label: "网易云音乐" },
    { id: "qqmusic", label: "QQ音乐" },
    { id: "kugou", label: "酷狗音乐" },
  ];

  onMount(async () => {
    unsubs.push(user.subscribe((v) => (u = v)));
    unsubs.push(currentView.subscribe((v) => (view = v)));
    unsubs.push(currentSong.subscribe((v) => (song = v)));
    unsubs.push(isPlaying.subscribe((v) => (playing = v)));
    unsubs.push(activeProvider.subscribe((v) => (provider = v)));

    await getActiveProviderKind();
    const ok = await fetchLoginStatus();
    const currentUser = get(user);
    if (ok && currentUser) {
      currentView.set("discover");
      await fetchUserPlaylists(currentUser.user_id);
    }
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  async function handleSwitchProvider(id: MusicProviderKind) {
    showProviderMenu = false;
    if (id === provider) return;
    await switchProvider(id);
  }

  async function handleLogout() {
    showProviderMenu = false;
    await logout();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="h-full flex flex-col" onclick={handleClickOutside}>
  <!-- Provider selector + account bar -->
  <div class="flex items-center justify-between pb-1.5">
    <div class="relative">
      <button
        onclick={(e) => { e.stopPropagation(); showProviderMenu = !showProviderMenu; }}
        class="flex items-center gap-1.5 px-2 py-1 rounded-md hover:bg-white/[0.03] active:scale-95 transition-all"
      >
        <span class="text-[10px] text-white/50">{providers.find(p => p.id === provider)?.label || provider}</span>
        <svg class="w-3 h-3 text-white/20 transition-transform {showProviderMenu ? 'rotate-180' : ''}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="6 9 12 15 18 9"/></svg>
      </button>
      {#if showProviderMenu}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div class="provider-menu">
          {#each providers as p (p.id)}
            <button
              onclick={() => handleSwitchProvider(p.id)}
              class="provider-option {p.id === provider ? 'active' : ''}"
            >{p.label}</button>
          {/each}
          {#if u}
            <div class="provider-divider"></div>
            <button
              onclick={handleLogout}
              class="provider-option text-red-400/50 hover:text-red-400 hover:bg-red-500/8"
            >退出登录</button>
          {/if}
        </div>
      {/if}
    </div>
    {#if u}
      <div class="flex items-center gap-1.5">
        {#if u.avatar_url}
          <img src={u.avatar_url} alt="" class="w-4 h-4 rounded-full" />
        {/if}
        <span class="text-[10px] text-white/30 truncate max-w-[80px]">{u.nickname}</span>
      </div>
    {/if}
  </div>

  <!-- Tab bar -->
  {#if view !== "login"}
    <div class="tab-bar">
      <button
        onclick={() => currentView.set('discover')}
        class="tab-item {view === 'discover' ? 'active' : ''}"
      >发现</button>
      <button
        onclick={() => currentView.set('playlists')}
        class="tab-item {view === 'playlists' ? 'active' : ''}"
      >歌单</button>
      <button
        onclick={() => currentView.set('search')}
        class="tab-item {view === 'search' ? 'active' : ''}"
      >搜索</button>
      {#if song}
        <button
          onclick={() => { previousView.set(view); currentView.set('nowplaying'); }}
          class="tab-item {view === 'nowplaying' ? 'active' : ''}"
        >播放</button>
      {/if}
    </div>
  {/if}

  <!-- Views -->
  <div class="flex-1 overflow-hidden">
    {#if view === "login"}
      <QrLogin />
    {:else if view === "discover"}
      <DiscoverView />
    {:else if view === "playlists"}
      <PlaylistView />
    {:else if view === "tracks"}
      <TrackList />
    {:else if view === "nowplaying"}
      <NowPlaying />
    {:else if view === "search"}
      <SearchView />
    {/if}
  </div>
</div>

<style>
  .provider-menu {
    position: absolute;
    top: 100%;
    left: 0;
    margin-top: 4px;
    background: color-mix(in srgb, var(--color-dark-surface) 95%, transparent);
    backdrop-filter: blur(12px);
    border: 1px solid var(--color-border-default);
    border-radius: 8px;
    padding: 4px 0;
    z-index: 50;
    min-width: 140px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
  }

  .provider-option {
    width: 100%;
    text-align: left;
    padding: 5px 12px;
    font-size: 10px;
    color: rgba(255, 255, 255, 0.45);
    transition: all 0.15s ease;
    background: transparent;
    border: none;
    cursor: pointer;
    display: block;
  }

  .provider-option:hover {
    color: rgba(255, 255, 255, 0.65);
    background: rgba(255, 255, 255, 0.03);
  }

  .provider-option.active {
    color: var(--color-accent-primary);
    background: color-mix(in srgb, var(--color-accent-primary) 8%, transparent);
  }

  .provider-divider {
    border-top: 1px solid rgba(255, 255, 255, 0.05);
    margin: 4px 0;
  }

  .tab-bar {
    display: flex;
    align-items: center;
    gap: 2px;
    padding-bottom: 8px;
    margin-bottom: 8px;
    border-bottom: 1px solid var(--color-border-subtle);
    position: relative;
  }

  .tab-item {
    padding: 4px 8px;
    font-size: 10px;
    border-radius: 4px;
    color: rgba(255, 255, 255, 0.25);
    transition: all 0.15s ease;
    background: transparent;
    border: none;
    cursor: pointer;
    position: relative;
  }

  .tab-item:hover {
    color: rgba(255, 255, 255, 0.45);
  }

  .tab-item.active {
    color: var(--color-accent-primary);
  }

  .tab-item.active::after {
    content: '';
    position: absolute;
    bottom: -9px;
    left: 50%;
    transform: translateX(-50%);
    width: 12px;
    height: 1.5px;
    background: var(--color-accent-primary);
    border-radius: 1px;
  }
</style>
