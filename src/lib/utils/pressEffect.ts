export function pressEffect(
  node: HTMLElement,
  { scale = 0.95, duration = 150 }: { scale?: number; duration?: number } = {}
): { destroy: () => void } {
  const spring = `cubic-bezier(0.34, 1.56, 0.64, 1)`;

  function onDown() {
    node.style.transform = `scale(${scale})`;
    node.style.transition = `transform ${duration}ms ${spring}`;
  }

  function onUp() {
    node.style.transform = "scale(1)";
  }

  node.addEventListener("pointerdown", onDown);
  node.addEventListener("pointerup", onUp);
  node.addEventListener("pointerleave", onUp);

  return {
    destroy() {
      node.removeEventListener("pointerdown", onDown);
      node.removeEventListener("pointerup", onUp);
      node.removeEventListener("pointerleave", onUp);
    },
  };
}
