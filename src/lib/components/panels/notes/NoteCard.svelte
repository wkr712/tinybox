<script lang="ts">
  import { editingNoteId, deleteNote, togglePin } from "../../../stores/notes";
  import type { Note } from "../../../types/note";
  import { pressEffect } from "../../../utils/pressEffect";
  import { ask } from "@tauri-apps/plugin-dialog";

  let { note }: { note: Note } = $props();

  let showActions = $state(false);
  let pinAnimating = $state(false);

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
    pinAnimating = true;
    setTimeout(() => (pinAnimating = false), 500);
    togglePin(note.id, note.pinned);
  }

  function handleDeleteClick(e: MouseEvent) {
    e.stopPropagation();
    confirmDelete();
  }

  function handleActionsEnter() { showActions = true; }
  function handleActionsLeave() { showActions = false; }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
  class="group note-card relative rounded-xl p-3 cursor-pointer transition-all duration-200 hover:scale-[1.01] active:scale-[0.99] border"
  style="background: {note.color}40; border-color: {note.color}60;"
  onmouseenter={handleActionsEnter}
  onmouseleave={handleActionsLeave}
  onclick={edit}
  role="button"
  tabindex="0"
  onkeydown={(e) => e.key === "Enter" && edit()}
>
  {#if note.pinned && !showActions}
    <div class="absolute top-1.5 right-1.5 pin-icon {pinAnimating ? 'pin-drop' : ''}">
      <svg class="w-3 h-3 text-accent-primary/60" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
        <path d="M16 12V4h1V2H7v2h1v8l-2 2v2h5.2v6h1.6v-6H18v-2l-2-2z"/>
      </svg>
    </div>
  {/if}

  <div class="text-xs font-medium text-white/90 truncate pr-4">{displayTitle}</div>
  {#if preview}
    <div class="text-[11px] text-white/40 mt-1 line-clamp-2 leading-relaxed">{preview}</div>
  {/if}

  <div class="text-[10px] text-white/20 mt-2">{note.updated_at}</div>

  {#if showActions}
    <div class="absolute top-1.5 right-1.5 flex gap-1">
      <button
        onclick={handlePinClick}
        class="w-5 h-5 rounded flex items-center justify-center bg-white/10 hover:bg-white/20 active:scale-90 transition-all"
        title={note.pinned ? "取消置顶" : "置顶"}
        use:pressEffect={{ scale: 0.85 }}
      >
        <svg class="w-3 h-3 pin-icon {pinAnimating ? 'pin-drop' : ''}" style="color: {note.pinned ? 'var(--color-accent-primary)' : 'rgba(255,255,255,0.5)'}" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
          <path d="M16 12V4h1V2H7v2h1v8l-2 2v2h5.2v6h1.6v-6H18v-2l-2-2z"/>
        </svg>
      </button>
      <button
        onclick={handleDeleteClick}
        class="w-5 h-5 rounded flex items-center justify-center bg-white/10 hover:bg-red-500/40 active:scale-90 transition-all text-white/50"
        title="删除"
      >
        <svg class="w-3 h-3" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="3 6 5 6 21 6"></polyline>
          <path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"></path>
        </svg>
      </button>
    </div>
  {/if}
</div>

<style>
  .pin-icon.pin-drop {
    animation: pin-drop 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  @keyframes pin-drop {
    0% { transform: rotate(-90deg) scale(0.5); opacity: 0; }
    60% { transform: rotate(10deg) scale(1.1); }
    100% { transform: rotate(0deg) scale(1); opacity: 1; }
  }
</style>
