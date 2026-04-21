import { writable } from "svelte/store";
import { select, execute } from "../utils/db";
import type { Todo, PomodoroSession, PomodoroPhase, TimerState } from "../types/todo";

// Todos
export const todos = writable<Todo[]>([]);
export const todoFilter = writable<"all" | "active" | "completed">("all");

export async function loadTodos() {
  const result = await select<Todo>(
    "SELECT * FROM todos ORDER BY completed ASC, sort_order ASC, created_at DESC"
  );
  todos.set(result);
}

export async function addTodo(text: string, priority: "low" | "normal" | "high" = "normal", dueDate: string | null = null) {
  await execute(
    "INSERT INTO todos (text, priority, due_date) VALUES (?, ?, ?)",
    [text, priority, dueDate]
  );
  await loadTodos();
}

export async function toggleTodo(id: number, completed: number) {
  await execute("UPDATE todos SET completed = ?, updated_at = datetime('now', 'localtime') WHERE id = ?", [
    completed ? 0 : 1,
    id,
  ]);
  await loadTodos();
}

export async function deleteTodo(id: number) {
  await execute("DELETE FROM todos WHERE id = ?", [id]);
  await loadTodos();
}

export async function updateTodoPriority(id: number, priority: "low" | "normal" | "high") {
  await execute("UPDATE todos SET priority = ?, updated_at = datetime('now', 'localtime') WHERE id = ?", [
    priority,
    id,
  ]);
  await loadTodos();
}

// Pomodoro Timer
export const timerState = writable<TimerState>("idle");
export const timerPhase = writable<PomodoroPhase>("work");
export const timerSeconds = writable(25 * 60);
export const timerTotal = writable(25 * 60);
export const pomodoroCount = writable(0);
export const todaySessions = writable(0);

const DURATIONS: Record<PomodoroPhase, number> = {
  work: 25 * 60,
  break: 5 * 60,
  long_break: 15 * 60,
};

export function setPhase(phase: PomodoroPhase) {
  timerPhase.set(phase);
  timerTotal.set(DURATIONS[phase]);
  timerSeconds.set(DURATIONS[phase]);
  timerState.set("idle");
}

export function startTimer() {
  timerState.set("running");
}

export function pauseTimer() {
  timerState.set("paused");
}

export function resetTimer() {
  const phase = getPhase();
  timerSeconds.set(DURATIONS[phase]);
  timerState.set("idle");
}

function getPhase(): PomodoroPhase {
  let p: PomodoroPhase = "work";
  timerPhase.subscribe((v) => (p = v))();
  return p;
}

export async function completePhase() {
  const phase = getPhase();
  let count = 0;
  pomodoroCount.subscribe((v) => (count = v))();

  await execute(
    "INSERT INTO pomodoro_sessions (duration, type, completed, started_at, ended_at) VALUES (?, ?, 1, datetime('now', 'localtime'), datetime('now', 'localtime'))",
    [DURATIONS[phase] / 60, phase]
  );

  if (phase === "work") {
    count++;
    pomodoroCount.set(count);
    if (count % 4 === 0) {
      setPhase("long_break");
    } else {
      setPhase("break");
    }
  } else {
    setPhase("work");
  }

  await loadTodaySessions();
}

export async function loadTodaySessions() {
  const result = await select<{ cnt: number }[]>(
    "SELECT COUNT(*) as cnt FROM pomodoro_sessions WHERE type = 'work' AND completed = 1 AND date(started_at) = date('now', 'localtime')"
  );
  todaySessions.set(result[0]?.cnt ?? 0);
}

export function getFilteredTodos(allTodos: Todo[], filter: "all" | "active" | "completed"): Todo[] {
  if (filter === "active") return allTodos.filter((t) => !t.completed);
  if (filter === "completed") return allTodos.filter((t) => t.completed);
  return allTodos;
}

export function formatTime(seconds: number): string {
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return `${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
}
