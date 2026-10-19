<script lang="ts">
  import { sceneFor, sceneWorlds } from "$lib/scenes";
  import type { ObjectiveResult, Quest } from "$lib/types";

  const dev = import.meta.env.DEV;
  const worlds = [...sceneWorlds.map((w) => `rust/${w}`), "rust/00-tutorial"];
  const levels = [{ n: 0, label: "0%" }, { n: 3, label: "50%" }, { n: 6, label: "100%" }];

  function quest(world: string): Quest {
    return { id: `${world}/demo`, track_id: "rust", flavour: "games", world_id: world, runner: "cargo", index: 1, slug: "demo", title: "Demo", summary: "", kind: "exercise", xp: 60, minutes: 10, tags: [], requires: [], objectives: [], hints: [], quiz: [] };
  }
  const objs = (passed: number): ObjectiveResult[] =>
    Array.from({ length: 6 }, (_, i) => ({ test: `t${i}`, label: `objective ${i + 1}`, status: i < passed ? "passed" : "notrun", output: null }));
</script>

<div class="mx-auto max-w-6xl px-6 py-8">
  <h1 class="mb-6 text-3xl font-extrabold">Scenes</h1>
  {#if !dev}
    <p class="text-dim">Dev only.</p>
  {:else}
    {#each worlds as w}
      {@const Scene = sceneFor(w)}
      <section class="mb-8">
        <h2 class="mb-2 font-mono text-sm text-rust-hi">{w}</h2>
        <div class="grid gap-4 md:grid-cols-3">
          {#each levels as l}
            <figure class="overflow-hidden rounded-2xl border border-line bg-panel" data-testid="scene-{w.split('/')[1]}-{l.label}">
              <Scene quest={quest(w)} objectives={objs(l.n)} />
              <figcaption class="px-3 py-1 text-xs text-dim">{l.label}</figcaption>
            </figure>
          {/each}
        </div>
      </section>
    {/each}
  {/if}
</div>
