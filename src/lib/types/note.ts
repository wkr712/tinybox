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
  "#2a1a3e",
  "#1a2e4a",
  "#1a3e2a",
  "#3e2a1a",
  "#3e1a2e",
  "#1a3e3e",
  "#3e3e1a",
  "#2e1a3e",
] as const;
