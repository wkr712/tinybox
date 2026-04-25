<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    currentSong, isPlaying, lyrics, playProgress, pauseMusic, resumeMusic,
    tracks, playSong, formatDuration,
  } from "../../stores/music";
  import { restoreFromPlayer } from "../../stores/app";
  import { getCurrentWindow, currentMonitor } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";

  interface SpecData {
    bands: number[]; bass: number; mid: number; treble: number;
    energy: number; beat: boolean; waveform: number[];
  }
  interface Pt { x: number; y: number; vx: number; vy: number; life: number; size: number; }

  // ── Color System (desaturated ~15% for commercial quality) ──
  const SC: Record<string, { p: number[]; s: number[]; pt: number[] }> = {
    cyan:  { p:[72,168,192],  s:[58,105,172],  pt:[75,142,185] },
    blue:  { p:[75,132,192],  s:[48,88,168],   pt:[65,115,178] },
    teal:  { p:[58,175,168],  s:[48,108,162],  pt:[62,150,168] },
    ice:   { p:[98,168,205],  s:[65,98,178],   pt:[82,142,192] },
  };

  // ── Reactive State ──
  let song = $state<any>(null);
  let playing = $state(false);
  let lrc = $state<any[]>([]);
  let progress = $state(0);
  let trackList = $state<any[]>([]);
  let unsubs: (() => void)[] = [];
  let windowHeight = $state(80);
  let windowWidth = $state(320);
  let currentLyric = $state("");
  let lastIdx = -1;
  type VizMode = "combined" | "spectrum" | "waveform";
  let vizMode: VizMode = $state("combined");
  let sensitivity = $state(1.0);
  let colorScheme = $state("cyan");
  let ctxMenu: { x: number; y: number } | null = $state(null);

  // ── Canvas / Animation State ──
  let canvas: HTMLCanvasElement;
  let rootEl: HTMLDivElement;
  let animFrame = 0;
  let mounted = false;
  const NB = 32, NW = 128;
  let tB = new Array(NB).fill(0);
  let tBa = 0, tMi = 0, tTr = 0, tEn = 0, tBeat = false;
  let tWv = new Array(NW).fill(0);
  let sB = new Array(NB).fill(0);
  let bVel = new Array(NB).fill(0);
  let sBa = 0, sMi = 0, sTr = 0, sEn = 0;
  let sWv = new Array(NW).fill(0);
  let bFlash = 0;
  let breathVal = 0;
  const barJitter = Array.from({ length: NB }, () => 0.92 + Math.random() * 0.16);
  let pts: Pt[] = [];

  // ── Expansion Animation ──
  const EXPAND_MS = 380;
  let expanding = false;
  let expandStart = 0;
  let expandP = 0;
  let expandEased = 0;
  let smoothH = 80;
  let smoothW = 320;

  // ── Unified Animation System ──
  function easeOutCubic(t: number): number {
    return 1 - Math.pow(1 - t, 3);
  }
  function easeOutBack(t: number): number {
    const s = 1.2;
    return 1 + (s + 1) * Math.pow(t - 1, 3) + s * Math.pow(t - 1, 2);
  }
  // Staggered layer delays (fraction of total duration)
  const LAYER = { container: 0, structure: 0.14, content: 0.28, detail: 0.43 };
  function staggered(raw: number, layer: number): number {
    const adj = Math.max(0, raw - layer) / Math.max(0.01, 1 - layer);
    return easeOutCubic(Math.min(1, adj));
  }

  // Song transition state
  let songFade = 1;
  let prevSongId: string | null = null;
  let songTransitionStart = 0;
  const SONG_TRANSITION_MS = 400;
  let songTransitioning = false;

  // Smooth play/pause state (0–1, canvas-driven)
  let playSmooth = 0;

  // ── Derived ──
  let activeLyricIdx = $derived.by(() => {
    for (let i = lrc.length - 1; i >= 0; i--) {
      if (progress >= lrc[i].time) return i;
    }
    return -1;
  });
  $effect(() => {
    if (activeLyricIdx >= 0 && activeLyricIdx !== lastIdx) {
      lastIdx = activeLyricIdx;
      currentLyric = lrc[activeLyricIdx]?.text || "";
    }
  });
  // Trigger song crossfade on song change
  $effect(() => {
    if (song && song.id !== prevSongId) {
      if (prevSongId !== null) {
        songFade = 0;
        songTransitionStart = performance.now();
        songTransitioning = true;
      }
      prevSongId = song?.id ?? null;
    }
  });

  let progressPct = $derived(
    song && song.duration > 0 ? Math.min((progress * 1000) / song.duration * 100, 100) : 0
  );
  let currentIdx = $derived(song ? trackList.findIndex((t) => t.id === song.id) : -1);
  let coverSize = $derived(Math.max(34, Math.min(72, Math.round(windowHeight * 0.42))));
  let showArtist = $derived(windowHeight >= 150);
  let showTime = $derived(windowHeight >= 150);
  let lyricsCount = $derived(
    windowHeight < 130 ? 0 : Math.min(14, Math.max(1, Math.floor((windowHeight - 130) / 22)))
  );
  let visibleLyrics = $derived.by(() => {
    if (lyricsCount <= 0) return [];
    const half = Math.floor(lyricsCount / 2);
    const start = Math.max(0, activeLyricIdx - half);
    const end = Math.min(lrc.length, start + lyricsCount);
    return lrc.slice(Math.max(0, end - lyricsCount), end);
  });
  let hasLyrics = $derived(lyricsCount > 0);
  let showHint = $derived(windowHeight > 330 && windowWidth > 260);
  let fmtCur = $derived(formatDuration(progress * 1000));
  let fmtDur = $derived(song?.duration ? formatDuration(song.duration) : "0:00");
  let vp = $derived(`rgb(${(SC[colorScheme] ?? SC.cyan).p.join(",")})`);
  let vpg = $derived(`rgba(${(SC[colorScheme] ?? SC.cyan).p.join(",")},0.08)`);
  function rgba(c: number[], a: number) { return `rgba(${c[0]},${c[1]},${c[2]},${a})`; }

  /* ══════════════════════════════════════════════════
     Render Pipeline
     ══════════════════════════════════════════════════ */

  function render() {
    if (!mounted || !canvas) return;
    const parent = canvas.parentElement;
    if (!parent) { animFrame = requestAnimationFrame(render); return; }
    const dpr = window.devicePixelRatio || 1;
    const rect = parent.getBoundingClientRect();
    const cw = Math.round(rect.width * dpr);
    const ch = Math.round(rect.height * dpr);
    if (canvas.width !== cw || canvas.height !== ch) { canvas.width = cw; canvas.height = ch; }
    const c = canvas.getContext("2d");
    if (!c) { animFrame = requestAnimationFrame(render); return; }
    c.setTransform(dpr, 0, 0, dpr, 0, 0);
    const w = rect.width, h = rect.height;
    const sch = SC[colorScheme] ?? SC.cyan;
    const S = sensitivity;
    const now = performance.now();

    // ── Smooth size interpolation ──
    smoothH += (h - smoothH) * 0.1;
    smoothW += (w - smoothW) * 0.1;

    // ── Song transition progress (staggered) ──
    if (songTransitioning) {
      const rawP = Math.min(1, (now - songTransitionStart) / SONG_TRANSITION_MS);
      songFade = staggered(rawP, LAYER.content);
      if (rawP >= 1) songTransitioning = false;
    }

    // ── Smooth play/pause transition ──
    const playTarget = playing ? 1 : 0;
    playSmooth += (playTarget - playSmooth) * 0.06;

    // ── Expansion animation ──
    if (expanding) {
      const elapsed = now - expandStart;
      expandP = Math.min(elapsed / EXPAND_MS, 1);
      expandEased = easeOutBack(expandP);
      if (rootEl) rootEl.style.setProperty("--ex", String(expandEased));
      breathVal *= (1 - expandP * 0.6);
      if (expandP >= 1) {
        expanding = false;
        expandP = 0;
        expandEased = 0;
        if (rootEl) rootEl.style.removeProperty("--ex");
        restoreFromPlayer();
        return;
      }
    }

    // ── Frequency-dependent spring physics ──
    for (let i = 0; i < NB; i++) {
      const target = tB[i] * S;
      let stiff, damp;
      if (i < 8)       { stiff = 0.08; damp = 0.88; }   // bass: slow, smooth breathing
      else if (i < 20)  { stiff = 0.14; damp = 0.84; }   // mid: moderate response
      else              { stiff = 0.22; damp = 0.80; }   // treble: fast, detailed
      bVel[i] += (target - sB[i]) * stiff;
      bVel[i] *= damp;
      sB[i] += bVel[i];
      if (sB[i] < 0) { sB[i] = 0; bVel[i] = 0; }
    }
    for (let i = 0; i < NW; i++) sWv[i] += (tWv[i] - sWv[i]) * 0.07;
    sBa += (tBa * S - sBa) * 0.05;
    sMi += (tMi * S - sMi) * 0.07;
    sTr += (tTr * S - sTr) * 0.09;
    sEn += (tEn * S - sEn) * 0.07;
    breathVal += (sBa - breathVal) * 0.025;

    if (tBeat) bFlash = 1;
    bFlash *= 0.88;

    // ── Push CSS custom properties for HTML transitions ──
    if (rootEl) {
      const energy = sB.reduce((a, b) => a + b, 0) / NB;
      rootEl.style.setProperty("--energy", String(energy));
      rootEl.style.setProperty("--breath", String(1 + breathVal * 0.01));
      rootEl.style.setProperty("--hr", String(Math.min(smoothH / 200, 1)));
      rootEl.style.setProperty("--sf", String(songFade));
      rootEl.style.setProperty("--ps", String(playSmooth));
    }

    updatePts(w, h);

    // ═══ DRAW ═══
    c.clearRect(0, 0, w, h);
    const expandBarMul = expanding ? (0.6 + expandEased * 0.4) : 1;
    const expandWaveMul = expanding ? expandEased : 1;
    const vizIntensity = 0.3 + playSmooth * 0.7;
    const expandBgBoost = expanding ? expandEased * 0.01 : 0;

    drawBg(c, w, h, sch, vizIntensity, expandBgBoost);
    if (vizMode === "combined" || vizMode === "waveform")
      drawWave(c, w, h, sch, expandWaveMul * vizIntensity);
    if (vizMode === "combined" || vizMode === "spectrum")
      drawBars(c, w, h, sch, expandBarMul * vizIntensity);
    drawPtFx(c, sch, vizIntensity);

    if (bFlash > 0.01) {
      c.fillStyle = rgba(sch.p, bFlash * 0.005);
      c.fillRect(0, 0, w, h);
    }

    // Vignette (soft)
    const vg = c.createRadialGradient(w / 2, h / 2, Math.min(w, h) * 0.25, w / 2, h / 2, Math.max(w, h) * 0.85);
    vg.addColorStop(0, "rgba(0,0,0,0)");
    vg.addColorStop(1, "rgba(0,0,0,0.28)");
    c.fillStyle = vg;
    c.fillRect(0, 0, w, h);

    animFrame = requestAnimationFrame(render);
  }

  function drawBg(c: CanvasRenderingContext2D, w: number, h: number, sch: typeof SC.cyan, intensity: number, boost: number) {
    // Bass-driven ambient glow (reduced)
    const r = 18 + breathVal * 22;
    const g = c.createRadialGradient(w * 0.22, h * 0.5, 0, w * 0.22, h * 0.5, r);
    g.addColorStop(0, rgba(sch.s, (0.005 + breathVal * 0.01 + boost) * intensity));
    g.addColorStop(1, "rgba(0,0,0,0)");
    c.fillStyle = g;
    c.fillRect(0, 0, w, h);

    // Treble accent (reduced)
    const r2 = 12 + sTr * 14;
    const g2 = c.createRadialGradient(w * 0.82, h * 0.45, 0, w * 0.82, h * 0.45, r2);
    g2.addColorStop(0, rgba(sch.p, (0.003 + sTr * 0.005) * intensity));
    g2.addColorStop(1, "rgba(0,0,0,0)");
    c.fillStyle = g2;
    c.fillRect(0, 0, w, h);
  }

  function drawBars(c: CanvasRenderingContext2D, w: number, h: number, sch: typeof SC.cyan, heightMul: number) {
    const gap = 2;
    const bw = Math.max(1, (w - gap * (NB - 1)) / NB);
    const maxH = h * 0.26 * heightMul;

    let mx = 0;
    for (let i = 0; i < NB; i++) mx = Math.max(mx, sB[i]);
    if (mx < 0.001) mx = 1;

    const barGrad = c.createLinearGradient(0, h, 0, h - maxH);
    barGrad.addColorStop(0, rgba(sch.s, 0.18));
    barGrad.addColorStop(0.4, rgba(sch.p, 0.10));
    barGrad.addColorStop(1, rgba(sch.p, 0.015));

    for (let i = 0; i < NB; i++) {
      const val = (sB[i] / mx) * barJitter[i];
      const bh = val * maxH + 0.2;
      const x = i * (bw + gap);
      const y = h - bh;
      const rd = Math.min(bw / 2, 3);

      c.fillStyle = barGrad;
      c.beginPath();
      c.moveTo(x, h);
      c.lineTo(x, y + rd);
      c.arcTo(x, y, x + rd, y, rd);
      c.arcTo(x + bw, y, x + bw, y + rd, rd);
      c.lineTo(x + bw, h);
      c.closePath();
      c.fill();

      // Subtle top glow
      if (val > 0.2) {
        const capR = bw * 0.5;
        const cg = c.createRadialGradient(x + bw / 2, y, 0, x + bw / 2, y, capR);
        cg.addColorStop(0, rgba(sch.p, val * 0.06));
        cg.addColorStop(1, "rgba(0,0,0,0)");
        c.fillStyle = cg;
        c.beginPath();
        c.arc(x + bw / 2, y, capR, 0, Math.PI * 2);
        c.fill();
      }
    }
  }

  function drawWave(c: CanvasRenderingContext2D, w: number, h: number, sch: typeof SC.cyan, opacityMul: number) {
    const midY = h * 0.48;
    const amp = h * 0.08;

    let mx = 0;
    for (let i = 0; i < NW; i++) mx = Math.max(mx, Math.abs(sWv[i]));
    if (mx < 0.001) mx = 1;

    const step = 4;
    const pts2: { x: number; y: number }[] = [];
    for (let i = 0; i < NW; i += step) {
      let sum = 0, cnt = 0;
      for (let j = i; j < Math.min(i + step, NW); j++) { sum += sWv[j]; cnt++; }
      pts2.push({ x: (i / (NW - 1)) * w, y: midY + (sum / cnt / mx) * amp });
    }

    // Catmull-Rom spline
    c.beginPath();
    c.moveTo(pts2[0].x, pts2[0].y);
    for (let i = 0; i < pts2.length - 1; i++) {
      const p0 = pts2[Math.max(0, i - 1)];
      const p1 = pts2[i];
      const p2 = pts2[Math.min(pts2.length - 1, i + 1)];
      const p3 = pts2[Math.min(pts2.length - 1, i + 2)];
      const cp1x = p1.x + (p2.x - p0.x) / 6;
      const cp1y = p1.y + (p2.y - p0.y) / 6;
      const cp2x = p2.x - (p3.x - p1.x) / 6;
      const cp2y = p2.y - (p3.y - p1.y) / 6;
      c.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, p2.x, p2.y);
    }

    c.strokeStyle = rgba(sch.p, (0.04 + sMi * 0.02) * opacityMul);
    c.lineWidth = 1;
    c.lineCap = "round";
    c.stroke();

    c.lineTo(w, midY);
    c.lineTo(0, midY);
    c.closePath();
    c.fillStyle = rgba(sch.p, (0.004 + sMi * 0.002) * opacityMul);
    c.fill();
  }

  function updatePts(w: number, h: number) {
    // Reduced particles (70% fewer) — edge only
    if (pts.length < 2 && sTr > 0.1 && Math.random() < sTr * 0.06) {
      const onLeft = Math.random() < 0.5;
      pts.push({
        x: onLeft ? Math.random() * w * 0.06 : w * 0.94 + Math.random() * w * 0.06,
        y: h * 0.2 + Math.random() * h * 0.6,
        vx: (Math.random() - 0.5) * 0.06,
        vy: -(0.02 + Math.random() * 0.08),
        life: 1,
        size: 0.2 + Math.random() * 0.6,
      });
    }
    for (let i = pts.length - 1; i >= 0; i--) {
      const p = pts[i];
      p.x += p.vx; p.y += p.vy; p.life -= 0.006;
      if (p.life <= 0 || p.y < -5) pts.splice(i, 1);
    }
  }

  function drawPtFx(c: CanvasRenderingContext2D, sch: typeof SC.cyan, intensity: number) {
    for (const p of pts) {
      const a = p.life;
      c.beginPath();
      c.arc(p.x, p.y, p.size * 1.5, 0, Math.PI * 2);
      c.fillStyle = rgba(sch.pt, a * 0.015 * intensity);
      c.fill();
      c.beginPath();
      c.arc(p.x, p.y, p.size * 0.5, 0, Math.PI * 2);
      c.fillStyle = rgba(sch.pt, a * 0.08 * intensity);
      c.fill();
    }
  }

  /* ── Lifecycle ── */
  onMount(() => {
    unsubs.push(currentSong.subscribe((v) => (song = v)));
    unsubs.push(isPlaying.subscribe((v) => (playing = v)));
    unsubs.push(lyrics.subscribe((v) => (lrc = v)));
    unsubs.push(playProgress.subscribe((v) => (progress = v)));
    unsubs.push(tracks.subscribe((v) => (trackList = v)));

    const win = getCurrentWindow();
    const updateSize = async () => {
      try {
        const size = await win.innerSize();
        const mon = await currentMonitor();
        const s = mon?.scaleFactor ?? 1;
        windowWidth = size.width / s;
        windowHeight = size.height / s;
      } catch {}
    };
    updateSize();
    win.onResized(() => updateSize()).then((unlisten) => {
      unsubs.push(() => unlisten());
    });

    const specId = setInterval(async () => {
      // Skip IPC when not playing — saves CPU and avoids unnecessary bridge traffic
      if (!playing) return;
      try {
        const d = await invoke<SpecData>("music_spectrum");
        for (let i = 0; i < NB && i < d.bands.length; i++) tB[i] = d.bands[i];
        for (let i = 0; i < NW && i < d.waveform.length; i++) tWv[i] = d.waveform[i];
        tBa = d.bass; tMi = d.mid; tTr = d.treble;
        tEn = d.energy; tBeat = d.beat;
      } catch {}
    }, 60);
    unsubs.push(() => clearInterval(specId));

    mounted = true;
    requestAnimationFrame(() => { animFrame = requestAnimationFrame(render); });
  });

  onDestroy(() => {
    mounted = false;
    if (animFrame) cancelAnimationFrame(animFrame);
    unsubs.forEach((u) => u());
    unsubs = [];
  });

  /* ── Actions ── */
  async function togglePlay(e: MouseEvent) {
    e.stopPropagation();
    if (playing) await pauseMusic(); else await resumeMusic();
  }
  async function prevTrack(e: MouseEvent) {
    e.stopPropagation();
    if (currentIdx > 0) await playSong(trackList[currentIdx - 1]);
  }
  async function nextTrack(e: MouseEvent) {
    e.stopPropagation();
    if (currentIdx < trackList.length - 1) await playSong(trackList[currentIdx + 1]);
  }
  function startExpand() {
    if (expanding) return;
    expanding = true;
    expandStart = performance.now();
    expandP = 0;
    expandEased = 0;
  }
  function handleCtx(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    if (!rootEl) return;
    const rect = rootEl.getBoundingClientRect();
    ctxMenu = {
      x: Math.min(e.clientX - rect.left, rect.width - 100),
      y: Math.min(e.clientY - rect.top, rect.height - 200),
    };
  }
  function closeCtx() { ctxMenu = null; }
  function setMode(m: VizMode) { vizMode = m; ctxMenu = null; }
  function setSens(v: number) { sensitivity = v; ctxMenu = null; }
  function setColor(v: string) { colorScheme = v; ctxMenu = null; }
