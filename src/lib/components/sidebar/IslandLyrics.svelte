<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { lyrics, playProgress } from "../../stores/music";

  let lrc = $state<any[]>([]);
  let progress = $state(0);
  let unsubs: (() => void)[] = [];
  let container: HTMLDivElement | undefined = $state();

  let activeIdx = $derived.by(() => {
    for (let i = lrc.length - 1; i >= 0; i--) {
      if (progress >= lrc[i].time) return i;
    }
    return -1;
  });

  let lastScrolledIdx = $state(-1);
  $effect(() => {
    if (activeIdx >= 0 && activeIdx !== lastScrolledIdx && container) {
      lastScrolledIdx = activeIdx;
      const el = container.querySelector(`[data-idx="${activeIdx}"]`);
      if (el) el.scrollIntoView({ behavior: "smooth", block: "center" });
    }
  });

  onMount(() => {
    unsubs.push(lyrics.subscribe((v) => (lrc = v)));
    unsubs.push(playProgress.subscribe((v) => (progress = v)));
  });

  onDestroy(() => {
    unsubs.forEach((u) => u());
    unsubs = [];
  });
</script>

{#if lrc.length > 0}
  <div
    bind:this={container}
    class="island-lyrics"
    style="mask-image: linear-gradient(transparent 0%, black 30%, black 70%, transparent 100%);"
  >
    {#each lrc as line, i (line.time)}
      <div
        data-idx={i}
        class="island-lyric-line {i === activeIdx ? 'active' : ''}"
      >
        {line.text}
      </div>
    {/each}
  </div>
{/if}

<style>
  .island-lyrics {
    flex: 1;
    overflow-y: auto;
    padding: 0 10px;
    scroll-behavior: smooth;
  }

  .island-lyric-line {
    font-size: 9px;
    color: rgba(255, 255, 255, 0.25);
    text-align: center;
    padding: 1px 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    transition: color 0.3s ease;
  }

  .island-lyric-line.active {
    color: var(--color-accent-primary, #00e5ff);
    font-weight: 500;
  }
</style>
