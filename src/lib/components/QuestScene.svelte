<script lang="ts">
  import { sceneFor } from "$lib/scenes";
  import type { ObjectiveResult, Quest } from "$lib/types";

  let { quest, objectives }: { quest: Quest; objectives: ObjectiveResult[] } = $props();

  const KEY = "oxide.scene.collapsed";
  let collapsed = $state(false);
  try { collapsed = localStorage.getItem(KEY) === "1"; } catch { /* storage unavailable */ }
  function toggle() {
    collapsed = !collapsed;
    try { localStorage.setItem(KEY, collapsed ? "1" : "0"); } catch { /* ignore */ }
  }

  const Scene = $derived(sceneFor(quest.world_id));
</script>

<section class="overflow-hidden rounded-2xl border border-line bg-panel" data-testid="scene">
  <button class="flex w-full items-center justify-between px-4 py-2 text-left text-sm text-dim hover:text-ink" onclick={toggle} aria-expanded={!collapsed}>
    <span class="font-semibold">Your build</span>
    <span aria-hidden="true">{collapsed ? "▸" : "▾"}</span>
  </button>
  {#if !collapsed}<Scene {quest} {objectives} />{/if}
</section>
