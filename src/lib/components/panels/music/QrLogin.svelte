<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { generateQr, checkQr, fetchLoginStatus, fetchUserPlaylists, currentView, user, activeProvider } from "../../../stores/music";
  import { get } from "svelte/store";
  import type { MusicProviderKind } from "../../../types/music";

  let qrUrl = $state("");
  let qrKey = $state("");
  let status = $state<"loading" | "waiting" | "scanned" | "expired" | "error">("loading");
  let pollTimer: ReturnType<typeof setInterval> | null = null;
  let expiryTimer: ReturnType<typeof setTimeout> | null = null;
  let provider = $state<MusicProviderKind>("ncm");

  const providerLabels: Record<MusicProviderKind, { name: string; hint: string }> = {
    ncm: { name: "网易云音乐", hint: "打开网易云音乐APP扫码登录" },
    qqmusic: { name: "QQ音乐", hint: "打开QQ音乐APP扫码登录" },
    kugou: { name: "酷狗音乐", hint: "打开酷狗音乐APP扫码登录" },
  };

  onMount(() => {
    provider = get(activeProvider);
    startQr();
  });

  onDestroy(() => {
    if (pollTimer) clearInterval(pollTimer);
    if (expiryTimer) clearTimeout(expiryTimer);
  });

  async function startQr() {
    status = "loading";
    try {
      const result = await generateQr();
      qrKey = result.key;
      qrUrl = result.qrurl;
      status = "waiting";
      startPolling();
    } catch (e) {
      status = "error";
    }
  }

  function startPolling() {
    if (pollTimer) clearInterval(pollTimer);
    if (expiryTimer) clearTimeout(expiryTimer);
    let expired = false;
    expiryTimer = setTimeout(() => {
      expired = true;
      status = "expired";
      if (pollTimer) { clearInterval(pollTimer); pollTimer = null; }
    }, 300000);

    pollTimer = setInterval(async () => {
      if (expired) return;
      try {
        const code = await checkQr(qrKey);
        if (code === 800) {
          status = "expired";
          clearInterval(pollTimer!);
          clearTimeout(expiryTimer!);
        } else if (code === 802) {
          status = "scanned";
        } else if (code === 803) {
          clearInterval(pollTimer!);
          clearTimeout(expiryTimer!);
          const ok = await fetchLoginStatus();
          if (ok) {
            const u = get(user);
            if (u) await fetchUserPlaylists(u.user_id);
            currentView.set("discover");
          }
        }
      } catch {
        // ignore
      }
    }, 2000);
  }
</script>

<div class="flex flex-col items-center gap-4 py-6">
  <div class="text-white/40 text-xs mb-2">扫码登录{providerLabels[provider].name}</div>

  {#if status === "loading"}
    <div class="w-40 h-40 bg-white/5 rounded-xl flex items-center justify-center">
      <div class="text-white/20 text-sm">加载中...</div>
    </div>
  {:else if status === "error"}
    <div class="w-40 h-40 bg-white/5 rounded-xl flex items-center justify-center">
      <div class="text-red-400/60 text-sm">加载失败</div>
    </div>
    <button onclick={startQr} class="text-accent-primary text-xs hover:underline">重试</button>
  {:else}
    <div class="w-40 h-40 bg-white rounded-xl p-2 relative">
      <img src="https://api.qrserver.com/v1/create-qr-code/?size=300x300&data={encodeURIComponent(qrUrl)}" alt="QR" class="w-full h-full" />
      {#if status === "scanned"}
        <div class="absolute inset-0 bg-black/50 rounded-xl flex items-center justify-center">
          <span class="text-accent-primary text-sm">已扫码，确认中...</span>
        </div>
      {/if}
    </div>
  {/if}

  {#if status === "waiting"}
    <div class="text-white/30 text-[10px]">{providerLabels[provider].hint}</div>
  {:else if status === "expired"}
    <div class="text-white/30 text-[10px]">二维码已过期</div>
    <button onclick={startQr} class="text-accent-primary text-xs hover:underline">刷新二维码</button>
  {:else if status === "scanned"}
    <div class="text-accent-primary text-[10px]">请在手机上确认登录</div>
  {/if}
</div>
