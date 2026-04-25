<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    todos, todoFilter, loadTodos, addTodo, getFilteredTodos,
    timerState, timerPhase, timerSeconds, timerTotal,
    pomodoroCount, todaySessions, loadTodaySessions,
    resetTimer, completePhase, setPhase, formatTime,
  } from "../../../stores/todo";
  import type { Todo } from "../../../types/todo";
  import TodoItem from "./TodoItem.svelte";
  import SkeletonLine from "../../shared/SkeletonLine.svelte";

  let allTodos = $state<Todo[]>([]);
  let filter = $state<"all" | "active" | "completed">("all");
  let newText = $state("");
  let newPriority = $state<"low" | "normal" | "high">("normal");
  let newDueDate = $state<string>("");
  let todoLoading = $state(true);

  let seconds = $state(25 * 60);
  let total = $state(25 * 60);
  let runState = $state<"idle" | "running" | "paused">("idle");
  let phase = $state<"work" | "break" | "long_break">("work");
  let count = $state(0);
  let sessions = $state(0);
  let unsubs: (() => void)[] = [];

  let filtered = $derived(getFilteredTodos(allTodos, filter));
  let progress = $derived(total > 0 ? ((total - seconds) / total) * 100 : 0);

  let timerInterval: ReturnType<typeof setInterval> | null = null;

  onMount(async () => {
    unsubs.push(todos.subscribe((v) => { allTodos = v; todoLoading = false; }));
    unsubs.push(todoFilter.subscribe((v) => (filter = v)));
    unsubs.push(timerSeconds.subscribe((v) => (seconds = v)));
    unsubs.push(timerTotal.subscribe((v) => (total = v)));
    unsubs.push(timerState.subscribe((v) => {
      runState = v;
      if (v === "running" && !timerInterval) {
        startInterval();
      } else if (v !== "running" && timerInterval) {
        clearInterval(timerInterval);
        timerInterval = null;
      }
    }));
    unsubs.push(timerPhase.subscribe((v) => (phase = v)));
    unsubs.push(pomodoroCount.subscribe((v) => (count = v)));
    unsubs.push(todaySessions.subscribe((v) => (sessions = v)));

    await loadTodos();
    todoLoading = false;
    loadTodaySessions();
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
    if (timerInterval) clearInterval(timerInterval);
  });

  function handleAdd() {
    const text = newText.trim();
    if (!text) return;
    addTodo(text, newPriority, newDueDate || null);
    newText = "";
    newDueDate = "";
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") handleAdd();
  }

  function startInterval() {
    if (timerInterval) return;
    timerInterval = setInterval(() => {
      let shouldComplete = false;
      timerSeconds.update((s) => {
        if (s <= 1) {
          shouldComplete = true;
          return 0;
        }
        return s - 1;
      });
      if (shouldComplete) {
        if (timerInterval) clearInterval(timerInterval);
        timerInterval = null;
        completePhase();
      }
    }, 1000);
  }

  function toggleRun() {
    if (runState === "idle" || runState === "paused") {
      timerState.set("running");
      // startInterval is handled by the timerState subscription above
    } else if (runState === "running") {
      timerState.set("paused");
    }
  }

  function handleReset() {
    if (timerInterval) {
      clearInterval(timerInterval);
      timerInterval = null;
    }
    resetTimer();
  }

  const phaseLabel: Record<string, string> = { work: "专注", break: "休息", long_break: "长休息" };
  const phaseColor: Record<string, string> = { work: "#00e5ff", break: "#66bb6a", long_break: "#b388ff" };
</script>

