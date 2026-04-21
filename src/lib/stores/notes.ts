import { writable } from "svelte/store";
import { select, execute } from "../utils/db";
import type { Note } from "../types/note";

export const notes = writable<Note[]>([]);
export const searchQuery = writable("");
export const editingNoteId = writable<number | null>(null);

export async function loadNotes() {
  const result = await select<Note>(
    "SELECT * FROM notes ORDER BY pinned DESC, sort_order ASC, updated_at DESC"
  );
  notes.set(result);
}

export async function createNote() {
  await execute(
    "INSERT INTO notes (title, content, color) VALUES ('', '', '#1a1a2e')"
  );
  await loadNotes();
  const rows = await select<{ id: number }[]>(
    "SELECT id FROM notes ORDER BY id DESC LIMIT 1"
  );
  if (rows.length > 0) {
    editingNoteId.set(rows[0].id);
  }
}

export async function updateNote(id: number, fields: Partial<Pick<Note, "title" | "content" | "color" | "pinned">>) {
  const sets: string[] = [];
  const values: unknown[] = [];

  for (const [key, value] of Object.entries(fields)) {
    sets.push(`${key} = ?`);
    values.push(value);
  }
  sets.push("updated_at = datetime('now', 'localtime')");
  values.push(id);

  await execute(`UPDATE notes SET ${sets.join(", ")} WHERE id = ?`, values);
  await loadNotes();
}

export async function deleteNote(id: number) {
  await execute("DELETE FROM notes WHERE id = ?", [id]);
  editingNoteId.set(null);
  await loadNotes();
}

export async function togglePin(id: number, currentPinned: number) {
  await updateNote(id, { pinned: currentPinned ? 0 : 1 });
}

export function getFilteredNotes(allNotes: Note[], query: string): Note[] {
  if (!query.trim()) return allNotes;
  const q = query.toLowerCase();
  return allNotes.filter(
    (n) => n.title.toLowerCase().includes(q) || n.content.toLowerCase().includes(q)
  );
}
