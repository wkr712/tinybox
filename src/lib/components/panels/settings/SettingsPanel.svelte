<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { settings, saveSetting } from "../../../stores/settings";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { getVersion } from "@tauri-apps/api/app";
  import { registerHotkeys } from "../../../utils/hotkeys";
  import { updateDurations } from "../../../stores/todo";
  import { setMaxHistory } from "../../../stores/clipboard";
  import { setVolume } from "../../../stores/music";

  let s = $state<any>({});
  let editing: string | null = $state(null);
  let editValue = $state("");
  let version = $state("1.0.0");
  let unsubs: (() => void)[] = [];

  onMount(async () => {
    unsubs.push(settings.subscribe((v) => (s = v)));

    try {
      version = await getVersion();
    } catch {
      // fallback
    }
    applySettings(s);
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  function applySettings(vals: any) {
    const workMin = parseInt(vals.pomodoro_work_duration) || 25;
    const breakMin = parseInt(vals.pomodoro_break_duration) || 5;
    updateDurations(workMin, breakMin);
    const max = parseInt(vals.clipboard_max_history) || 100;
    setMaxHistory(max);
    const vol = parseInt(vals.music_volume) || 80;
    setVolume(vol / 100);
  }

  async function toggleAlwaysOnTop() {
    const newVal = s.always_on_top !== "true";
    await saveSetting("always_on_top", newVal ? "true" : "false");
    await getCurrentWindow().setAlwaysOnTop(newVal);
  }

  function startEdit(key: string) {
    editing = key;
    editValue = s[key] || "";
  }

  async function saveEdit(key: string) {
    if (editValue.trim()) {
      await saveSetting(key, editValue.trim());
      await registerHotkeys();
    }
    editing = null;
  }

  function cancelEdit() {
    editing = null;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (editing && e.key === "Enter") {
      saveEdit(editing!);
    } else if (editing && e.key === "Escape") {
      cancelEdit();
    }
  }

  async function saveNumberSetting(key: string, value: string, min = 1, max = 999) {
    const num = parseInt(value) || min;
    const clamped = Math.max(min, Math.min(max, num));
    await saveSetting(key, String(clamped));
    applySettings({ ...s, [key]: String(clamped) });
  }

  async function exportData() {
    try {
      const { select } = await import("../../../utils/db");
      const { copyToClipboard } = await import("../../../stores/clipboard");
      const notes = await select("SELECT * FROM notes");
      const todos = await select("SELECT * FROM todos");
      const settingsRows = await select("SELECT * FROM settings");
      const data = JSON.stringify({ notes, todos, settings: settingsRows, exportedAt: new Date().toISOString() }, null, 2);
      await copyToClipboard(data);
    } catch (e) {
      console.error("Export failed:", e);
    }
  }
</script>

<div class="h-full flex flex-col gap-4 overflow-y-auto">
  <!-- Appearance -->
  <section>
    <div class="text-[10px] text-white/20 mb-2 px-1">外观</div>
    <div class="px-3 py-2">
      <div class="text-xs text-white/60 mb-2">主题</div>
      <div class="flex items-center gap-3">
        {#each [
          { id: "midnight", label: "深夜", color: "#00e5ff" },
          { id: "ocean", label: "海洋", color: "#4fc3f7" },
          { id: "rose", label: "玫瑰", color: "#f48fb1" },
          { id: "forest", label: "森林", color: "#81c784" },
          { id: "light", label: "明亮", color: "#0288d1" },
        ] as t (t.id)}
          <button
            onclick={() => saveSetting('theme', t.id)}
            class="flex flex-col items-center gap-1 group"
            title={t.label}
          >
            <div
              class="w-6 h-6 rounded-full transition-all {s.theme === t.id ? 'ring-2 ring-offset-1 ring-offset-dark-bg scale-110' : 'ring-1 ring-white/10 hover:scale-105'}"
              style="background: {t.color}; {s.theme === t.id ? 'ring-color: ' + t.color : ''}"
            ></div>
            <span class="text-[9px] {s.theme === t.id ? 'text-white/60' : 'text-white/20'} transition-colors">{t.label}</span>
          </button>
        {/each}
      </div>
    </div>
  </section>

  <!-- Window -->
  <section>
    <div class="text-[10px] text-white/20 mb-2 px-1">窗口</div>
    <div class="space-y-1">
      <button
        onclick={toggleAlwaysOnTop}
        class="w-full flex items-center justify-between px-3 py-2 rounded-lg hover:bg-white/[0.03] active:scale-[0.99] transition-all"
      >
        <span class="text-xs text-white/60">窗口置顶</span>
        <span class="text-[10px] {s.always_on_top === 'true' ? 'text-accent-primary' : 'text-white/20'}">
          {s.always_on_top === "true" ? "开" : "关"}
        </span>
      </button>
    </div>
  </section>

  <!-- Clipboard -->
  <section>
    <div class="text-[10px] text-white/20 mb-2 px-1">剪贴板</div>
    <div class="space-y-1">
      <button
        onclick={() => saveSetting('clipboard_monitor_enabled', s.clipboard_monitor_enabled === 'true' ? 'false' : 'true')}
        class="w-full flex items-center justify-between px-3 py-2 rounded-lg hover:bg-white/[0.03] active:scale-[0.99] transition-all"
      >
        <span class="text-xs text-white/60">剪贴板监听</span>
        <span class="text-[10px] {s.clipboard_monitor_enabled === 'true' ? 'text-accent-primary' : 'text-white/20'}">
          {s.clipboard_monitor_enabled === "true" ? "开" : "关"}
        </span>
      </button>
      <div class="flex items-center justify-between px-3 py-2">
        <span class="text-xs text-white/60">最大历史数</span>
        <input
          type="number"
          value={s.clipboard_max_history}
          onchange={(e) => saveNumberSetting('clipboard_max_history', (e.target as HTMLInputElement).value, 10, 1000)}
          class="w-16 bg-white/[0.05] text-[11px] text-white/60 px-2 py-0.5 rounded text-right outline-none focus:ring-1 focus:ring-accent-primary/30"
        />
      </div>
    </div>
  </section>

  <!-- Pomodoro -->
  <section>
    <div class="text-[10px] text-white/20 mb-2 px-1">番茄钟</div>
    <div class="space-y-1">
      <div class="flex items-center justify-between px-3 py-2">
        <span class="text-xs text-white/60">工作时长（分钟）</span>
        <input
          type="number"
          value={s.pomodoro_work_duration}
          onchange={(e) => saveNumberSetting('pomodoro_work_duration', (e.target as HTMLInputElement).value, 1, 120)}
          class="w-16 bg-white/[0.05] text-[11px] text-white/60 px-2 py-0.5 rounded text-right outline-none focus:ring-1 focus:ring-accent-primary/30"
        />
      </div>
      <div class="flex items-center justify-between px-3 py-2">
        <span class="text-xs text-white/60">休息时长（分钟）</span>
        <input
          type="number"
          value={s.pomodoro_break_duration}
          onchange={(e) => saveNumberSetting('pomodoro_break_duration', (e.target as HTMLInputElement).value, 1, 60)}
          class="w-16 bg-white/[0.05] text-[11px] text-white/60 px-2 py-0.5 rounded text-right outline-none focus:ring-1 focus:ring-accent-primary/30"
        />
      </div>
    </div>
  </section>

  <!-- Music -->
  <section>
    <div class="text-[10px] text-white/20 mb-2 px-1">音乐</div>
    <div class="flex items-center justify-between px-3 py-2">
      <span class="text-xs text-white/60">默认音量</span>
      <div class="flex items-center gap-2">
        <input
          type="range"
          min="0"
          max="100"
          value={s.music_volume || 80}
          onchange={(e) => saveNumberSetting('music_volume', (e.target as HTMLInputElement).value)}
          class="w-20 accent-accent-primary [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-2.5 [&::-webkit-slider-thumb]:h-2.5 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-accent-primary [&::-webkit-slider-runnable-track]:h-0.5 [&::-webkit-slider-runnable-track]:bg-white/10 [&::-webkit-slider-runnable-track]:rounded-full"
        />
        <span class="text-[10px] text-white/40 w-6 text-right">{s.music_volume || 80}%</span>
      </div>
    </div>
  </section>

  <!-- Hotkeys -->
  <section>
    <div class="text-[10px] text-white/20 mb-2 px-1">快捷键</div>
    <div class="space-y-1">
      {#each [
        { key: "hotkey_toggle_sidebar", label: "显示/隐藏" },
        { key: "hotkey_clipboard", label: "剪贴板历史" },
        { key: "hotkey_new_note", label: "新建便签" },
        { key: "hotkey_play_pause", label: "播放/暂停" },
        { key: "hotkey_show_lyrics", label: "显示歌词" },
      ] as item (item.key)}
        <div class="flex items-center justify-between px-3 py-2 rounded-lg hover:bg-white/[0.02]">
          <span class="text-xs text-white/60">{item.label}</span>
          {#if editing === item.key}
            <input
              type="text"
              bind:value={editValue}
              onkeydown={handleKeydown}
              class="w-32 bg-accent-primary/10 text-[10px] text-accent-primary px-2 py-0.5 rounded outline-none"
              placeholder="按键..."
            />
            <button onclick={() => saveEdit(item.key)} class="ml-1 text-[10px] text-accent-primary hover:underline active:scale-90 transition-transform">确定</button>
            <button onclick={cancelEdit} class="ml-1 text-[10px] text-white/20 hover:text-white/40 active:scale-90 transition-transform">取消</button>
          {:else}
            <div class="flex items-center gap-1">
              <span class="text-[10px] text-white/25 bg-white/[0.05] px-1.5 py-0.5 rounded">{s[item.key]}</span>
              <button onclick={() => startEdit(item.key)} class="text-white/20 hover:text-white/50" aria-label="编辑快捷键">
                <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
              </button>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </section>

  <!-- About -->
  <section class="pt-2 border-t border-white/5">
    <div class="space-y-1">
      <button
        onclick={exportData}
        class="w-full flex items-center justify-between px-3 py-2 rounded-lg hover:bg-white/[0.03] active:scale-[0.99] transition-all"
      >
        <span class="text-xs text-white/60">导出数据</span>
        <svg class="w-3 h-3 text-white/20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
      </button>
    </div>
    <div class="text-center py-2">
      <div class="text-[10px] text-white/15">TinyBox v{version}</div>
      <div class="text-[9px] text-white/10 mt-0.5">Tauri 2 + Svelte 5 + SQLite</div>
    </div>
  </section>
</div>