<div class="h-full flex flex-col">
  <!-- Pomodoro Timer -->
  <div class="pomo-card">
    <div class="flex gap-2 mb-3">
      {#each ["work", "break", "long_break"] as p}
        <button
          onclick={() => { if (runState !== "running") setPhase(p as "work" | "break" | "long_break"); }}
          class="pomo-tab {phase === p ? 'active' : ''}"
          style={phase === p ? `color: ${phaseColor[p]}` : ''}
        >
          {phaseLabel[p]}
        </button>
      {/each}
    </div>

    <div class="relative w-24 h-24 flex items-center justify-center mx-auto">
      <svg class="absolute inset-0 -rotate-90" viewBox="0 0 100 100">
        <circle cx="50" cy="50" r="44" fill="none" stroke="rgba(255,255,255,0.05)" stroke-width="3"/>
        <circle cx="50" cy="50" r="44" fill="none" stroke={phaseColor[phase]} stroke-width="3"
          stroke-linecap="round" stroke-dasharray={2 * Math.PI * 44}
          stroke-dashoffset={2 * Math.PI * 44 * (1 - progress / 100)}
          style="transition: stroke-dashoffset 1s linear"
        />
      </svg>
      <div class="text-xl font-light tracking-wider text-white/85 font-mono">{formatTime(seconds)}</div>
    </div>

    <div class="flex items-center gap-2 mt-3 justify-center">
      <button onclick={handleReset} class="text-[10px] text-white/25 hover:text-white/50 hover:bg-white/[0.04] px-2 py-1 rounded-lg active:scale-95 transition-all">重置</button>
      <button onclick={toggleRun} class="glow-button text-xs px-5 py-1.5">
        {runState === "running" ? "暂停" : "开始"}
      </button>
    </div>

    <div class="flex items-center justify-center gap-3 mt-2 text-[10px] text-white/20">
      <span>番茄 {count}</span>
      <span class="text-white/8">·</span>
      <span>今日专注 {sessions} 次</span>
    </div>
  </div>

  <!-- Add Todo -->
  <div class="mb-3 space-y-1.5">
    <div class="flex items-center gap-2">
      <select bind:value={newPriority} class="input-compact" style="width: auto; padding-right: 18px; background-image: url(&quot;data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='8' height='8' viewBox='0 0 24 24' fill='none' stroke='rgba(255,255,255,0.25)' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'/%3E%3C/svg%3E&quot;); background-repeat: no-repeat; background-position: right 5px center;">
        <option value="low" style="background: var(--color-dark-bg)">↓ 低</option>
        <option value="normal" style="background: var(--color-dark-bg)">· 中</option>
        <option value="high" style="background: var(--color-dark-bg)">↑ 高</option>
      </select>
      <input
        type="date"
        bind:value={newDueDate}
        class="input-compact"
        style="width: auto"
      />
    </div>
    <div class="flex items-center gap-2">
      <input
        type="text"
        bind:value={newText}
        onkeydown={handleKeydown}
        placeholder="添加任务..."
        class="input-compact flex-1"
      />
      <button onclick={handleAdd} class="glow-button text-xs px-3 py-1.5 shrink-0">添加</button>
    </div>
  </div>

  <!-- Filter -->
  <div class="flex gap-1 mb-2">
    {#each [["all", "全部"], ["active", "进行中"], ["completed", "已完成"]] as [f, l]}
      <button
        onclick={() => todoFilter.set(f as "all" | "active" | "completed")}
        class="filter-tab {filter === f ? 'active' : ''}"
      >
        {l}
      </button>
    {/each}
  </div>

  <!-- Todo List -->
  <div class="flex-1 overflow-y-auto space-y-1">
    {#if todoLoading}
      {#each { length: 5 } as _}
        <div class="flex items-center gap-2 px-2.5 py-2">
          <SkeletonLine width="18px" height="18px" rounded="50%" />
          <SkeletonLine width="50%" height="12px" />
        </div>
      {/each}
    {:else}
    {#each filtered as todo (todo.id)}
      <TodoItem {todo} />
    {/each}
    {/if}
    {#if !todoLoading && filtered.length === 0}
      <div class="empty-state">
        <span>{filter === "completed" ? "没有已完成的任务" : "没有任务，添加一个吧"}</span>
      </div>
    {/if}
  </div>
</div>

<style>
  .pomo-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 12px 0 10px;
    margin-bottom: 12px;
    border-radius: 10px;
    background: var(--color-surface-3);
    border: 1px solid var(--color-border-subtle);
  }

  .pomo-tab {
    font-size: 10px;
    padding: 2px 10px;
    border-radius: 999px;
    color: rgba(255, 255, 255, 0.25);
    transition: all 0.15s ease;
    background: transparent;
    border: none;
    cursor: pointer;
  }

  .pomo-tab:hover {
    color: rgba(255, 255, 255, 0.45);
  }

  .pomo-tab.active {
    background: rgba(255, 255, 255, 0.06);
  }

  .input-compact {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    font-size: 10px;
    padding: 6px 8px;
    border-radius: 8px;
    outline: none;
    color: rgba(255, 255, 255, 0.5);
    transition: all 0.2s ease;
    appearance: none;
  }

  .input-compact::placeholder {
    color: rgba(255, 255, 255, 0.2);
  }

  .input-compact:focus {
    border-color: color-mix(in srgb, var(--color-accent-primary) 25%, transparent);
  }

  .filter-tab {
    font-size: 10px;
    padding: 2px 8px;
    border-radius: 999px;
    color: rgba(255, 255, 255, 0.25);
    transition: all 0.15s ease;
    background: transparent;
    border: none;
    cursor: pointer;
  }

  .filter-tab:hover {
    color: rgba(255, 255, 255, 0.45);
  }

  .filter-tab.active {
    background: var(--color-surface-8);
    color: rgba(255, 255, 255, 0.75);
  }

  .empty-state {
    text-align: center;
    padding: 24px 0;
    color: rgba(255, 255, 255, 0.25);
    font-size: 12px;
  }
</style>
