export interface DropZoneFile {
  id: number;
  file_name: string;
  file_path: string;
  file_size: number;
  mime_type: string | null;
  thumbnail_path: string | null;
  tags: string | null;
  created_at: string;
}
