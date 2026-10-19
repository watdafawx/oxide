<script lang="ts">
  import { app } from "$lib/store.svelte";
  const p = $derived(app.profile);
  const pct = $derived(p ? Math.min(100, (p.xp_into_level / Math.max(1, p.xp_for_next)) * 100) : 0);
</script>

{#if p}
  <div class="flex items-center gap-5 rounded-2xl border border-line bg-panel px-5 py-3" data-testid="xpbar">
    <span class="rounded-lg bg-rust px-3 py-1 font-extrabold text-black">Lv {p.level}</span>
    <div class="min-w-0 flex-1">
      <div class="h-3 overflow-hidden rounded-full bg-panel-2">
        <div class="h-full rounded-full bg-gradient-to-r from-rust to-amber-300 transition-all duration-700" style="width:{pct}%"></div>
      </div>
      <div class="mt-1 text-xs text-dim tabular-nums">{p.xp_into_level} / {p.xp_for_next} XP to level {p.level + 1}</div>
    </div>
    <div class="flex items-center gap-2 font-bold {p.streak_days > 0 ? 'text-rust-hi' : 'text-dim'}" title="Day streak">
      <span class={p.streak_days > 0 ? "flame text-2xl" : "text-2xl grayscale"}>🔥</span>
      <span class="text-xl tabular-nums">{p.streak_days}</span>
    </div>
  </div>
{/if}
