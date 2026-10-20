<script lang="ts">
  import confetti from "canvas-confetti";
  import { app } from "$lib/store.svelte";
  import { play } from "$lib/sound";
  import type { Completion } from "$lib/types";

  let shown = $state(0);
  let showLevel = $state(false);
  let toastC = $state<Completion | null>(null);
  let modalC = $state<Completion | null>(null);
  let raf = 0;
  let timer: ReturnType<typeof setTimeout> | undefined;

  function burst(big: boolean) {
    const base = { origin: { y: 0.65 }, colors: ["#f97316", "#fb923c", "#fcd34d", "#4ade80", "#f2ebe4"] };
    confetti({ ...base, particleCount: big ? 160 : 90, spread: big ? 100 : 70 });
    if (big) setTimeout(() => confetti({ ...base, particleCount: 100, spread: 120, startVelocity: 45 }), 250);
  }

  function countUp(to: number) {
    cancelAnimationFrame(raf);
    const t0 = performance.now();
    const step = (t: number) => {
      const k = Math.min(1, (t - t0) / 900);
      shown = Math.round(to * (1 - (1 - k) ** 3));
      if (k < 1) raf = requestAnimationFrame(step);
    };
    raf = requestAnimationFrame(step);
  }

  $effect(() => {
    const c = app.celebration;
    if (!c) return;
    app.celebration = null;
    const levelUp = c.level_after > c.level_before;
    burst(levelUp);
    toastC = c;
    countUp(c.xp_awarded);
    clearTimeout(timer);
    timer = setTimeout(() => (toastC = null), 9000);
    if (c.crit) {
      // just after the verdict chord, so the zap is heard on its own
      setTimeout(() => {
        play("crit");
        app.critFlash++;
      }, 250);
    }
    if (levelUp) {
      modalC = c;
      showLevel = true;
      setTimeout(() => play("level"), c.crit ? 650 : 350);
    }
    if (c.new_achievements.length) setTimeout(() => app.enqueueAchievements(c.new_achievements), levelUp ? 1400 : 700);
  });

  async function goNext() {
    toastC = null;
    closeModal();
    await app.goNext();
  }

  const nextHere = $derived(app.next?.quest && app.next.from === (toastC ?? modalC)?.quest_id ? app.next : null);

  function closeModal() {
    showLevel = false;
    modalC = null;
  }
</script>

<svelte:window onkeydown={(e) => { if (showLevel && (e.key === "Escape" || e.key === "Enter")) { e.stopPropagation(); closeModal(); } }} />

{#if toastC && !showLevel}
  <div class="pop fixed bottom-8 left-1/2 z-50 -translate-x-1/2 rounded-2xl border border-rust/50 bg-panel-2 px-8 py-4 text-center shadow-2xl" data-testid="xp-toast">
    {#if toastC.crit}<div class="crit mb-1 text-2xl font-black tracking-wider text-amber-300" data-testid="crit">CRIT! 2x</div>{/if}
    <div class="text-4xl font-extrabold text-rust-hi tabular-nums">+{shown} XP</div>
    <div class="text-sm text-dim">Quest cleared{toastC.clean ? " · clean compile" : ""}{toastC.focus_bonus ? " · focus bonus" : ""}{toastC.combo_mult > 1 ? ` · combo +${Math.round((toastC.combo_mult - 1) * 100)}%` : ""}</div>
    {#if nextHere}<button class="btn btn-primary mt-3 w-full" onclick={goNext} data-testid="toast-next">{nextHere.label}</button>{/if}
  </div>
{/if}

{#if showLevel && modalC}
  <div class="fixed inset-0 z-[60] flex items-center justify-center bg-black/70 p-6" role="dialog" aria-modal="true" aria-label="Level up">
    <div class="pop w-full max-w-md rounded-3xl border border-rust/60 bg-panel p-10 text-center shadow-2xl">
      <div class="text-sm font-semibold tracking-widest text-rust-hi uppercase">Level up</div>
      <div class="my-3 text-8xl font-black text-rust">{modalC.level_after}</div>
      {#if modalC.crit}<div class="crit mb-1 text-2xl font-black tracking-wider text-amber-300" data-testid="crit">CRIT! 2x</div>{/if}
      <div class="text-dim">+{modalC.xp_awarded} XP · level {modalC.level_before} → {modalC.level_after}</div>
      {#if nextHere}
        <button class="btn btn-primary btn-lg mt-8 w-full" onclick={goNext} data-testid="toast-next">{nextHere.label}</button>
        <button class="btn btn-ghost mt-2 w-full" onclick={closeModal}>Stay here</button>
      {:else}
        <button class="btn btn-primary btn-lg mt-8 w-full" onclick={closeModal}>Keep going</button>
      {/if}
    </div>
  </div>
{/if}

{#if app.achToast}
  {#key app.achToast.key}
    <div class="pop pointer-events-none fixed top-6 left-1/2 z-[70] -translate-x-1/2 rounded-2xl border border-amber-300/50 bg-panel-2 px-6 py-3 text-center shadow-2xl" role="status" data-testid="ach-toast">
      <div class="text-xs font-semibold tracking-widest text-amber-300 uppercase">Achievement</div>
      <div class="text-lg font-bold">✨ {app.achievementTitle(app.achToast.id)}</div>
    </div>
  {/key}
{/if}

<style>
  .crit { animation: critpop 0.5s ease-out; text-shadow: 0 0 18px rgb(252 211 77 / 0.6); }
  @keyframes critpop { 0% { transform: scale(0.4) rotate(-6deg); opacity: 0 } 55% { transform: scale(1.25) rotate(2deg); opacity: 1 } 100% { transform: scale(1) rotate(0) } }
</style>
