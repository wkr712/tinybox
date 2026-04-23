<script lang="ts">
  let { width = "100%", height = "12px", rounded = "6px", style = "" }: {
    width?: string;
    height?: string;
    rounded?: string;
    style?: string;
  } = $props();
</script>

<div
  class="skeleton-line"
  style="width: {width}; height: {height}; border-radius: {rounded}; {style}"
></div>

<style>
  .skeleton-line {
    background: linear-gradient(
      90deg,
      rgba(255, 255, 255, 0.04) 25%,
      rgba(255, 255, 255, 0.08) 50%,
      rgba(255, 255, 255, 0.04) 75%
    );
    background-size: 200% 100%;
    animation: shimmer 1.5s ease-in-out infinite;
  }

  @keyframes shimmer {
    0% { background-position: 200% 0; }
    100% { background-position: -200% 0; }
  }
</style>
