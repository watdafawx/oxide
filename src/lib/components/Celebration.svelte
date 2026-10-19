<script lang="ts">
  import confetti from "canvas-confetti";
  import { app } from "$lib/store.svelte";
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
    timer = setTimeout(() => (toastC = null), 5000);
    if (levelUp) {
      modalC = c;
      showLevel = true;
      setTimeout(() => app.ding("level"), 350);
    }
  });

  function closeModal() {
    showLevel = false;
    modalC = null;
  }
</script>

<svelte:window onkeydown={(e) => { if (showLevel && (e.key === "Escape" || e.key === "Enter")) { e.stopPropagation(); closeModal(); } }} />

{#if toastC && !showLevel}
  <div class="pop fixed bottom-8 left-1/2 z-50 -translate-x-1/2 rounded-2xl border border-rust/50 bg-panel-2 px-8 py-4 text-center shadow-2xl" data-testid="xp-toast">
    <div class="text-4xl font-extrabold text-rust-hi tabular-nums">+{shown} XP</div>
    <div class="text-sm text-dim">Quest cleared{toastC.clean ? " · clean compile" : ""}{toastC.focus_bonus ? " · focus bonus" : ""}</div>
    {#each toastC.new_achievements as a}<div class="mt-1 text-sm font-semibold text-amber-200">🏆 {app.achievementTitle(a)}</div>{/each}
  </div>
{/if}

{#if showLevel && modalC}
  <div class="fixed inset-0 z-[60] flex items-center justify-center bg-black/70 p-6" role="dialog" aria-modal="true" aria-label="Level up">
    <div class="pop w-full max-w-md rounded-3xl border border-rust/60 bg-panel p-10 text-center shadow-2xl">
      <div class="text-sm font-semibold tracking-widest text-rust-hi uppercase">Level up</div>
      <div class="my-3 text-8xl font-black text-rust">{modalC.level_after}</div>
      <div class="text-dim">+{modalC.xp_awarded} XP · level {modalC.level_before} → {modalC.level_after}</div>
      {#if modalC.new_achievements.length}
        <div class="mt-6 text-left">
          <div class="mb-2 text-sm text-dim">New achievements</div>
          <ul class="space-y-2">
            {#each modalC.new_achievements as a}
              <li class="rounded-xl border border-line bg-panel-2 px-4 py-2 font-semibold">🏆 {app.achievementTitle(a)}</li>
            {/each}
          </ul>
        </div>
      {/if}
      <button class="btn btn-primary btn-lg mt-8 w-full" onclick={closeModal}>Keep going</button>
    </div>
  </div>
{/if}
