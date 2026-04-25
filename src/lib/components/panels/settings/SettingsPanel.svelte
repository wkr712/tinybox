<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { settings, saveSetting } from "../../../stores/settings";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { getVersion } from "@tauri-apps/api/app";
  import { registerHotkeys } from "../../../utils/hotkeys";
  import { updateDurations } from "../../../stores/todo";
  import { setMaxHistory, setClipboardMonitorPaused } from "../../../stores/clipboard";
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
    } catch {}
    applySettings(s, false);
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  function applySettings(vals: any, includeVolume = false) {
    const workMin = parseInt(vals.pomodoro_work_duration) || 25;
    const breakMin = parseInt(vals.pomodoro_break_duration) || 5;
    updateDurations(workMin, breakMin);
    const max = parseInt(vals.clipboard_max_history) || 100;
    setMaxHistory(max);
    if (includeVolume) {
      const vol = parseInt(vals.music_volume) || 80;
      setVolume(vol / 100);
    }
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
    applySettings({ ...s, [key]: String(clamped) }, key === "music_volume");
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
    <div class="section-label">外观</div>
    <div class="setting-item">
      <div class="text-xs text-white/55">主题</div>
      <div class="flex items-center gap-3 mt-2">
        {#each [
          { id: "midnight", label: "深夜", color: "#00e5ff" },
          { id: "ocean", label: "海洋", color: "#38bdf8" },
          { id: "rose", label: "玫瑰", color: "#fb7185" },
          { id: "forest", label: "森林", color: "#34d399" },
          { id: "lavender", label: "薰衣草", color: "#a78bfa" },
        ] as t (t.id)}
          <button
            onclick={() => saveSetting('theme', t.id)}
            class="flex flex-col items-center gap-1.5"
            title={t.label}
          >
            <div
              class="theme-dot {s.theme === t.id ? 'active' : ''}"
              style="--dot-color: {t.color}"
            ></div>
            <span class="text-[9px] {s.theme === t.id ? 'text-white/55' : 'text-white/18'} transition-colors">{t.label}</span>
          </button>
        {/each}
      </div>
    </div>
  </section>

  <!-- Window -->
  <section>
    <div class="section-label">窗口</div>
    <div class="space-y-px">
      <button
        onclick={toggleAlwaysOnTop}
        class="setting-row"
      >
        <span class="text-xs text-white/55">窗口置顶</span>
        <span class="text-[10px] {s.always_on_top === 'true' ? 'text-accent-primary' : 'text-white/18'}">
          {s.always_on_top === "true" ? "开" : "关"}
        </span>
      </button>
    </div>
  </section>

  <!-- Clipboard -->
  <section>
    <div class="section-label">剪贴板</div>
    <div class="space-y-px">
      <button
        onclick={async () => {
          const newVal = s.clipboard_monitor_enabled === 'true' ? 'false' : 'true';
          await saveSetting('clipboard_monitor_enabled', newVal);
          await setClipboardMonitorPaused(newVal !== 'true');
        }}
        class="setting-row"
      >
        <span class="text-xs text-white/55">剪贴板监听</span>
        <span class="text-[10px] {s.clipboard_monitor_enabled === 'true' ? 'text-accent-primary' : 'text-white/18'}">
          {s.clipboard_monitor_enabled === "true" ? "开" : "关"}
        </span>
      </button>
      <div class="setting-row">
        <span class="text-xs text-white/55">最大历史数</span>
        <input
          type="number"
          value={s.clipboard_max_history}
          onchange={(e) => saveNumberSetting('clipboard_max_history', (e.target as HTMLInputElement).value, 10, 1000)}
          class="num-input"
        />
      </div>
    </div>
  </section>

  <!-- Pomodoro -->
  <section>
    <div class="section-label">番茄钟</div>
    <div class="space-y-px">
      <div class="setting-row">
        <span class="text-xs text-white/55">工作时长（分钟）</span>
        <input
          type="number"
          value={s.pomodoro_work_duration}
          onchange={(e) => saveNumberSetting('pomodoro_work_duration', (e.target as HTMLInputElement).value, 1, 120)}
          class="num-input"
        />
      </div>
      <div class="setting-row">
        <span class="text-xs text-white/55">休息时长（分钟）</span>
        <input
          type="number"
          value={s.pomodoro_break_duration}
          onchange={(e) => saveNumberSetting('pomodoro_break_duration', (e.target as HTMLInputElement).value, 1, 60)}
          class="num-input"
        />
      </div>
    </div>
  </section>

  <!-- Music -->
  <section>
    <div class="section-label">音乐</div>
    <div class="setting-row">
      <span class="text-xs text-white/55">默认音量</span>
      <div class="flex items-center gap-2">
        <input
          type="range"
          min="0"
          max="100"
          value={s.music_volume || 80}
          onchange={(e) => saveNumberSetting('music_volume', (e.target as HTMLInputElement).value)}
          class="volume-slider"
        />
        <span class="text-[10px] text-white/30 w-6 text-right">{s.music_volume || 80}%</span>
      </div>
    </div>
  </section>

  <!-- Hotkeys -->
  <section>
    <div class="section-label">快捷键</div>
    <div class="space-y-px">
      {#each [
        { key: "hotkey_toggle_sidebar", label: "显示/隐藏" },
        { key: "hotkey_clipboard", label: "剪贴板历史" },
        { key: "hotkey_new_note", label: "新建便签" },
        { key: "hotkey_play_pause", label: "播放/暂停" },
        { key: "hotkey_show_lyrics", label: "显示歌词" },
      ] as item (item.key)}
        <div class="setting-row">
          <span class="text-xs text-white/55">{item.label}</span>
          {#if editing === item.key}
            <div class="flex items-center gap-1">
              <input
                type="text"
                bind:value={editValue}
                onkeydown={handleKeydown}
                class="hotkey-input"
                placeholder="按键..."
              />
              <button onclick={() => saveEdit(item.key)} class="text-[10px] text-accent-primary hover:underline active:scale-90 transition-transform">确定</button>
              <button onclick={cancelEdit} class="text-[10px] text-white/18 hover:text-white/40 active:scale-90 transition-transform">取消</button>
            </div>
          {:else}
            <div class="flex items-center gap-1">
              <span class="hotkey-badge">{s[item.key]}</span>
              <button onclick={() => startEdit(item.key)} class="text-white/15 hover:text-white/40" aria-label="编辑快捷键">
                <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
              </button>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </section>

  <!-- About -->
  <section class="pt-3 border-t border-white/5">
    <button
      onclick={exportData}
      class="setting-row"
    >
      <span class="text-xs text-white/55">导出数据</span>
      <svg class="w-3 h-3 text-white/15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
    </button>
    <div class="text-center py-3">
      <div class="text-[10px] text-white/25">TinyBox v{version}</div>
      <div class="text-[9px] text-white/12 mt-0.5">Tauri 2 + Svelte 5 + SQLite</div>
    </div>
  </section>
</div>

<style>
  .section-label {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.18);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    margin-bottom: 6px;
    padding: 0 4px;
  }

  .setting-item {
    padding: 0 12px 10px;
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 7px 12px;
    border-radius: 6px;
    transition: background 0.15s ease;
    background: transparent;
    border: none;
    width: 100%;
    cursor: pointer;
  }

  .setting-row:hover {
    background: rgba(255, 255, 255, 0.02);
  }

  .setting-row:active {
    transform: scale(0.995);
  }

  .theme-dot {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    background: var(--dot-color);
    border: 2px solid rgba(255, 255, 255, 0.08);
    transition: all 0.2s ease;
  }

  .theme-dot:hover {
    transform: scale(1.08);
  }

  .theme-dot.active {
    border-color: transparent;
    box-shadow: 0 0 0 2px var(--color-dark-bg), 0 0 0 4px var(--dot-color);
    transform: scale(1.1);
  }

  .num-input {
    width: 52px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.06);
    font-size: 10px;
    color: rgba(255, 255, 255, 0.5);
    padding: 2px 6px;
    border-radius: 4px;
    text-align: right;
    outline: none;
    transition: all 0.2s ease;
  }

  .num-input:focus {
    border-color: color-mix(in srgb, var(--color-accent-primary) 30%, transparent);
  }

  .volume-slider {
    width: 72px;
    accent-color: var(--color-accent-primary);
  }

  .hotkey-input {
    width: 80px;
    background: color-mix(in srgb, var(--color-accent-primary) 8%, transparent);
    font-size: 10px;
    color: var(--color-accent-primary);
    padding: 2px 6px;
    border-radius: 4px;
    outline: none;
    border: none;
  }

  .hotkey-badge {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.2);
    background: rgba(255, 255, 255, 0.04);
    padding: 2px 6px;
    border-radius: 3px;
  }
</style>
