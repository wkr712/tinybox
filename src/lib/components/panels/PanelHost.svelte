<script lang="ts">
  import { activePanel } from "../../stores/app";

  let current = $state<string | null>(null);
  activePanel.subscribe((v) => (current = v));

  const panelComponents: Record<string, string> = {
    notes: "便签",
    todo: "待办计时",
    clipboard: "剪贴板历史",
    dropzone: "文件暂存",
    music: "网易云音乐",
  };
</script>

<div class="h-full flex flex-col p-4">
  <h2 class="text-sm font-semibold text-white/70 mb-4 tracking-wide">
    {current ? panelComponents[current] || "" : ""}
  </h2>

  <div class="flex-1 overflow-y-auto">
    {#if current === "notes"}
      <div class="text-white/40 text-sm">便签功能 - v0.2.0</div>
    {:else if current === "todo"}
      <div class="text-white/40 text-sm">待办计时 - v0.3.0</div>
    {:else if current === "clipboard"}
      <div class="text-white/40 text-sm">剪贴板历史 - v0.4.0</div>
    {:else if current === "dropzone"}
      <div class="text-white/40 text-sm">文件暂存 - v0.5.0</div>
    {:else if current === "music"}
      <div class="text-white/40 text-sm">网易云音乐 - v0.6.0</div>
    {/if}
  </div>
</div>
