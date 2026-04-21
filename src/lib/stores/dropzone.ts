import { writable } from "svelte/store";
import { select, execute } from "../utils/db";
import { invoke } from "@tauri-apps/api/core";
import type { DropZoneFile } from "../types/dropzone";

interface StoredFile {
  file_name: string;
  file_path: string;
  file_size: number;
  stored_path: string;
}

export const dropzoneFiles = writable<DropZoneFile[]>([]);
export const isDragOver = writable(false);

export async function loadFiles() {
  const result = await select<DropZoneFile>(
    "SELECT * FROM dropzone_files ORDER BY created_at DESC"
  );
  dropzoneFiles.set(result);
}

export async function storeFiles(filePaths: string[]) {
  const stored: StoredFile[] = await invoke("dropzone_store", { filePaths });

  for (const f of stored) {
    await execute(
      "INSERT INTO dropzone_files (file_name, file_path, file_size, mime_type) VALUES (?, ?, ?, ?)",
      [f.file_name, f.stored_path, f.file_size, guessMime(f.file_name)]
    );
  }

  await loadFiles();
}

export async function copyOut(id: number, targetDir: string) {
  const files = await select<DropZoneFile>("SELECT * FROM dropzone_files WHERE id = ?", [id]);
  if (files.length === 0) return;
  const file = files[0];
  await invoke("dropzone_copy_out", { storedPath: file.file_path, targetDir });
}

export async function deleteFile(id: number) {
  const files = await select<DropZoneFile>("SELECT * FROM dropzone_files WHERE id = ?", [id]);
  if (files.length === 0) return;
  const file = files[0];
  await invoke("dropzone_delete", { storedPath: file.file_path });
  await execute("DELETE FROM dropzone_files WHERE id = ?", [id]);
  await loadFiles();
}

export async function updateTags(id: number, tags: string) {
  await execute("UPDATE dropzone_files SET tags = ? WHERE id = ?", [tags, id]);
  await loadFiles();
}

function guessMime(name: string): string {
  const ext = name.split(".").pop()?.toLowerCase() || "";
  const map: Record<string, string> = {
    png: "image/png", jpg: "image/jpeg", jpeg: "image/jpeg", gif: "image/gif",
    pdf: "application/pdf", txt: "text/plain", md: "text/markdown",
    zip: "application/zip", json: "application/json", csv: "text/csv",
    doc: "application/msword", docx: "application/msword",
    xls: "application/vnd.ms-excel", xlsx: "application/vnd.ms-excel",
    mp3: "audio/mpeg", mp4: "video/mp4", svg: "image/svg+xml",
  };
  return map[ext] || "application/octet-stream";
}

export function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

export function fileIcon(mime: string | null): string {
  if (!mime) return "file";
  if (mime.startsWith("image/")) return "image";
  if (mime.startsWith("video/")) return "video";
  if (mime.startsWith("audio/")) return "audio";
  if (mime.includes("pdf")) return "pdf";
  if (mime.includes("zip")) return "zip";
  return "file";
}
