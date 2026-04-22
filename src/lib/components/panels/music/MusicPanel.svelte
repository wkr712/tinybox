<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    user, currentView, previousView, fetchLoginStatus, fetchUserPlaylists, currentSong, isPlaying,
    pauseMusic, resumeMusic,
  } from "../../../stores/music";
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
  let unsubs: (() => void)[] = [];

  onMount(async () => {
    unsubs.push(user.subscribe((v) => (u = v)));
    unsubs.push(currentView.subscribe((v) => (view = v)));
    unsubs.push(currentSong.subscribe((v) => (song = v)));
    unsubs.push(isPlaying.subscribe((v) => (playing = v)));

    const ok = await fetchLoginStatus();
    if (ok && u) {
      currentView.set("discover");
      await fetchUserPlaylists(u.user_id);
    }
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });
</script>

<div class="h-full flex flex-col">
  <!-- Top bar when logged in -->
  {#if view !== "login"}
    <div class="flex items-center gap-1 pb-2 border-b border-white/5 mb-2">
      <button
        onclick={() => currentView.set('discover')}
        class="px-2 py-1 text-[10px] rounded {view === 'discover' ? 'text-accent-cyan bg-accent-cyan/10' : 'text-white/30 hover:text-white/50'}"
      >发现</button>
      <button
        onclick={() => currentView.set('playlists')}
        class="px-2 py-1 text-[10px] rounded {view === 'playlists' ? 'text-accent-cyan bg-accent-cyan/10' : 'text-white/30 hover:text-white/50'}"
      >歌单</button>
      <button
        onclick={() => currentView.set('search')}
        class="px-2 py-1 text-[10px] rounded {view === 'search' ? 'text-accent-cyan bg-accent-cyan/10' : 'text-white/30 hover:text-white/50'}"
      >搜索</button>
      {#if song}
        <button
          onclick={() => { previousView.set(view); currentView.set('nowplaying'); }}
          class="px-2 py-1 text-[10px] rounded {view === 'nowplaying' ? 'text-accent-cyan bg-accent-cyan/10' : 'text-white/30 hover:text-white/50'}"
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
