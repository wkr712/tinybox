<script lang="ts">
  import { onDestroy } from "svelte";
  import { editingNoteId, updateNote, deleteNote, togglePin } from "../../../stores/notes";
  import { NOTE_COLORS } from "../../../types/note";
  import type { Note } from "../../../types/note";

  let { note }: { note: Note } = $props();

  let title = $state("");
  let content = $state("");
  let showColors = $state(false);
  let saveTimeout: ReturnType<typeof setTimeout> | null = null;

  onDestroy(() => {
    if (saveTimeout) clearTimeout(saveTimeout);
  });

  // Initialize from note id — only reset when switching notes, not on every store update
  let lastNoteId = $state(0);
  $effect(() => {
    if (note.id !== lastNoteId) {
      lastNoteId = note.id;
      title = note.title;
      content = note.content;
    }
  });

  function save() {
    if (saveTimeout) clearTimeout(saveTimeout);
    const id = lastNoteId;
    const t = title;
    const c = content;
    saveTimeout = setTimeout(() => {
      updateNote(id, { title: t, content: c });
    }, 300);
  }

  async function close() {
    if (saveTimeout) {
      clearTimeout(saveTimeout);
      saveTimeout = null;
    }
    await updateNote(lastNoteId, { title, content });
    editingNoteId.set(null);
  }

  function changeColor(color: string) {
    updateNote(lastNoteId, { color });
    showColors = false;
  }

  function handleTogglePin() {
    togglePin(lastNoteId, note.pinned);
  }

  function confirmDelete() {
    if (confirm("删除这条便签？")) {
      deleteNote(lastNoteId);
    }
  }
</script>

<div class="rounded-xl p-3 border transition-colors" style="background: {note.color}50; border-color: {note.color}80;">
  <div class="flex items-center justify-between mb-2">
    <input
      type="text"
      bind:value={title}
      oninput={save}
      placeholder="标题"
      class="bg-transparent text-sm font-medium text-white/90 placeholder-white/30 outline-none flex-1"
    />
    <div class="flex items-center gap-1">
      <button
        onclick={() => (showColors = !showColors)}
        class="w-5 h-5 rounded-full border-2 border-white/20 transition-transform hover:scale-110"
        style="background: {note.color}"
        title="更换颜色"
        aria-label="更换颜色"
      ></button>
      <button
        onclick={handleTogglePin}
        class="w-5 h-5 rounded flex items-center justify-center hover:bg-white/10 transition-colors"
        title={note.pinned ? "取消置顶" : "置顶"}
      >
        <svg class="w-3 h-3" style="color: {note.pinned ? 'var(--color-accent-cyan)' : 'rgba(255,255,255,0.4)'}" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
          <path d="M16 12V4h1V2H7v2h1v8l-2 2v2h5.2v6h1.6v-6H18v-2l-2-2z"/>
        </svg>
      </button>
      <button
        onclick={confirmDelete}
        class="w-5 h-5 rounded flex items-center justify-center hover:bg-red-500/30 transition-colors text-white/40 hover:text-red-400"
        title="删除"
      >
        <svg class="w-3 h-3" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="3 6 5 6 21 6"></polyline>
          <path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"></path>
        </svg>
      </button>
      <button
        onclick={close}
        class="w-5 h-5 rounded flex items-center justify-center hover:bg-white/10 transition-colors text-white/40"
        title="完成编辑"
      >
        <svg class="w-3.5 h-3.5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="20 6 9 17 4 12"></polyline>
        </svg>
      </button>
    </div>
  </div>

  {#if showColors}
    <div class="flex gap-1.5 mb-2">
      {#each NOTE_COLORS as color}
        <button
          onclick={() => changeColor(color)}
          class="w-4 h-4 rounded-full border transition-transform hover:scale-125 {note.color === color ? 'border-accent-cyan ring-1 ring-accent-cyan/50' : 'border-white/20'}"
          style="background: {color}"
          title="选择颜色"
          aria-label="选择颜色 {color}"
        ></button>
      {/each}
    </div>
  {/if}

  <textarea
    bind:value={content}
    oninput={save}
    placeholder="写点什么..."
    rows="6"
    class="w-full bg-transparent text-xs text-white/70 placeholder-white/25 outline-none resize-none leading-relaxed"
  ></textarea>

  <div class="text-[10px] text-white/15 mt-1">{note.updated_at}</div>
</div>
