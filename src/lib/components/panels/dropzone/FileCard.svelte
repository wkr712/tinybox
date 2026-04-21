<script lang="ts">
  import { deleteFile, formatSize, fileIcon } from "../../../stores/dropzone";
  import type { DropZoneFile } from "../../../types/dropzone";

  let { file }: { file: DropZoneFile } = $props();
  let showActions = $state(false);

  const iconMap: Record<string, string> = {
    image: "🖼", video: "🎬", audio: "🎵", pdf: "📄", zip: "📦", file: "📎",
  };

  async function handleDelete(e: MouseEvent) {
    e.stopPropagation();
    if (confirm(`删除 ${file.file_name}？`)) {
      await deleteFile(file.id);
    }
  }

  function handleDragStart(e: DragEvent) {
    // Store the file info for drag-out
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
  onmouseleave={() => (showActions = false)}
>
  <span class="text-base shrink-0">{iconMap[fileIcon(file.mime_type)] || "📎"}</span>

  <div class="flex-1 min-w-0">
    <div class="text-xs text-white/80 truncate">{file.file_name}</div>
    <div class="text-[10px] text-white/20 mt-0.5">{formatSize(file.file_size)} · {file.created_at.slice(0, 16)}</div>
  </div>

  <div class="shrink-0 {showActions ? 'opacity-100' : 'opacity-0'} transition-opacity">
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
