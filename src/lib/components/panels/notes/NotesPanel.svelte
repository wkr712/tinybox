<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { notes, createNote, loadNotes } from "../../../stores/notes";
  import NoteCard from "./NoteCard.svelte";
  import NoteEditor from "./NoteEditor.svelte";
  import { editingNoteId } from "../../../stores/notes";
  import { getFilteredNotes } from "../../../stores/notes";
  import type { Note } from "../../../types/note";
  import SkeletonLine from "../../shared/SkeletonLine.svelte";

  let allNotes = $state<Note[]>([]);
  let query = $state("");
  let currentEditId = $state<number | null>(null);
  let loading = $state(true);
  let unsubs: (() => void)[] = [];

  let filtered = $derived(getFilteredNotes(allNotes, query));
  let pinned = $derived(filtered.filter((n) => n.pinned));
  let unpinned = $derived(filtered.filter((n) => !n.pinned));

  onMount(async () => {
    unsubs.push(notes.subscribe((v) => { allNotes = v; loading = false; }));
    unsubs.push(editingNoteId.subscribe((v) => (currentEditId = v)));

    await loadNotes();
    loading = false;
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-2 mb-3">
    <div class="flex-1 search-box">
      <svg class="search-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
      <input
        type="text"
        placeholder="搜索便签..."
        bind:value={query}
        class="search-input"
      />
    </div>
    <button onclick={createNote} class="glow-button text-xs px-3 py-1.5">新建</button>
  </div>

  <div class="flex-1 overflow-y-auto space-y-2">
    {#if loading}
      {#each { length: 4 } as _}
        <div class="rounded-[10px] p-3 border border-white/5 space-y-2">
          <SkeletonLine width="70%" height="12px" />
          <SkeletonLine width="100%" height="10px" />
          <SkeletonLine width="40%" height="8px" />
        </div>
      {/each}
    {:else if pinned.length > 0}
      <div class="section-label">已置顶</div>
      {#each pinned as note (note.id)}
        {#if currentEditId === note.id}
          <NoteEditor {note} />
        {:else}
          <NoteCard {note} />
        {/if}
      {/each}
      <div class="divider"></div>
    {/if}

    {#each unpinned as note (note.id)}
      {#if currentEditId === note.id}
        <NoteEditor {note} />
      {:else}
        <NoteCard {note} />
      {/if}
    {/each}

    {#if filtered.length === 0 && allNotes.length > 0}
      <div class="empty-state">
        <svg class="empty-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
        <span>没有匹配的便签</span>
      </div>
    {:else if allNotes.length === 0}
      <div class="empty-state">
        <svg class="empty-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M20 2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h4l4 4 4-4h4c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2z"/></svg>
        <span>还没有便签，点击「新建」开始</span>
      </div>
    {/if}
  </div>
</div>

<style>
  .search-box {
    position: relative;
  }

  .search-icon {
    position: absolute;
    left: 8px;
    top: 50%;
    transform: translateY(-50%);
    width: 14px;
    height: 14px;
    color: rgba(255, 255, 255, 0.25);
  }

  .search-input {
    width: 100%;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    padding: 6px 12px 6px 28px;
    font-size: 12px;
    color: rgba(255, 255, 255, 0.8);
    outline: none;
    transition: all 0.2s ease;
  }

  .search-input::placeholder {
    color: rgba(255, 255, 255, 0.25);
  }

  .search-input:focus {
    border-color: color-mix(in srgb, var(--color-accent-primary) 30%, transparent);
    background: rgba(255, 255, 255, 0.06);
  }

  .section-label {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.25);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    margin-bottom: 4px;
  }

  .divider {
    height: 1px;
    background: var(--color-border-subtle);
    margin: 8px 0;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 32px 0;
    gap: 8px;
    color: rgba(255, 255, 255, 0.25);
    font-size: 12px;
  }

  .empty-icon {
    width: 24px;
    height: 24px;
    opacity: 0.1;
  }
</style>
