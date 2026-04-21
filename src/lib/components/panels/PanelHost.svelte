<script lang="ts">
  import { activePanel } from "../../stores/app";
  import NotesPanel from "./notes/NotesPanel.svelte";
  import TodoPanel from "./todo/TodoPanel.svelte";
  import ClipboardPanel from "./clipboard/ClipboardPanel.svelte";

  let current = $state<string | null>(null);
  activePanel.subscribe((v) => (current = v));

  const panelTitles: Record<string, string> = {
    notes: "便签",
    todo: "待办计时",
    clipboard: "剪贴板历史",
    dropzone: "文件暂存",
    music: "网易云音乐",
  };
</script>

<div class="h-full flex flex-col">
  <div class="px-4 pt-4 pb-2">
    <h2 class="text-sm font-semibold text-white/70 tracking-wide">
      {current ? panelTitles[current] || "" : ""}
    </h2>
  </div>

  <div class="flex-1 overflow-hidden px-4 pb-4">
    {#if current === "notes"}
      <NotesPanel />
    {:else if current === "todo"}
      <TodoPanel />
    {:else if current === "clipboard"}
      <ClipboardPanel />
    {:else if current === "dropzone"}
      <div class="text-white/40 text-sm text-center py-12">文件暂存 - v0.5.0</div>
    {:else if current === "music"}
      <div class="text-white/40 text-sm text-center py-12">网易云音乐 - v0.6.0</div>
    {/if}
  </div>
</div>
