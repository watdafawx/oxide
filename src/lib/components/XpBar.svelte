<script lang="ts">
  import { app } from "$lib/store.svelte";
  const p = $derived(app.profile);
  // pulse the combo counter whenever it goes up
  let pulse = $state(0);
  let prevCombo = app.profile?.combo ?? 0;
  $effect(() => {
    const c = p?.combo ?? 0;
    if (c > prevCombo) pulse++;
    prevCombo = c;
  });
  const comboPct = $derived(Math.round(((p?.combo ?? 0) >= 10 ? 0.5 : 0.05 * (p?.combo ?? 0)) * 100));
  const pct = $derived(p ? Math.min(100, (p.xp_into_level / Math.max(1, p.xp_for_next)) * 100) : 0);
</script>

{#if p}
  <div class="flex items-center gap-5 rounded-2xl border border-line bg-panel px-5 py-3" data-testid="xpbar">
    <span class="rounded-lg bg-rust px-3 py-1 font-extrabold text-on-rust">Lv {p.level}</span>
    <div class="min-w-0 flex-1">
      <div class="h-3 overflow-hidden rounded-full bg-panel-2">
        <div class="h-full rounded-full bg-gradient-to-r from-rust to-amber-300 transition-all duration-700" style="width:{pct}%"></div>
      </div>
      <div class="mt-1 text-xs text-dim tabular-nums">{p.xp_into_level} / {p.xp_for_next} XP to level {p.level + 1}</div>
    </div>
    {#if p.combo > 0}
      {#key pulse}
        <div class="combo flex items-center gap-1 rounded-lg border border-amber-300/40 bg-amber-300/10 px-2.5 py-1 font-extrabold text-amber-300 tabular-nums" title="Combo: clears in a row without a hint. Next clear gets +{comboPct}% XP." data-testid="combo">
          <span>⚡</span><span>x{p.combo}</span>
        </div>
      {/key}
    {/if}
    <div class="flex items-center gap-2 font-bold {p.streak_days > 0 ? 'text-rust-hi' : 'text-dim'}" title="Day streak">
      <span class={p.streak_days > 0 ? "flame text-2xl" : "text-2xl grayscale"}>🔥</span>
      <span class="text-xl tabular-nums">{p.streak_days}</span>
      {#if p.freezes > 0}
        <span class="text-lg" title="{p.freezes} streak freeze{p.freezes > 1 ? 's' : ''}: a skipped day uses one instead of resetting" data-testid="freezes">{"❄️".repeat(p.freezes)}</span>
      {/if}
    </div>
  </div>
{/if}

<style>
  .combo { animation: combopulse 0.45s ease-out; }
  @keyframes combopulse { 0% { transform: scale(1) } 40% { transform: scale(1.35); box-shadow: 0 0 14px rgb(252 211 77 / 0.6) } 100% { transform: scale(1) } }
</style>
