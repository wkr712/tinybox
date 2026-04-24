<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    user, currentView, previousView, fetchLoginStatus, fetchUserPlaylists,
    currentSong, isPlaying, activeProvider, switchProvider, logout,
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

<div class="h-full flex flex-col">
  <!-- Provider selector + account bar -->
  <div class="flex items-center justify-between pb-1.5">
    <div class="relative">
      <button
        onclick={() => (showProviderMenu = !showProviderMenu)}
        class="flex items-center gap-1.5 px-2 py-1 rounded-md hover:bg-white/[0.04] active:scale-95 transition-all"
      >
        <span class="text-[10px] text-white/60">{providers.find(p => p.id === provider)?.label || provider}</span>
        <svg class="w-3 h-3 text-white/25 transition-transform {showProviderMenu ? 'rotate-180' : ''}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="6 9 12 15 18 9"/></svg>
      </button>
      {#if showProviderMenu}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div class="absolute top-full left-0 mt-1 bg-dark-surface/95 backdrop-blur-md border border-white/10 rounded-lg py-1 z-50 min-w-[140px] shadow-xl">
          {#each providers as p (p.id)}
            <button
              onclick={() => handleSwitchProvider(p.id)}
              class="w-full text-left px-3 py-1.5 text-[10px] {p.id === provider ? 'text-accent-primary bg-accent-primary/10' : 'text-white/50 hover:text-white/70 hover:bg-white/[0.03]'} transition-all"
            >{p.label}</button>
          {/each}
          {#if u}
            <div class="border-t border-white/5 mt-1 pt-1">
              <button
                onclick={handleLogout}
                class="w-full text-left px-3 py-1.5 text-[10px] text-red-400/60 hover:text-red-400 hover:bg-red-500/10 transition-all"
              >退出登录</button>
            </div>
          {/if}
        </div>
      {/if}
    </div>
    {#if u}
      <div class="flex items-center gap-1.5">
        {#if u.avatar_url}
          <img src={u.avatar_url} alt="" class="w-4 h-4 rounded-full" />
        {/if}
        <span class="text-[10px] text-white/40 truncate max-w-[80px]">{u.nickname}</span>
      </div>
    {/if}
  </div>

  <!-- Tab bar when logged in -->
  {#if view !== "login"}
    <div class="flex items-center gap-1 pb-2 border-b border-white/5 mb-2">
      <button
        onclick={() => currentView.set('discover')}
        class="px-2 py-1 text-[10px] rounded active:scale-95 transition-all {view === 'discover' ? 'text-accent-primary bg-accent-primary/10' : 'text-white/30 hover:text-white/50'}"
      >发现</button>
      <button
        onclick={() => currentView.set('playlists')}
        class="px-2 py-1 text-[10px] rounded active:scale-95 transition-all {view === 'playlists' ? 'text-accent-primary bg-accent-primary/10' : 'text-white/30 hover:text-white/50'}"
      >歌单</button>
      <button
        onclick={() => currentView.set('search')}
        class="px-2 py-1 text-[10px] rounded active:scale-95 transition-all {view === 'search' ? 'text-accent-primary bg-accent-primary/10' : 'text-white/30 hover:text-white/50'}"
      >搜索</button>
      {#if song}
        <button
          onclick={() => { previousView.set(view); currentView.set('nowplaying'); }}
          class="px-2 py-1 text-[10px] rounded active:scale-95 transition-all {view === 'nowplaying' ? 'text-accent-primary bg-accent-primary/10' : 'text-white/30 hover:text-white/50'}"
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
