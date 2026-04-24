import { writable, get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Song, Playlist, MusicUser, LyricLine, MusicView, MusicProviderKind, HotSearch } from "../types/music";

export const activeProvider = writable<MusicProviderKind>("ncm");
export const user = writable<MusicUser | null>(null);
export const playlists = writable<Playlist[]>([]);
export const currentPlaylist = writable<Playlist | null>(null);
export const tracks = writable<Song[]>([]);
export const currentSong = writable<Song | null>(null);
export const lyrics = writable<LyricLine[]>([]);
export const isPlaying = writable(false);
export const currentView = writable<MusicView>("login");
export const previousView = writable<MusicView>("playlists");
export const searchResults = writable<Song[]>([]);
export const volume = writable(0.8);

export const playProgress = writable(0);
export const recommendPlaylists = writable<Playlist[]>([]);
export const recommendSongs = writable<Song[]>([]);
export const hotSearches = writable<HotSearch[]>([]);

let progressTimer: ReturnType<typeof setInterval> | null = null;

export async function generateQr() {
  return await invoke<{ key: string; qrurl: string }>("music_qr_generate");
}

export async function checkQr(key: string) {
  const result = await invoke<Record<string, any>>("music_qr_check", { key });
  const provider = get(activeProvider);
  if (provider === "ncm") {
    return (result.code ?? 800) as number;
  } else if (provider === "qqmusic") {
    // QQ Music: check status field
    const status = result["music.client.QRCodeLogin.Server"]?.["data"]?.["status"] ?? 0;
    if (status === 0) return 800;   // expired
    if (status === 1) return 802;   // scanned
    if (status === 2) return 803;   // confirmed
    return 800;
  } else {
    // Kugou: 1=waiting, 2=scanned, 3=confirmed
    const status = result["data"]?.["status"] ?? 0;
    if (status === 1) return 801;
    if (status === 2) return 802;
    if (status === 3) return 803;
    return 800;
  }
}

export async function fetchLoginStatus() {
  const resp = await invoke<Record<string, any>>("music_login_status");
  const provider = get(activeProvider);
  if (provider === "ncm") {
    if (resp.code === 200 && resp.account) {
      const profile = resp.profile || {};
      user.set({
        user_id: profile.userId || resp.account?.id || 0,
        nickname: profile.nickname || resp.account?.userName || "",
        avatar_url: profile.avatarUrl || resp.account?.avatar || "",
      });
      return true;
    }
  } else {
    // QQ Music / Kugou: unified format from login_status
    if (resp.code === 200 && resp.account) {
      user.set({
        user_id: resp.account.id || 0,
        nickname: resp.account.userName || "",
        avatar_url: resp.account.avatar || "",
      });
      return true;
    }
  }
  return false;
}

export async function fetchUserPlaylists(uid: number) {
  const resp = await invoke<Record<string, any>>("music_user_playlist", { uid });
  const provider = get(activeProvider);
  const rawList = resp.playlist || [];
  playlists.set(
    rawList.map((p: any) => ({
      id: p.id,
      name: p.name,
      cover_img_url: p.coverImgUrl || p.cover_img_url || "",
      track_count: p.trackCount || p.track_count || 0,
      creator: p.creator?.nickname || p.creator || "",
    }))
  );
}

export async function fetchPlaylistTracks(id: number) {
  const resp = await invoke<Record<string, any>>("music_playlist_detail", { id });
  const provider = get(activeProvider);
  const songs = resp.playlist?.tracks || resp.songs || [];
  tracks.set(
    songs.map((s: any) => parseSong(s, provider))
  );
}

function parseSong(s: any, _provider: MusicProviderKind): Song {
  return {
    id: String(s.id ?? s.songmid ?? ""),
    name: s.name || s.songname || "",
    artists: typeof s.artists === 'string' ? s.artists : (s.ar || s.singer || s.artists || []).map((a: any) => a.name || a).join(" / "),
    album: s.album || s.al?.name || s.albumname || "",
    album_id: s.album_id || s.al?.id || 0,
    duration: s.duration || s.dt || 0,
    pic_url: s.pic_url || s.al?.picUrl || "",
  };
}

export async function playSong(song: Song) {
  try {
    const url = await invoke<string>("music_song_url", { id: String(song.id) });
    if (!url) return false;

    await invoke("music_play", { url });
    currentSong.set(song);
    playProgress.set(0);
    startProgressTimer();

    fetchLyrics(song.id).catch(() => {});
    return true;
  } catch {
    return false;
  }
}

export async function fetchLyrics(id: string) {
  try {
    const resp = await invoke<Record<string, any>>("music_lyric", { id });
    const lrcText = resp.lrc?.lyric || "";
    lyrics.set(parseLrc(lrcText));
  } catch {}
}

export async function searchSongs(keywords: string) {
  try {
    const resp = await invoke<Record<string, any>>("music_search", { keywords });
    const provider = get(activeProvider);
    const songs = resp.result?.songs || [];
    searchResults.set(songs.map((s: any) => parseSong(s, provider)));
  } catch {}
}

export async function pauseMusic() {
  try { await invoke("music_pause"); } catch {}
  stopProgressTimer();
}

export async function resumeMusic() {
  try { await invoke("music_resume"); } catch {}
  startProgressTimer();
}

