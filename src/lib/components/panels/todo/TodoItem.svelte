<script lang="ts">
  import { toggleTodo, deleteTodo, updateTodoPriority } from "../../../stores/todo";
  import type { Todo } from "../../../types/todo";
  import { pressEffect } from "../../../utils/pressEffect";

  let { todo }: { todo: Todo } = $props();

  const priorityLabels: Record<string, string> = { low: "↓", normal: "·", high: "↑" };
  const priorityColors: Record<string, string> = { low: "#60a5fa", normal: "rgba(255,255,255,0.4)", high: "#f87171" };

  let checkBounce = $state(false);

  function handleToggle() {
    checkBounce = true;
    setTimeout(() => (checkBounce = false), 400);
    toggleTodo(todo.id, todo.completed);
  }

  function handleDelete() {
    deleteTodo(todo.id);
  }

  function cyclePriority() {
    const order: Array<"low" | "normal" | "high"> = ["low", "normal", "high"];
    const idx = order.indexOf(todo.priority);
    const next = order[(idx + 1) % 3];
    updateTodoPriority(todo.id, next);
  }
</script>

<div class="todo-item flex items-center gap-2 px-2.5 py-2 rounded-lg hover:bg-white/[0.03] transition-all duration-200 group {todo.completed ? 'opacity-50' : ''}"
>
  <button
    onclick={handleToggle}
    class="todo-check w-4 h-4 rounded-full border-2 flex items-center justify-center shrink-0 transition-colors {checkBounce ? 'bounce' : ''}"
    style="border-color: {todo.completed ? 'var(--color-accent-primary)' : 'rgba(255,255,255,0.2)'}; background: {todo.completed ? 'var(--color-accent-primary)' : 'transparent'}"
    use:pressEffect={{ scale: 0.85 }}
  >
    {#if todo.completed}
      <svg class="check-svg w-2.5 h-2.5 text-[#12121c]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="20 6 9 17 4 12"></polyline>
      </svg>
    {/if}
  </button>

  <button onclick={cyclePriority} class="text-xs shrink-0 hover:scale-125 active:scale-90 transition-transform" title="切换优先级" style="color: {priorityColors[todo.priority]}">
    {priorityLabels[todo.priority]}
  </button>

  <span class="flex-1 text-xs text-white/80 {todo.completed ? 'line-through' : ''} truncate">
    {todo.text}
  </span>

  {#if todo.due_date}
    <span class="text-[10px] text-white/20 shrink-0">{todo.due_date}</span>
  {/if}

  <button
    onclick={handleDelete}
    class="todo-delete w-4 h-4 rounded flex items-center justify-center opacity-0 group-hover:opacity-100 focus-visible:opacity-100 hover:bg-red-500/30 active:scale-90 transition-all text-white/30 hover:text-red-400 shrink-0"
    title="删除"
    aria-label="删除任务"
  >
    <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <line x1="18" y1="6" x2="6" y2="18"></line>
      <line x1="6" y1="6" x2="18" y2="18"></line>
    </svg>
  </button>
</div>

<style>
  .todo-check.bounce {
    animation: check-bounce 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .check-svg {
    animation: draw-check 0.3s ease-out;
  }

  @keyframes check-bounce {
    0% { transform: scale(1); }
    40% { transform: scale(1.2); }
    100% { transform: scale(1); }
  }

  @keyframes draw-check {
    from { stroke-dashoffset: 24; }
    to { stroke-dashoffset: 0; }
  }

  .check-svg {
    stroke-dasharray: 24;
    stroke-dashoffset: 0;
  }
</style>
