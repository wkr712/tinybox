import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { NcmSong, NcmPlaylist, NcmUser, LyricLine, MusicView, HotSearch } from "../types/music";

export const user = writable<NcmUser | null>(null);
export const playlists = writable<NcmPlaylist[]>([]);
export const currentPlaylist = writable<NcmPlaylist | null>(null);
export const tracks = writable<NcmSong[]>([]);
export const currentSong = writable<NcmSong | null>(null);
export const lyrics = writable<LyricLine[]>([]);
export const isPlaying = writable(false);
export const currentView = writable<MusicView>("login");
export const previousView = writable<MusicView>("playlists");
export const searchResults = writable<NcmSong[]>([]);
export const searchQuery = writable("");
export const volume = writable(0.8);

// v0.7.0 additions
export const playProgress = writable(0); // seconds elapsed
export const recommendPlaylists = writable<NcmPlaylist[]>([]);
export const recommendSongs = writable<NcmSong[]>([]);
export const hotSearches = writable<HotSearch[]>([]);

let progressTimer: ReturnType<typeof setInterval> | null = null;

export async function generateQr() {
  return await invoke<{ key: string; qrurl: string }>("music_qr_generate");
}

export async function checkQr(key: string) {
  return await invoke<number>("music_qr_check", { key });
}

export async function fetchLoginStatus() {
  const resp = await invoke<Record<string, any>>("music_login_status");
  if (resp.code === 200 && resp.account) {
    const profile = resp.profile || {};
    user.set({
      user_id: profile.userId || 0,
      nickname: profile.nickname || "",
      avatar_url: profile.avatarUrl || "",
    });
    return true;
  }
  return false;
}

export async function fetchUserPlaylists(uid: number) {
  const resp = await invoke<Record<string, any>>("music_user_playlist", { uid });
  if (resp.playlist) {
    playlists.set(
      resp.playlist.map((p: any) => ({
        id: p.id,
        name: p.name,
        cover_img_url: p.coverImgUrl || "",
        track_count: p.trackCount || 0,
        creator: p.creator?.nickname || "",
      }))
    );
  }
}

export async function fetchPlaylistTracks(id: number) {
  const resp = await invoke<Record<string, any>>("music_playlist_detail", { id });
  const songs = resp.playlist?.tracks || [];
  tracks.set(
    songs.map((s: any) => ({
      id: s.id,
      name: s.name,
      artists: (s.ar || []).map((a: any) => a.name).join(" / "),
      album: s.al?.name || "",
      album_id: s.al?.id || 0,
      duration: s.dt || 0,
      pic_url: s.al?.picUrl || "",
    }))
  );
}

export async function playSong(song: NcmSong) {
  const resp = await invoke<Record<string, any>>("music_song_url", { id: song.id });
  const url = resp.data?.[0]?.url;
  if (!url) return false;

  await invoke("music_play", { url });
  currentSong.set(song);
  playProgress.set(0);
  startProgressTimer();

  fetchLyrics(song.id);
  return true;
}

export async function fetchLyrics(id: number) {
  const resp = await invoke<Record<string, any>>("music_lyric", { id });
  const lrcText = resp.lrc?.lyric || "";
  lyrics.set(parseLrc(lrcText));
}

export async function searchSongs(keywords: string) {
  const resp = await invoke<Record<string, any>>("music_search", { keywords });
  const songs = resp.result?.songs || [];
  searchResults.set(
    songs.map((s: any) => ({
      id: s.id,
      name: s.name,
      artists: (s.ar || []).map((a: any) => a.name).join(" / "),
      album: s.al?.name || "",
      album_id: s.al?.id || 0,
      duration: s.dt || 0,
      pic_url: s.al?.picUrl || "",
    }))
  );
}

export async function pauseMusic() {
  await invoke("music_pause");
  stopProgressTimer();
}

export async function resumeMusic() {
  await invoke("music_resume");
  startProgressTimer();
}

export async function stopMusic() {
  await invoke("music_stop");
  currentSong.set(null);
  playProgress.set(0);
  stopProgressTimer();
}

export async function setVolume(vol: number) {
  await invoke("music_set_volume", { volume: vol });
  volume.set(vol);
}

// v0.7.0: Recommendations
export async function fetchRecommendPlaylists() {
  const resp = await invoke<Record<string, any>>("music_personalized", { limit: 12 });
  if (resp.result) {
    recommendPlaylists.set(
      resp.result.map((p: any) => ({
        id: p.id,
        name: p.name,
        cover_img_url: p.picUrl || "",
        track_count: p.trackCount || 0,
        creator: p.copywriter || "",
      }))
    );
  }
}

export async function fetchRecommendSongs() {
  const resp = await invoke<Record<string, any>>("music_recommend_songs");
  const songs = resp.data?.dailySongs || resp.data || [];
  if (Array.isArray(songs)) {
    recommendSongs.set(
      songs.slice(0, 20).map((s: any) => ({
        id: s.id,
        name: s.name,
        artists: (s.ar || []).map((a: any) => a.name).join(" / "),
        album: s.al?.name || "",
        album_id: s.al?.id || 0,
        duration: s.dt || 0,
        pic_url: s.al?.picUrl || "",
      }))
    );
  }
}

export async function fetchHotSearches() {
  const resp = await invoke<Record<string, any>>("music_search_hot");
  if (resp.data) {
    hotSearches.set(
      resp.data.map((h: any) => ({
        search_word: h.searchWord || "",
        score: h.score || 0,
        content: h.content || "",
      }))
    );
  }
}

// Listen for audio state changes from Rust backend
listen<{ playing: boolean; error?: string }>("audio-state-changed", (event) => {
  isPlaying.set(event.payload.playing);
  if (!event.payload.playing && !event.payload.error) {
    stopProgressTimer();
  }
});

// Progress tracking
function startProgressTimer() {
  stopProgressTimer();
  progressTimer = setInterval(() => {
    playProgress.update((p) => p + 0.2);
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
