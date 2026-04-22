<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { activePanel } from "../../stores/app";
  import NotesPanel from "./notes/NotesPanel.svelte";
  import TodoPanel from "./todo/TodoPanel.svelte";
  import ClipboardPanel from "./clipboard/ClipboardPanel.svelte";
  import DropZonePanel from "./dropzone/DropZonePanel.svelte";
  import MusicPanel from "./music/MusicPanel.svelte";
  import SettingsPanel from "./settings/SettingsPanel.svelte";

  let current = $state<string | null>(null);
  let unsubs: (() => void)[] = [];

  onMount(() => {
    unsubs.push(activePanel.subscribe((v) => (current = v)));
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  const panelTitles: Record<string, string> = {
    notes: "便签",
    todo: "待办计时",
    clipboard: "剪贴板历史",
    dropzone: "文件暂存",
    music: "网易云音乐",
    settings: "设置",
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
      <DropZonePanel />
    {:else if current === "music"}
      <MusicPanel />
    {:else if current === "settings"}
      <SettingsPanel />
    {/if}
  </div>
</div>
