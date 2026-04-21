export interface Todo {
  id: number;
  text: string;
  completed: number;
  priority: "low" | "normal" | "high";
  due_date: string | null;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export interface PomodoroSession {
  id: number;
  duration: number;
  type: "work" | "break" | "long_break";
  completed: number;
  started_at: string | null;
  ended_at: string | null;
  created_at: string;
}

export type PomodoroPhase = "work" | "break" | "long_break";
export type TimerState = "idle" | "running" | "paused";
