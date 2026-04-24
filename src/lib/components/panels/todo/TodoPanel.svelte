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
      startInterval();
    } else if (runState === "running") {
      timerState.set("paused");
      if (timerInterval) {
        clearInterval(timerInterval);
        timerInterval = null;
      }
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
  const phaseColor: Record<string, string> = { work: "#00e5ff", break: "#4caf50", long_break: "#b388ff" };
</script>

<div class="h-full flex flex-col">
  <!-- Pomodoro Timer -->
  <div class="flex flex-col items-center py-3 mb-3 rounded-xl bg-white/[0.03] border border-white/[0.06]">
    <div class="flex gap-3 mb-3 text-[10px]">
      {#each ["work", "break", "long_break"] as p}
        <button
          onclick={() => { if (runState !== "running") setPhase(p as "work" | "break" | "long_break"); }}
          class="px-2.5 py-0.5 rounded-full active:scale-95 transition-all {phase === p ? 'text-white' : 'text-white/30 hover:text-white/50'}"
          style={phase === p ? `background: ${phaseColor[p]}20; color: ${phaseColor[p]}` : ''}
        >
          {phaseLabel[p]}
        </button>
      {/each}
    </div>

    <div class="relative w-28 h-28 flex items-center justify-center">
      <svg class="absolute inset-0 -rotate-90" viewBox="0 0 100 100">
        <circle cx="50" cy="50" r="44" fill="none" stroke="rgba(255,255,255,0.06)" stroke-width="4"/>
        <circle cx="50" cy="50" r="44" fill="none" stroke={phaseColor[phase]} stroke-width="4"
          stroke-linecap="round" stroke-dasharray={2 * Math.PI * 44}
          stroke-dashoffset={2 * Math.PI * 44 * (1 - progress / 100)}
          style="transition: stroke-dashoffset 1s linear"
        />
      </svg>
      <div class="text-2xl font-light tracking-wider text-white/90 font-mono">{formatTime(seconds)}</div>
    </div>

    <div class="flex items-center gap-2 mt-3">
      <button onclick={handleReset} class="px-2 py-1 rounded-lg text-[10px] text-white/30 hover:text-white/60 hover:bg-white/5 active:scale-95 transition-all">重置</button>
      <button onclick={toggleRun} class="glow-button text-xs px-5 py-1.5">
        {runState === "running" ? "暂停" : "开始"}
      </button>
    </div>

    <div class="flex items-center gap-3 mt-2 text-[10px] text-white/25">
      <span>番茄 {count}</span>
      <span>·</span>
      <span>今日专注 {sessions} 次</span>
    </div>
  </div>

  <!-- Add Todo -->
  <div class="flex items-center gap-1.5 mb-3">
    <div class="flex-1 flex items-center bg-white/5 border border-white/10 rounded-lg overflow-hidden">
      <select bind:value={newPriority} class="bg-transparent text-[10px] px-1.5 py-1.5 outline-none text-white/50 cursor-pointer" style="appearance: none; background-image: url(&quot;data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='8' height='8' viewBox='0 0 24 24' fill='none' stroke='rgba(255,255,255,0.3)' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'/%3E%3C/svg%3E&quot;); background-repeat: no-repeat; background-position: right 2px center; padding-right: 12px;">
        <option value="low" style="background: #16162a">↓ 低</option>
        <option value="normal" style="background: #16162a">· 中</option>
        <option value="high" style="background: #16162a">↑ 高</option>
      </select>
      <input
        type="date"
        bind:value={newDueDate}
        class="bg-transparent text-[10px] px-1 py-1.5 outline-none text-white/50 cursor-pointer"
      />
      <input
        type="text"
        bind:value={newText}
        onkeydown={handleKeydown}
        placeholder="添加任务..."
        class="flex-1 bg-transparent text-xs text-white/80 placeholder-white/25 outline-none py-1.5 pr-2"
      />
    </div>
    <button onclick={handleAdd} class="glow-button text-xs px-3 py-1.5 shrink-0">添加</button>
  </div>

  <!-- Filter -->
  <div class="flex gap-1 mb-2 text-[10px]">
    {#each [["all", "全部"], ["active", "进行中"], ["completed", "已完成"]] as [f, l]}
      <button
        onclick={() => todoFilter.set(f as "all" | "active" | "completed")}
        class="px-2 py-0.5 rounded-full active:scale-95 transition-all {filter === f ? 'bg-white/10 text-white/80' : 'text-white/30 hover:text-white/50'}"
      >
        {l}
      </button>
    {/each}
  </div>

  <!-- Todo List -->
  <div class="flex-1 overflow-y-auto space-y-1.5">
    {#if todoLoading}
      {#each { length: 5 } as _}
        <div class="flex items-center gap-2 px-2.5 py-2">
          <SkeletonLine width="16px" height="16px" rounded="50%" />
          <SkeletonLine width="50%" height="12px" />
          <SkeletonLine width="30%" height="10px" />
        </div>
      {/each}
    {:else}
    {#each filtered as todo (todo.id)}
      <TodoItem {todo} />
    {/each}
    {/if}
    {#if !todoLoading && filtered.length === 0}
      <div class="text-white/30 text-xs text-center py-6">
        {filter === "completed" ? "没有已完成的任务" : "没有任务，添加一个吧"}
      </div>
    {/if}
  </div>
</div>
