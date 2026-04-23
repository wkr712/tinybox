import { writable, get } from "svelte/store";
import { select, execute } from "../utils/db";
import type { Note } from "../types/note";

export const notes = writable<Note[]>([]);
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
  const rows = await select<{ id: number }>(
    "SELECT last_insert_rowid() as id"
  );
  if (rows.length > 0) {
    editingNoteId.set(rows[0].id);
  }
  await loadNotes();
}

const NOTE_COLUMNS = new Set(["title", "content", "color", "pinned"]);

export async function updateNote(id: number, fields: Partial<Pick<Note, "title" | "content" | "color" | "pinned">>) {
  const sets: string[] = [];
  const values: unknown[] = [];

  for (const [key, value] of Object.entries(fields)) {
    if (!NOTE_COLUMNS.has(key)) continue;
    sets.push(`${key} = ?`);
    values.push(value);
  }
  sets.push("updated_at = datetime('now', 'localtime')");
  values.push(id);

  await execute(`UPDATE notes SET ${sets.join(", ")} WHERE id = ?`, values);
  await loadNotes();
}

export async function deleteNote(id: number) {
  const snapshot = get(notes);
  notes.update(items => items.filter(n => n.id !== id));
  editingNoteId.set(null);
  execute("DELETE FROM notes WHERE id = ?", [id]).catch(() => notes.set(snapshot));
}

export async function togglePin(id: number, currentPinned: number) {
  const newVal = currentPinned ? 0 : 1;
  notes.update(items => items.map(n => n.id === id ? { ...n, pinned: newVal } : n));
  updateNote(id, { pinned: newVal }).catch(() => loadNotes());
}

export function getFilteredNotes(allNotes: Note[], query: string): Note[] {
  if (!query.trim()) return allNotes;
  const q = query.toLowerCase();
  return allNotes.filter(
    (n) => n.title.toLowerCase().includes(q) || n.content.toLowerCase().includes(q)
  );
}
