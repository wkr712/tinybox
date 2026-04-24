export interface Song {
  id: string;
  name: string;
  artists: string;
  album: string;
  album_id: number;
  duration: number;
  pic_url: string;
}

export interface Playlist {
  id: number;
  name: string;
  cover_img_url: string;
  track_count: number;
  creator: string;
}

export interface MusicUser {
  user_id: number;
  nickname: string;
  avatar_url: string;
}

export interface LyricLine {
  time: number;
  text: string;
}

export type MusicView = "login" | "playlists" | "tracks" | "search" | "nowplaying" | "discover";

export type MusicProviderKind = "ncm" | "qqmusic" | "kugou";

export interface HotSearch {
  search_word: string;
  score: number;
  content: string;
}
