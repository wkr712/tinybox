export interface ClipboardItem {
  id: number;
  content: string;
  content_type: "text" | "image_path";
  source_app: string | null;
  is_favorite: number;
  created_at: string;
}
