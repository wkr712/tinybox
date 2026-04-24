<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  let eyeOpen = $state(true);
  let tailWag = $state(false);
  let bounce = $state(false);
  let purring = $state(false);
  let mouseX = $state(0);
  let mouseY = $state(0);
  let blinkTimer: ReturnType<typeof setTimeout> | null = null;
  let wagTimer: ReturnType<typeof setTimeout> | null = null;
  let purrTimer: ReturnType<typeof setTimeout> | null = null;

  onMount(() => {
    scheduleBlink();
    scheduleWag();
  });

  onDestroy(() => {
    if (blinkTimer) clearTimeout(blinkTimer);
    if (wagTimer) clearTimeout(wagTimer);
    if (purrTimer) clearTimeout(purrTimer);
  });

  function scheduleBlink() {
    const delay = 2000 + Math.random() * 4000;
    blinkTimer = setTimeout(() => {
      eyeOpen = false;
      setTimeout(() => {
        eyeOpen = true;
        scheduleBlink();
      }, 150);
    }, delay);
  }

  function scheduleWag() {
    const delay = 3000 + Math.random() * 5000;
    wagTimer = setTimeout(() => {
      tailWag = true;
      setTimeout(() => {
        tailWag = false;
        scheduleWag();
      }, 800);
    }, delay);
  }

  function handleClick() {
    bounce = true;
    purring = true;
    setTimeout(() => { bounce = false; }, 400);
    if (purrTimer) clearTimeout(purrTimer);
    purrTimer = setTimeout(() => { purring = false; }, 2000);
  }

  function handleMousemove(e: MouseEvent) {
    const rect = (e.currentTarget as SVGElement).getBoundingClientRect();
    const cx = rect.left + rect.width / 2;
    const cy = rect.top + rect.height / 2;
    const dx = (e.clientX - cx) / rect.width;
    const dy = (e.clientY - cy) / rect.height;
    mouseX = Math.max(-2, Math.min(2, dx * 5));
    mouseY = Math.max(-1.5, Math.min(1.5, dy * 5));
  }

  function handleMouseleave() {
    mouseX = 0;
    mouseY = 0;
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<svg
  viewBox="0 0 100 100"
  xmlns="http://www.w3.org/2000/svg"
  class="cat-mascot {bounce ? 'cat-bounce' : ''} {purring ? 'cat-purr' : ''}"
  onclick={handleClick}
  onmousemove={handleMousemove}
  onmouseleave={handleMouseleave}
>
  <!-- Tail -->
  <path
    class="cat-tail {tailWag ? 'tail-wag' : ''}"
    d="M72 72 Q88 62 82 48 Q78 40 84 32"
    fill="none"
    stroke="currentColor"
    stroke-width="3.5"
    stroke-linecap="round"
  />

  <!-- Body -->
  <ellipse cx="50" cy="72" rx="22" ry="16" fill="currentColor" opacity="0.15" />

  <!-- Head -->
  <circle cx="50" cy="44" r="22" fill="currentColor" opacity="0.12" />

  <!-- Left ear -->
  <path d="M32 30 L28 10 L42 26" fill="currentColor" opacity="0.15" />
  <path d="M33 28 L30 14 L40 26" fill="currentColor" opacity="0.25" />

  <!-- Right ear -->
  <path d="M68 30 L72 10 L58 26" fill="currentColor" opacity="0.15" />
  <path d="M67 28 L70 14 L60 26" fill="currentColor" opacity="0.25" />

  <!-- Eyes -->
  {#if eyeOpen}
    <!-- Left eye -->
    <ellipse cx={42 + mouseX * 0.3} cy={42 + mouseY * 0.3} rx="3.5" ry="4" fill="currentColor" opacity="0.8" />
    <!-- Left eye highlight -->
    <circle cx={43.5 + mouseX * 0.3} cy={40.5 + mouseY * 0.3} r="1.2" fill="white" opacity="0.9" />
    <!-- Right eye -->
    <ellipse cx={58 + mouseX * 0.3} cy={42 + mouseY * 0.3} rx="3.5" ry="4" fill="currentColor" opacity="0.8" />
    <!-- Right eye highlight -->
    <circle cx={59.5 + mouseX * 0.3} cy={40.5 + mouseY * 0.3} r="1.2" fill="white" opacity="0.9" />
  {:else}
    <!-- Closed eyes (cute lines) -->
    <path d="M38 42 Q42 44 46 42" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" opacity="0.6" />
    <path d="M54 42 Q58 44 62 42" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" opacity="0.6" />
  {/if}

  <!-- Nose -->
  <ellipse cx="50" cy="49" rx="2" ry="1.5" fill="var(--color-accent-primary)" opacity="0.6" />

  <!-- Mouth -->
  <path d="M47 51 Q50 54 53 51" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" opacity="0.35" />

  <!-- Whiskers -->
  <line x1="30" y1="46" x2="40" y2="48" stroke="currentColor" stroke-width="0.8" opacity="0.2" />
  <line x1="30" y1="50" x2="40" y2="50" stroke="currentColor" stroke-width="0.8" opacity="0.2" />
  <line x1="60" y1="48" x2="70" y2="46" stroke="currentColor" stroke-width="0.8" opacity="0.2" />
  <line x1="60" y1="50" x2="70" y2="50" stroke="currentColor" stroke-width="0.8" opacity="0.2" />

  <!-- Paws -->
  <ellipse cx="38" cy="84" rx="6" ry="4" fill="currentColor" opacity="0.1" />
  <ellipse cx="62" cy="84" rx="6" ry="4" fill="currentColor" opacity="0.1" />

  <!-- Purr effect (shown when clicked) -->
  {#if purring}
    <text x="76" y="36" font-size="8" fill="var(--color-accent-primary)" opacity="0.5" class="purr-text">♪</text>
    <text x="20" y="32" font-size="6" fill="var(--color-accent-secondary)" opacity="0.4" class="purr-text-delay">♫</text>
  {/if}
</svg>

<style>
  .cat-mascot {
    color: var(--color-accent-primary);
    width: 100%;
    height: 100%;
    cursor: pointer;
    transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    filter: drop-shadow(0 0 8px color-mix(in srgb, var(--color-accent-primary) 15%, transparent));
  }

  .cat-mascot:hover {
    filter: drop-shadow(0 0 12px color-mix(in srgb, var(--color-accent-primary) 25%, transparent));
  }

  .cat-bounce {
    animation: cat-bounce 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .cat-purr .purr-text {
    animation: purr-float 1.5s ease-out infinite;
  }

  .cat-purr .purr-text-delay {
    animation: purr-float 1.5s ease-out 0.3s infinite;
  }

  .cat-tail {
    transition: d 0.3s ease;
  }

  .tail-wag {
    animation: tail-wag 0.4s ease-in-out infinite alternate;
  }

  @keyframes cat-bounce {
    0% { transform: scale(1); }
    30% { transform: scale(1.15); }
    50% { transform: scale(0.92); }
    70% { transform: scale(1.05); }
    100% { transform: scale(1); }
  }

  @keyframes tail-wag {
    0% { d: path("M72 72 Q88 62 82 48 Q78 40 84 32"); }
    100% { d: path("M72 72 Q90 66 86 52 Q83 42 90 35"); }
  }

  @keyframes purr-float {
    0% { opacity: 0.6; transform: translateY(0); }
    100% { opacity: 0; transform: translateY(-12px); }
  }
</style>
