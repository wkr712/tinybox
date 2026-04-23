import type { TransitionConfig } from "svelte/transition";

const decelerate = "cubic-bezier(0.16, 1, 0.3, 1)";

export function springFly(
  node: Element,
  { x = 0, duration = 280 }: { x?: number; duration?: number } = {}
): TransitionConfig {
  const style = getComputedStyle(node);
  const opacity = +style.opacity;
  return {
    duration,
    css: (t) =>
      `opacity: ${t * opacity}; transform: translateX(${(1 - t) * x}px); transition-timing-function: ${decelerate};`,
  };
}
