<script lang="ts">
  import { editingNoteId, deleteNote, togglePin } from "../../../stores/notes";
  import type { Note } from "../../../types/note";
  import { ask } from "@tauri-apps/plugin-dialog";

  let { note }: { note: Note } = $props();

  let showActions = $state(false);

  function edit() {
    editingNoteId.set(note.id);
  }

  async function confirmDelete() {
    const yes = await ask("删除这条便签？", { title: "确认删除", kind: "warning", okLabel: "删除", cancelLabel: "取消" });
    if (yes) deleteNote(note.id);
  }

  let displayTitle = $derived(note.title || note.content.split("\n")[0]?.slice(0, 40) || "空便签");
  let preview = $derived(note.title ? note.content.split("\n").slice(0, 2).join(" ").slice(0, 60) : "");

  function handlePinClick(e: MouseEvent) {
    e.stopPropagation();
    togglePin(note.id, note.pinned);
  }

  function handleDeleteClick(e: MouseEvent) {
    e.stopPropagation();
    confirmDelete();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
  class="note-card relative rounded-[10px] px-3 py-2.5 cursor-pointer transition-all duration-200 hover:scale-[1.01] active:scale-[0.99]"
  onmouseenter={() => (showActions = true)}
  onmouseleave={() => (showActions = false)}
  onclick={edit}
  role="button"
  tabindex="0"
  onkeydown={(e) => e.key === "Enter" && edit()}
>
  <div class="note-bar" style="background: {note.color}"></div>

  <div class="text-xs font-medium text-white/85 truncate pr-4">{displayTitle}</div>
  {#if preview}
    <div class="text-[11px] text-white/35 mt-0.5 line-clamp-2 leading-relaxed">{preview}</div>
  {/if}

  <div class="text-[10px] text-white/15 mt-1.5">{note.updated_at}</div>

  {#if showActions}
    <div class="absolute top-2 right-2 flex gap-1">
      <button
        onclick={handlePinClick}
        class="w-5 h-5 rounded flex items-center justify-center bg-white/[0.06] hover:bg-white/10 active:scale-90 transition-all"
        title={note.pinned ? "取消置顶" : "置顶"}
      >
        <svg class="w-3 h-3" style="color: {note.pinned ? 'var(--color-accent-primary)' : 'rgba(255,255,255,0.4)'}" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
          <path d="M16 12V4h1V2H7v2h1v8l-2 2v2h5.2v6h1.6v-6H18v-2l-2-2z"/>
        </svg>
      </button>
      <button
        onclick={handleDeleteClick}
        class="w-5 h-5 rounded flex items-center justify-center bg-white/[0.06] hover:bg-red-500/30 active:scale-90 transition-all text-white/40"
        title="删除"
      >
        <svg class="w-3 h-3" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="3 6 5 6 21 6"></polyline>
          <path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"></path>
        </svg>
      </button>
    </div>
  {:else if note.pinned}
    <div class="absolute top-2 right-2">
      <svg class="w-3 h-3 text-accent-primary/50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
        <path d="M16 12V4h1V2H7v2h1v8l-2 2v2h5.2v6h1.6v-6H18v-2l-2-2z"/>
      </svg>
    </div>
  {/if}
</div>

<style>
  .note-card {
    border: 1px solid var(--color-border-default);
    background: var(--color-surface-3);
    position: relative;
    overflow: hidden;
  }

  .note-card:hover {
    border-color: var(--color-border-strong);
  }

  .note-bar {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 3px;
    border-radius: 3px 0 0 3px;
  }
</style>
