<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { notes, searchQuery, createNote, loadNotes } from "../../../stores/notes";
  import NoteCard from "./NoteCard.svelte";
  import NoteEditor from "./NoteEditor.svelte";
  import { editingNoteId } from "../../../stores/notes";
  import { getFilteredNotes } from "../../../stores/notes";
  import type { Note } from "../../../types/note";

  let allNotes = $state<Note[]>([]);
  let query = $state("");
  let currentEditId = $state<number | null>(null);
  let unsubs: (() => void)[] = [];

  let filtered = $derived(getFilteredNotes(allNotes, query));
  let pinned = $derived(filtered.filter((n) => n.pinned));
  let unpinned = $derived(filtered.filter((n) => !n.pinned));

  onMount(() => {
    unsubs.push(notes.subscribe((v) => (allNotes = v)));
    unsubs.push(editingNoteId.subscribe((v) => (currentEditId = v)));
    unsubs.push(searchQuery.subscribe((v) => (query = v)));

    loadNotes();
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-2 mb-3 px-1">
    <div class="flex-1 relative">
      <svg class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-white/30" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
      <input
        type="text"
        placeholder="搜索便签..."
        bind:value={query}
        class="w-full bg-white/5 border border-white/10 rounded-lg pl-8 pr-3 py-1.5 text-xs text-white/80 placeholder-white/30 outline-none focus:border-white/20 transition-colors"
      />
    </div>
    <button onclick={createNote} class="glow-button text-xs px-3 py-1.5">新建</button>
  </div>

  <div class="flex-1 overflow-y-auto space-y-2 px-1">
    {#if pinned.length > 0}
      <div class="text-[10px] text-white/30 uppercase tracking-wider mb-1">已置顶</div>
      {#each pinned as note (note.id)}
        {#if currentEditId === note.id}
          <NoteEditor {note} />
        {:else}
          <NoteCard {note} />
        {/if}
      {/each}
      <div class="h-px bg-white/5 my-2"></div>
    {/if}

    {#each unpinned as note (note.id)}
      {#if currentEditId === note.id}
        <NoteEditor {note} />
      {:else}
        <NoteCard {note} />
      {/if}
    {/each}

    {#if filtered.length === 0 && allNotes.length > 0}
      <div class="text-white/30 text-xs text-center py-8">没有匹配的便签</div>
    {:else if allNotes.length === 0}
      <div class="text-white/30 text-xs text-center py-8">还没有便签，点击「新建」开始</div>
    {/if}
  </div>
</div>