export async function stopMusic() {
  try { await invoke("music_stop"); } catch {}
  currentSong.set(null);
  playProgress.set(0);
  stopProgressTimer();
}

export async function setVolume(vol: number) {
  try { await invoke("music_set_volume", { volume: vol }); } catch {}
  volume.set(vol);
}

export async function seekTo(positionSeconds: number) {
  try { await invoke("music_seek", { positionMs: Math.round(positionSeconds * 1000) }); } catch {}
  playProgress.set(positionSeconds);
}

export async function fetchRecommendPlaylists() {
  try {
    const resp = await invoke<Record<string, any>>("music_personalized", { limit: 12 });
    if (resp.result) {
      recommendPlaylists.set(
        resp.result.map((p: any) => ({
          id: p.id,
          name: p.name,
          cover_img_url: p.picUrl || p.cover_img_url || "",
          track_count: p.trackCount || p.track_count || 0,
          creator: p.copywriter || p.creator || "",
        }))
      );
    }
  } catch {}
}

export async function fetchRecommendSongs() {
  try {
    const resp = await invoke<Record<string, any>>("music_recommend_songs");
    const provider = get(activeProvider);
    const songs = resp.data?.dailySongs || resp.data?.daily_songs || resp.data || [];
    if (Array.isArray(songs)) {
      recommendSongs.set(songs.slice(0, 20).map((s: any) => parseSong(s, provider)));
    }
  } catch {}
}

export async function fetchHotSearches() {
  try {
    const resp = await invoke<Record<string, any>>("music_search_hot");
    const hotList = resp.result?.hots || resp.data || [];
    hotSearches.set(
      hotList.map((h: any) => ({
        search_word: h.searchWord || h.search_word || "",
        score: h.score || 0,
        content: h.content || "",
      }))
    );
  } catch {}
}

export async function switchProvider(provider: MusicProviderKind) {
  try { await invoke("music_set_provider", { provider }); } catch {}
  activeProvider.set(provider);
  // Reset state
  user.set(null);
  playlists.set([]);
  tracks.set([]);
  currentSong.set(null);
  lyrics.set([]);
  searchResults.set([]);
  recommendPlaylists.set([]);
  recommendSongs.set([]);
  hotSearches.set([]);
  currentView.set("login");
}

export async function getActiveProviderKind(): Promise<MusicProviderKind> {
  const kind = await invoke<string>("music_get_provider");
  activeProvider.set(kind as MusicProviderKind);
  return kind as MusicProviderKind;
}

export async function logout() {
  await invoke("music_logout");
  user.set(null);
  playlists.set([]);
  tracks.set([]);
  currentSong.set(null);
  lyrics.set([]);
  currentView.set("login");
}

let _audioUnlisten: (() => void) | null = null;

export async function initAudioListener() {
  if (_audioUnlisten) return;
  _audioUnlisten = await listen<{ playing: boolean; error?: string; reason?: string }>("audio-state-changed", (event) => {
    isPlaying.set(event.payload.playing);
    if (!event.payload.playing) {
      stopProgressTimer();
      if (event.payload.reason === "ended") {
        const song = get(currentSong);
        const trackList = get(tracks);
        if (song && trackList.length > 0) {
          const idx = trackList.findIndex((t) => t.id === song.id);
          if (idx >= 0 && idx < trackList.length - 1) {
            playSong(trackList[idx + 1]);
          }
        }
      } else if (event.payload.reason === "error") {
        currentSong.set(null);
        playProgress.set(0);
      }
    }
  });
}

export function destroyAudioListener() {
  if (_audioUnlisten) {
    _audioUnlisten();
    _audioUnlisten = null;
  }
}

function startProgressTimer() {
  stopProgressTimer();
  progressTimer = setInterval(() => {
    playProgress.update((p) => {
      const song = get(currentSong);
      const maxSeconds = (song?.duration || 0) / 1000;
      if (maxSeconds > 0 && p + 0.2 >= maxSeconds) return p;
      return p + 0.2;
    });
  }, 200);
}

function stopProgressTimer() {
  if (progressTimer) {
    clearInterval(progressTimer);
    progressTimer = null;
  }
}

function parseLrc(lrc: string): LyricLine[] {
  const lines: LyricLine[] = [];
  const timeRegex = /\[(\d{2}):(\d{2})\.(\d{2,3})\]/g;

  for (const line of lrc.split("\n")) {
    const times: number[] = [];
    let match: RegExpExecArray | null;
    timeRegex.lastIndex = 0;

    while ((match = timeRegex.exec(line)) !== null) {
      const min = parseInt(match[1]);
      const sec = parseInt(match[2]);
      const ms = parseInt(match[3].padEnd(3, "0"));
      times.push(min * 60 + sec + ms / 1000);
    }

    const text = line.replace(/\[\d{2}:\d{2}\.\d{2,3}\]/g, "").trim();
    if (text && times.length > 0) {
      for (const time of times) {
        lines.push({ time, text });
      }
    }
  }

  return lines.sort((a, b) => a.time - b.time);
}

export function formatDuration(ms: number): string {
  const s = Math.floor(ms / 1000);
  const m = Math.floor(s / 60);
  const sec = s % 60;
  return `${m}:${sec.toString().padStart(2, "0")}`;
}
