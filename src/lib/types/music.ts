export interface NcmSong {
  id: number;
  name: string;
  artists: string;
  album: string;
  album_id: number;
  duration: number;
  pic_url: string;
}

export interface NcmPlaylist {
  id: number;
  name: string;
  cover_img_url: string;
  track_count: number;
  creator: string;
}

export interface NcmUser {
  user_id: number;
  nickname: string;
  avatar_url: string;
}

export interface LyricLine {
  time: number;
  text: string;
}

export type MusicView = "login" | "playlists" | "tracks" | "search" | "nowplaying" | "discover";

export interface HotSearch {
  search_word: string;
  score: number;
  content: string;
}
