export interface Note {
  id: number;
  title: string;
  content: string;
  color: string;
  pinned: number;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export const NOTE_COLORS = [
  "#1a1a2e",
  "#2d1b3d",
  "#1b2d3d",
  "#1b3d2d",
  "#3d2d1b",
  "#3d1b2d",
  "#2d3d1b",
  "#1b1b3d",
] as const;
