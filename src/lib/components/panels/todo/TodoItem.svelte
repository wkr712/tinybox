<script lang="ts">
  import { toggleTodo, deleteTodo, updateTodoPriority } from "../../../stores/todo";
  import type { Todo } from "../../../types/todo";

  let { todo }: { todo: Todo } = $props();

  const priorityColors: Record<string, string> = { low: "#60a5fa", normal: "transparent", high: "#f87171" };

  function handleToggle() {
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

<div class="todo-item flex items-center gap-2.5 px-2.5 py-2 rounded-lg hover:bg-white/[0.03] transition-all duration-200 group {todo.completed ? 'opacity-40' : ''}">
  <button
    onclick={handleToggle}
    class="todo-check w-[18px] h-[18px] rounded-full border-2 flex items-center justify-center shrink-0 transition-all"
    style="border-color: {todo.completed ? 'var(--color-accent-primary)' : 'rgba(255,255,255,0.15)'}; background: {todo.completed ? 'var(--color-accent-primary)' : 'transparent'}"
  >
    {#if todo.completed}
      <svg class="w-2.5 h-2.5 text-[#12121c]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="20 6 9 17 4 12"></polyline>
      </svg>
    {/if}
  </button>

  <button
    onclick={cyclePriority}
    class="w-2 h-2 rounded-full shrink-0 transition-transform hover:scale-150 active:scale-75"
    style="background: {priorityColors[todo.priority]}"
    title="切换优先级"
  ></button>

  <span class="flex-1 text-xs text-white/80 truncate">
    {todo.text}
  </span>

  {#if todo.due_date}
    <span class="text-[10px] text-white/15 shrink-0">{todo.due_date}</span>
  {/if}

  <button
    onclick={handleDelete}
    class="w-4 h-4 rounded flex items-center justify-center opacity-0 group-hover:opacity-100 hover:bg-red-500/20 active:scale-90 transition-all text-white/25 hover:text-red-400 shrink-0"
    title="删除"
    aria-label="删除任务"
  >
    <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <line x1="18" y1="6" x2="6" y2="18"></line>
      <line x1="6" y1="6" x2="18" y2="18"></line>
    </svg>
  </button>
</div>
