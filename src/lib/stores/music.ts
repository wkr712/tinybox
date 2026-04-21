import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { NcmSong, NcmPlaylist, NcmUser, LyricLine, MusicView } from "../types/music";

export const user = writable<NcmUser | null>(null);
export const playlists = writable<NcmPlaylist[]>([]);
export const currentPlaylist = writable<NcmPlaylist | null>(null);
export const tracks = writable<NcmSong[]>([]);
export const currentSong = writable<NcmSong | null>(null);
export const lyrics = writable<LyricLine[]>([]);
export const isPlaying = writable(false);
export const currentView = writable<MusicView>("login");
export const searchResults = writable<NcmSong[]>([]);
export const searchQuery = writable("");
export const volume = writable(0.8);

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
  isPlaying.set(true);

  // Fetch lyrics
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

export async function togglePlay() {
  const playing = await invoke<boolean>("music_pause").then(() => false).catch(() => false);
  if (!playing) {
    await invoke("music_resume");
    isPlaying.set(true);
  } else {
    await invoke("music_pause");
    isPlaying.set(false);
  }
}

export async function pauseMusic() {
  await invoke("music_pause");
  isPlaying.set(false);
}

export async function resumeMusic() {
  await invoke("music_resume");
  isPlaying.set(true);
}

export async function stopMusic() {
  await invoke("music_stop");
  isPlaying.set(false);
  currentSong.set(null);
}

export async function setVolume(vol: number) {
  await invoke("music_set_volume", { volume: vol });
  volume.set(vol);
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
