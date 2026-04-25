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
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="dropzone-area {dragOver ? 'drag-active' : ''}"
    role="region"
    aria-label="文件拖放区域"
    ondrop={handleDrop}
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
  >
    {#if files.length === 0 && !dragOver}
      <div class="empty-drop">
        <div class="empty-drop-circle">
          <svg class="w-6 h-6" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"></path>
            <polyline points="17 8 12 3 7 8"></polyline>
            <line x1="12" y1="3" x2="12" y2="15"></line>
          </svg>
        </div>
        <div class="text-xs text-white/20 text-center leading-relaxed">
          拖拽文件到这里<br/>临时保存，随时拖出使用
        </div>
      </div>
    {:else if dragOver}
      <div class="flex-1 flex items-center justify-center gap-2">
        <svg class="w-6 h-6 text-accent-primary/50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"></path>
          <polyline points="17 8 12 3 7 8"></polyline>
          <line x1="12" y1="3" x2="12" y2="15"></line>
        </svg>
        <span class="text-accent-primary/70 text-sm">释放以保存</span>
      </div>
    {:else}
      <div class="flex-1 overflow-y-auto p-2 space-y-1">
        {#each files as file (file.id)}
          <FileCard {file} />
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .dropzone-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    border-radius: 10px;
    border: 2px dashed rgba(255, 255, 255, 0.08);
    background: var(--color-surface-3);
    transition: all 0.2s ease;
  }

  .dropzone-area.drag-active {
    border-color: color-mix(in srgb, var(--color-accent-primary) 40%, transparent);
    background: color-mix(in srgb, var(--color-accent-primary) 3%, transparent);
  }

  .empty-drop {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 32px 0;
  }

  .empty-drop-circle {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    border: 1.5px dashed rgba(255, 255, 255, 0.1);
    display: flex;
    align-items: center;
    justify-content: center;
    color: color-mix(in srgb, var(--color-accent-primary) 20%, transparent);
  }
</style>
