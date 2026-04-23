<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    dropzoneFiles, isDragOver, loadFiles, storeFiles,
  } from "../../../stores/dropzone";
  import FileCard from "./FileCard.svelte";
  import type { DropZoneFile } from "../../../types/dropzone";

  let files = $state<DropZoneFile[]>([]);
  let dragOver = $state(false);
  let unsubs: (() => void)[] = [];

  onMount(() => {
    unsubs.push(dropzoneFiles.subscribe((v) => (files = v)));
    unsubs.push(isDragOver.subscribe((v) => (dragOver = v)));

    loadFiles();
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    dragOver = false;
    isDragOver.set(false);

    const paths: string[] = [];
    if (e.dataTransfer?.files) {
      for (let i = 0; i < e.dataTransfer.files.length; i++) {
        const f = e.dataTransfer.files[i];
        // In Tauri, we can get the path from the file object via the path property
        const path = (f as any).path as string | undefined;
        if (path) paths.push(path);
      }
    }
    if (paths.length > 0) {
      await storeFiles(paths);
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    dragOver = true;
    isDragOver.set(true);
  }

  function handleDragLeave(e: DragEvent) {
    if (e.currentTarget && e.relatedTarget && (e.currentTarget as HTMLElement).contains(e.relatedTarget as Node)) return;
    dragOver = false;
    isDragOver.set(false);
  }
</script>

<div class="h-full flex flex-col">
  <div
    class="flex-1 flex flex-col overflow-hidden rounded-xl border-2 border-dashed transition-all duration-300 {dragOver ? 'border-accent-cyan bg-accent-cyan/5 scale-[1.01]' : 'border-white/10 bg-white/[0.02]'}"
    ondrop={handleDrop}
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
  >
    {#if files.length === 0 && !dragOver}
      <div class="flex-1 flex flex-col items-center justify-center gap-3 py-8">
        <svg class="w-10 h-10 text-white/15" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"></path>
          <polyline points="17 8 12 3 7 8"></polyline>
          <line x1="12" y1="3" x2="12" y2="15"></line>
        </svg>
        <div class="text-xs text-white/25 text-center leading-relaxed">
          拖拽文件到这里<br/>临时保存，随时拖出使用
        </div>
      </div>
    {:else if dragOver}
      <div class="flex-1 flex items-center justify-center gap-2 py-8">
        <svg class="w-8 h-8 text-accent-cyan/60 animate-bounce" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"></path>
          <polyline points="17 8 12 3 7 8"></polyline>
          <line x1="12" y1="3" x2="12" y2="15"></line>
        </svg>
        <span class="text-accent-cyan text-sm">释放以保存文件</span>
      </div>
    {:else}
      <div class="flex-1 overflow-y-auto p-2 space-y-1.5">
        {#each files as file (file.id)}
          <FileCard {file} />
        {/each}
      </div>
    {/if}
  </div>
</div>