</script>

<svelte:window onclick={closeCtx} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="fp {playing ? 'playing' : ''}" bind:this={rootEl}
     data-tauri-drag-region
     style="--vp:{vp};--vpg:{vpg}">

  <canvas bind:this={canvas} class="viz"></canvas>

  {#if ctxMenu}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="ctx" style="left:{ctxMenu.x}px;top:{ctxMenu.y}px"
         onmousedown={(e) => e.stopPropagation()}>
      <div class="ctx-l">可视化</div>
      <button class="ctx-i" class:act={vizMode==="combined"} onclick={() => setMode("combined")}>综合</button>
      <button class="ctx-i" class:act={vizMode==="spectrum"} onclick={() => setMode("spectrum")}>频谱</button>
      <button class="ctx-i" class:act={vizMode==="waveform"} onclick={() => setMode("waveform")}>波形</button>
      <div class="ctx-d"></div>
      <div class="ctx-l">灵敏度</div>
      <button class="ctx-i" class:act={sensitivity===0.6} onclick={() => setSens(0.6)}>低</button>
      <button class="ctx-i" class:act={sensitivity===1.0} onclick={() => setSens(1.0)}>中</button>
      <button class="ctx-i" class:act={sensitivity===1.5} onclick={() => setSens(1.5)}>高</button>
      <div class="ctx-d"></div>
      <div class="ctx-l">配色</div>
      <button class="ctx-i" class:act={colorScheme==="cyan"} onclick={() => setColor("cyan")}>青蓝</button>
      <button class="ctx-i" class:act={colorScheme==="blue"} onclick={() => setColor("blue")}>深蓝</button>
      <button class="ctx-i" class:act={colorScheme==="teal"} onclick={() => setColor("teal")}>蓝绿</button>
      <button class="ctx-i" class:act={colorScheme==="ice"} onclick={() => setColor("ice")}>冰蓝</button>
    </div>
  {/if}

  <div class="cover-slot" data-tauri-drag-region>
    <div class="cover" style="--s:{coverSize}px">
      {#if song?.pic_url}
        <img src="{song.pic_url}?param=200y200" alt="" />
      {:else}
        <div class="cover-ph">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" opacity="0.2"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>
        </div>
      {/if}
    </div>
  </div>

  <div class="info-col">
    <div class="title">{song?.name || '未在播放'}</div>
    {#if showArtist}
      <div class="sub">{song?.artists || ''}</div>
    {/if}
    {#if !hasLyrics && currentLyric}
      <div class="mini-lrc">{currentLyric}</div>
    {/if}
  </div>

  <div class="btns">
    <button class="b" onclick={prevTrack} aria-label="上一首">
      <svg width="9" height="9" viewBox="0 0 24 24" fill="currentColor"><polygon points="19 20 9 12 19 4"/><rect x="5" y="4" width="2" height="16"/></svg>
    </button>
    <button class="b play" onclick={togglePlay} aria-label={playing ? '暂停' : '播放'}>
      {#if playing}
        <svg width="11" height="11" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
      {:else}
        <svg width="11" height="11" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"/></svg>
      {/if}
    </button>
    <button class="b" onclick={nextTrack} aria-label="下一首">
      <svg width="9" height="9" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 4 15 12 5 20"/><rect x="17" y="4" width="2" height="16"/></svg>
    </button>
  </div>

  <button class="expand-btn" onclick={startExpand} aria-label="展开面板">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <polyline points="15 3 21 3 21 9"></polyline>
      <polyline points="9 21 3 21 3 15"></polyline>
      <line x1="21" y1="3" x2="14" y2="10"></line>
      <line x1="3" y1="21" x2="10" y2="14"></line>
    </svg>
  </button>

  {#if hasLyrics}
    <div class="lrc-wrap">
      {#if visibleLyrics.length > 0}
        <div class="lrc-scroll">
          {#each visibleLyrics as line, idx (line.time)}
            <div class="l {lrc.indexOf(line) === activeLyricIdx ? 'on' : ''}" style="--li:{idx}">{line.text}</div>
          {/each}
        </div>
      {:else if song}
        <div class="lrc-scroll">
          {#if lyricsCount >= 3}
            <div class="l dim">暂无歌词</div>
          {:else}
            <div class="l focus">{currentLyric || '♪ ♪ ♪'}</div>
          {/if}
        </div>
      {/if}
    </div>
  {/if}

  {#if showTime}
    <div class="time" data-tauri-drag-region>{fmtCur} / {fmtDur}</div>
  {/if}
  {#if showHint}
    <div class="hint" data-tauri-drag-region>继续拉大可展开全功能面板</div>
  {/if}

  <div class="prog">
    <div class="prog-fill" style="width:{progressPct}%"></div>
  </div>
</div>

<style>
  .fp {
    display: grid;
    grid-template-columns: auto 1fr auto auto;
    grid-template-rows: auto 1fr auto auto;
    align-items: center;
    width: 100%; height: 100%;
    position: relative;
    user-select: none;
    overflow: hidden;
    padding: 6px 10px 8px;
    gap: 0;
    background: rgba(5, 5, 14, 0.96);
    backdrop-filter: blur(32px) saturate(1.15);
    -webkit-backdrop-filter: blur(32px) saturate(1.15);
    transform: scale(calc(1 + var(--ex, 0) * 0.02));
    box-shadow: 0 2px 12px rgba(0, 0, 0, calc(0.2 + var(--ex, 0) * 0.12));
    transition: box-shadow 0.3s ease;
    will-change: transform;
  }

  .viz {
    position: absolute;
    top: 0; left: 0;
    width: 100%; height: 100%;
    z-index: 0;
    pointer-events: none;
  }

  .ctx {
    position: absolute; z-index: 20;
    background: rgba(8, 8, 18, 0.97);
    backdrop-filter: blur(18px);
    -webkit-backdrop-filter: blur(18px);
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 10px;
    padding: 4px 0;
    min-width: 96px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  }
  .ctx-l { padding: 4px 12px 2px; color: rgba(255,255,255,0.22); font-size: 8px; text-transform: uppercase; letter-spacing: 0.06em; }
  .ctx-d { height: 1px; background: rgba(255,255,255,0.03); margin: 3px 8px; }
  .ctx-i { display: block; width: 100%; padding: 4px 12px; background: none; border: none; color: rgba(255,255,255,0.35); text-align: left; cursor: pointer; font-size: 10px; transition: all 0.15s; font-family: inherit; }
  .ctx-i:hover { background: rgba(255,255,255,0.03); color: rgba(255,255,255,0.65); }
  .ctx-i.act { color: var(--vp, #48a8c0); }

  /* Cover — smooth crossfade via --sf, breathing glow via --ps */
  .cover-slot {
    grid-column: 1; grid-row: 1;
    padding-right: 10px; z-index: 1;
    opacity: var(--sf, 1);
  }
  .cover {
    width: var(--s); height: var(--s);
    border-radius: 50%; overflow: hidden;
    box-shadow:
      0 2px 8px rgba(0,0,0,0.25),
      0 0 calc(2px + var(--energy, 0) * 8px * var(--ps, 0)) var(--vpg, rgba(60,168,192,0.08));
    transform: scale(calc(0.95 + var(--sf, 1) * 0.05 + var(--ex, 0) * 0.04));
    transition: box-shadow 0.3s ease, transform 0.35s cubic-bezier(0.22, 1, 0.36, 1);
  }
  .cover img { width: 100%; height: 100%; object-fit: cover; display: block; }
  .cover-ph { width: 100%; height: 100%; background: rgba(255,255,255,0.02); display: flex; align-items: center; justify-content: center; color: white; }

  /* Info — song crossfade via --sf */
  .info-col {
    grid-column: 2; grid-row: 1;
    display: flex; flex-direction: column; gap: 1px;
    min-width: 0; overflow: hidden; padding-right: 4px; z-index: 1;
    opacity: calc((0.75 + var(--ex, 0) * 0.25) * var(--sf, 1));
    transform: translateX(calc((1 - var(--sf, 1)) * 4px));
    transition: opacity 0.1s linear, transform 0.1s linear;
  }
  .title { font-size: 11px; font-weight: 500; color: rgba(255,255,255,0.85); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; line-height: 1.3; }
  .sub { font-size: 9px; color: rgba(255,255,255,0.28); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; line-height: 1.3; }
  .mini-lrc { font-size: 9px; color: var(--vp, #48a8c0); opacity: 0.45; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; line-height: 1.3; }

  .btns { grid-column: 3; grid-row: 1; display: flex; align-items: center; gap: 1px; flex-shrink: 0; z-index: 1; }
  .b {
    width: 24px; height: 24px; border-radius: 50%; border: none;
    background: transparent; color: rgba(255,255,255,0.28);
    display: flex; align-items: center; justify-content: center;
    cursor: pointer; transition: color 0.2s, background 0.2s;
    flex-shrink: 0;
  }
  .b:hover { color: rgba(255,255,255,0.65); background: rgba(255,255,255,0.03); }
  .b:active { transform: scale(0.9); }
  .b.play { width: 28px; height: 28px; color: var(--vp, #48a8c0); }
  .b.play:hover { background: rgba(255,255,255,0.04); }

  .expand-btn {
    grid-column: 4; grid-row: 1;
    width: 30px; height: 30px; border-radius: 8px;
    border: 1px solid rgba(255,255,255,0.05);
    background: rgba(255,255,255,0.02);
    color: rgba(255,255,255,0.22);
    display: flex; align-items: center; justify-content: center;
    cursor: pointer; transition: all 0.25s ease;
    margin-left: 4px; flex-shrink: 0; z-index: 1;
    opacity: calc(1 - var(--ex, 0) * 0.8);
    transform: scale(calc(1 + var(--ex, 0) * 0.08));
  }
  .expand-btn:hover { color: rgba(255,255,255,0.6); background: rgba(255,255,255,0.05); border-color: rgba(255,255,255,0.08); }

  /* Lyrics — staggered cascading transitions */
  .lrc-wrap {
    grid-column: 1 / -1; grid-row: 2;
    min-height: 0; overflow: hidden;
    padding: 4px 16px 0; z-index: 1;
    opacity: calc(0.6 + var(--ex, 0) * 0.4);
    transform: translateY(calc((1 - var(--ex, 0)) * 6px));
    transition: opacity 0.35s ease-out, transform 0.35s cubic-bezier(0.22, 1, 0.36, 1);
  }
  .lrc-scroll {
    display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 3px;
    mask-image: linear-gradient(to bottom, transparent 0%, white 12%, white 88%, transparent 100%);
    -webkit-mask-image: linear-gradient(to bottom, transparent 0%, white 12%, white 88%, transparent 100%);
  }
  .l {
    font-size: 12px; color: rgba(255,255,255,0.14); text-align: center; line-height: 1.5;
    white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 100%;
    transition: color 0.4s ease calc(var(--li, 0) * 25ms),
                font-size 0.35s ease calc(var(--li, 0) * 25ms),
                font-weight 0.3s ease calc(var(--li, 0) * 25ms);
  }
  .l.on { color: rgba(255,255,255,0.88); font-size: 13px; font-weight: 500; }
  .l.focus { color: rgba(255,255,255,0.6); font-size: 13px; }
  .l.dim { color: rgba(255,255,255,0.07); font-size: 10px; }

  .time { grid-column: 1 / -1; grid-row: 3; text-align: center; font-size: 9px; font-family: "SF Mono","Cascadia Code","Consolas",monospace; color: rgba(255,255,255,0.12); padding: 3px 0 0; letter-spacing: 0.03em; z-index: 1; }
  .hint { grid-column: 1 / -1; grid-row: 4; text-align: center; font-size: 9px; color: rgba(255,255,255,0.05); padding: 1px 0 0; z-index: 1; }

  .prog { position: absolute; bottom: 0; left: 14px; right: 14px; height: 2px; background: rgba(255,255,255,0.025); border-radius: 2px; overflow: hidden; z-index: 2; }
  .prog-fill { height: 100%; background: var(--vp, #48a8c0); border-radius: 2px; transition: width 0.4s linear; opacity: calc(0.3 + var(--ps, 0) * 0.2); }
</style>
