<script lang="ts">
  import { deleteFile, copyOut, updateTags, formatSize, fileIcon } from "../../../stores/dropzone";
  import { open } from "@tauri-apps/plugin-dialog";
  import type { DropZoneFile } from "../../../types/dropzone";

  let { file }: { file: DropZoneFile } = $props();
  let showActions = $state(false);
  let editingTags = $state(false);
  let tagInput = $state(file.tags || "");

  const iconMap: Record<string, string> = {
    image: "🖼", video: "🎬", audio: "🎵", pdf: "📄", zip: "📦", file: "📎",
  };

  async function handleDelete(e: MouseEvent) {
    e.stopPropagation();
    if (confirm(`删除 ${file.file_name}？`)) {
      await deleteFile(file.id);
    }
  }

  async function handleCopyOut(e: MouseEvent) {
    e.stopPropagation();
    const selected = await open({ directory: true, title: "选择导出目录" });
    if (selected) {
      await copyOut(file.id, selected as string);
    }
  }

  function startEditTags(e: MouseEvent) {
    e.stopPropagation();
    tagInput = file.tags || "";
    editingTags = true;
  }

  async function saveTags() {
    editingTags = false;
    if (tagInput !== (file.tags || "")) {
      await updateTags(file.id, tagInput.trim());
    }
  }

  function handleDragStart(e: DragEvent) {
    if (e.dataTransfer) {
      e.dataTransfer.setData("text/plain", JSON.stringify({
        id: file.id,
        name: file.file_name,
        path: file.file_path,
      }));
    }
  }
</script>

<div
  class="flex items-center gap-2.5 px-3 py-2 rounded-lg hover:bg-white/[0.03] transition-colors cursor-grab active:cursor-grabbing group"
  draggable="true"
  ondragstart={handleDragStart}
  onmouseenter={() => (showActions = true)}
  onmouseleave={() => { showActions = false; if (editingTags) saveTags(); }}
>
  <span class="text-base shrink-0">{iconMap[fileIcon(file.mime_type)] || "📎"}</span>

  <div class="flex-1 min-w-0">
    <div class="text-xs text-white/80 truncate">{file.file_name}</div>
    <div class="text-[10px] text-white/20 mt-0.5">{formatSize(file.file_size)} · {file.created_at.slice(0, 16)}</div>
    {#if editingTags}
      <input
        type="text"
        bind:value={tagInput}
        onkeydown={(e) => { if (e.key === "Enter") saveTags(); if (e.key === "Escape") { editingTags = false; } }}
        onblur={saveTags}
        placeholder="标签（逗号分隔）"
        class="mt-1 w-full bg-white/5 text-[10px] text-white/50 px-1.5 py-0.5 rounded outline-none focus:ring-1 focus:ring-accent-cyan/30"
      />
    {:else if file.tags}
      <button
        onclick={startEditTags}
        class="mt-1 flex flex-wrap gap-1 cursor-text"
      >
        {#each file.tags.split(",").map((t: string) => t.trim()).filter(Boolean) as tag}
          <span class="text-[9px] bg-white/5 text-white/30 px-1.5 py-0.5 rounded">{tag}</span>
        {/each}
      </button>
    {:else}
      <button
        onclick={startEditTags}
        class="mt-1 text-[9px] text-white/15 hover:text-white/30 transition-colors"
      >+ 标签</button>
    {/if}
  </div>

  <div class="shrink-0 flex items-center gap-1 {showActions ? 'opacity-100' : 'opacity-0'} transition-opacity">
    <button
      onclick={handleCopyOut}
      class="w-5 h-5 rounded flex items-center justify-center hover:bg-accent-cyan/20 text-white/30 hover:text-accent-cyan transition-colors"
      title="导出"
    >
      <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"/>
        <polyline points="7 10 12 15 17 10"/>
        <line x1="12" y1="15" x2="12" y2="3"/>
      </svg>
    </button>
    <button
      onclick={handleDelete}
      class="w-5 h-5 rounded flex items-center justify-center hover:bg-red-500/30 text-white/30 hover:text-red-400 transition-colors"
      title="删除"
    >
      <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="3 6 5 6 21 6"></polyline>
        <path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"></path>
      </svg>
    </button>
  </div>
</div>
