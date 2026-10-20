<script lang="ts">
  import { goto } from "$app/navigation";
  import { onDestroy, tick, untrack } from "svelte";
  import { api } from "$lib/api";
  import { step } from "$lib/sound";
  import { app } from "$lib/store.svelte";
  import { kindLabel } from "$lib/ui";
  import type { Quest, World } from "$lib/types";

  let worlds = $state<World[]>([]);
  let loaded = $state(false);

  $effect(() => {
    void app.profile?.completed.length;
    void app.profile?.settings.flavour;
    const t = app.track?.id;
    api.listWorlds(t).then((w) => { worlds = w; loaded = true; }).catch((e) => app.error(e));
  });

  type NodeState = "locked" | "available" | "started" | "done";
  function stateOf(q: Quest): NodeState {
    const p = app.profile;
    if (!p) return "locked";
    if (p.completed.includes(q.id)) return "done";
    if (!q.requires.every((r) => p.completed.includes(r))) return "locked";
    return p.started.includes(q.id) ? "started" : "available";
  }
  const done = (w: World) => w.quests.filter((q) => app.profile?.completed.includes(q.id)).length;
  function title(q: Quest) {
    const base = `${q.title} (${kindLabel[q.kind]}, +${q.xp} XP)`;
    const p = app.profile;
    const foreign = p ? q.requires.find((r) => !p.completed.includes(r) && r.split("/")[0] !== q.track_id) : undefined;
    return foreign ? `${base}. Needs ${app.questPath(foreign)} (other track)` : base;
  }

  // ---------- layout: one winding path through every world ----------
  const W = 720, PITCH = 108, HEAD = 96, TOP = 20;
  interface MapNode { q: Quest; s: NodeState; x: number; y: number; r: number }
  interface Banner { w: World; y: number }

  const layout = $derived.by(() => {
    const nodes: MapNode[] = [];
    const banners: Banner[] = [];
    let y = TOP;
    for (const w of worlds) {
      banners.push({ w, y });
      y += HEAD;
      for (const q of w.quests) {
        const r = q.kind === "boss" ? 38 : 29;
        nodes.push({ q, s: stateOf(q), x: Math.round(W / 2 + 230 * Math.sin(nodes.length * 0.85 + 0.6)), y, r });
        y += PITCH;
      }
      y += 12;
    }
    return { nodes, banners, height: y + 16 };
  });
  const nodes = $derived(layout.nodes);
  /** where the character stands: first open quest, or the last node when everything is cleared */
  const currentIdx = $derived.by(() => {
    const i = nodes.findIndex((n) => n.s === "available" || n.s === "started");
    return i >= 0 ? i : nodes.every((n) => n.s === "done") ? nodes.length - 1 : 0;
  });

  // ---------- character ----------
  const feet = (n: MapNode) => ({ x: n.x, y: n.y - n.r + 2 });
  let pos = $state<{ x: number; y: number } | null>(null);
  let facing = $state(1);
  let walking = $state(false);
  let hop = $state(0);
  let puff = $state(0);
  let standingId: string | null = null; // quest id the character is on (or walking to)
  let standingTrack = "";
  let raf = 0;
  let boot = true;
  const reduced = typeof matchMedia !== "undefined" && matchMedia("(prefers-reduced-motion: reduce)").matches;
  const storeKey = (t: string) => `oxide.map.at.${t}`;
  const readSaved = (t: string): string | null => { try { return localStorage.getItem(storeKey(t)); } catch { return null; } };
  const writeSaved = (t: string, id: string) => { try { localStorage.setItem(storeKey(t), id); } catch { /* ignore */ } };

  function faceAt(i: number) {
    const a = nodes[i], b = nodes[i + 1];
    if (a && b) facing = b.x >= a.x ? 1 : -1;
  }
  function placeAt(i: number) {
    pos = feet(nodes[i]);
    faceAt(i);
  }
  const ease = (t: number) => (t < 0.5 ? 2 * t * t : 1 - Math.pow(-2 * t + 2, 2) / 2);

  function walk(from: number, to: number) {
    cancelAnimationFrame(raf);
    const idxs: number[] = [];
    for (let i = from; i !== to; i += from < to ? 1 : -1) idxs.push(i);
    idxs.push(to);
    const pts = idxs.map((i) => feet(nodes[i]));
    const lens = pts.slice(1).map((p, i) => Math.hypot(p.x - pts[i].x, p.y - pts[i].y));
    const total = lens.reduce((a, b) => a + b, 0) || 1;
    const arrive = () => {
      walking = false;
      pos = pts[pts.length - 1];
      faceAt(to);
      hop++;
      puff++;
      step();
    };
    if (reduced) return arrive();
    walking = true;
    const t0 = performance.now();
    const DUR = 800;
    const frame = (now: number) => {
      const t = Math.min(1, (now - t0) / DUR);
      let d = ease(t) * total;
      let k = 0;
      while (k < lens.length - 1 && d > lens[k]) { d -= lens[k]; k++; }
      const f = lens[k] ? Math.min(1, d / lens[k]) : 1;
      const a = pts[k], b = pts[k + 1] ?? a;
      const bob = t < 1 ? -Math.abs(Math.sin(t * Math.PI * Math.max(2, total / 38))) * 4 : 0;
      pos = { x: a.x + (b.x - a.x) * f, y: a.y + (b.y - a.y) * f + bob };
      if (b.x !== a.x) facing = b.x > a.x ? 1 : -1;
      if (t < 1) raf = requestAnimationFrame(frame);
      else arrive();
    };
    raf = requestAnimationFrame(frame);
  }

  $effect(() => {
    if (!loaded || !nodes.length) return;
    const target = nodes[currentIdx];
    const track = target.q.track_id;
    untrack(() => {
      if (standingTrack !== track) { standingTrack = track; standingId = null; }
      if (standingId === null) {
        const saved = readSaved(track);
        const si = saved ? nodes.findIndex((n) => n.q.id === saved) : -1;
        const from = si >= 0 ? si : currentIdx;
        standingId = nodes[from].q.id;
        placeAt(from);
        if (boot) {
          boot = false;
          tick().then(() => document.querySelector("[data-testid=hero]")?.scrollIntoView({ block: "center" }));
        }
      }
      if (standingId !== target.q.id) {
        const from = nodes.findIndex((n) => n.q.id === standingId);
        standingId = target.q.id;
        if (from < 0) placeAt(currentIdx);
        else walk(from, currentIdx);
      }
      writeSaved(track, target.q.id);
    });
  });
  onDestroy(() => cancelAnimationFrame(raf));

  // ---------- keyboard ----------
  let hl = $state(0);
  $effect(() => { hl = currentIdx; });
  function onKey(e: KeyboardEvent) {
    const d = e.key === "ArrowRight" || e.key === "ArrowDown" ? 1 : e.key === "ArrowLeft" || e.key === "ArrowUp" ? -1 : 0;
    if (!d || !nodes.length) return;
    e.preventDefault();
    hl = Math.max(0, Math.min(nodes.length - 1, hl + d));
    (document.querySelector(`[data-node="${hl}"]`) as HTMLElement | null)?.focus();
  }
  function open(n: MapNode) {
    if (n.s !== "locked") goto(`/quest/${n.q.id}`);
  }

  const ring: Record<NodeState, string> = {
    locked: "border-line bg-panel text-dim/40 cursor-not-allowed",
    available: "border-rust bg-panel-2 text-rust-hi hover:bg-rust/20 cursor-pointer",
    started: "border-amber-300 bg-amber-300/15 text-amber-200 cursor-pointer",
    done: "border-ok bg-ok/15 text-ok cursor-pointer shadow-[0_0_18px_-2px_rgba(74,222,128,.45)]",
  };
  const glyph: Record<NodeState, string> = { locked: "🔒", available: "", started: "…", done: "✓" };
  const gate: Record<NodeState, string> = { locked: "#4a433c", available: "#f97316", started: "#fcd34d", done: "#4ade80" };

  // ---------- sprite: 16x10 rust-orange crab ----------
  const CRAB = [
    "................",
    ".oo..........oo.",
    "oooo.ww..ww.oooo",
    "oo.o.oooooo.o.oo",
    "ooo.oooooooo.ooo",
    ".oo.oooooooo.oo.",
    "....oooooooo....",
    "....dddddddd....",
    "..o.o.o..o.o.o..",
    ".o..o.o..o.o..o.",
  ];
  const COL: Record<string, string> = { o: "#f97316", d: "#c2410c", w: "#fff7ed" };
  const crab = CRAB.flatMap((row, y) => [...row].flatMap((c, x) => (COL[c] ? [{ x, y, fill: COL[c] }] : [])));
</script>

<div class="mx-auto max-w-3xl px-6 py-10">
  <h1 class="mb-6 text-3xl font-extrabold">Map{#if app.track}<span class="ml-3 text-xl font-bold text-dim">{app.track.title}</span>{/if}</h1>
  {#if !loaded}
    <div class="h-48 animate-pulse rounded-2xl bg-panel"></div>
  {/if}

  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div class="overflow-x-auto rounded-3xl border border-line bg-panel" role="group" aria-label="Quest map" onkeydown={onKey}>
    <div class="relative mx-auto" style="width:{W}px;height:{layout.height}px">
      <!-- dotted path -->
      <svg class="pointer-events-none absolute inset-0" width={W} height={layout.height} aria-hidden="true">
        {#each nodes as n, i (n.q.id)}
          {#if i > 0}
            {@const p = nodes[i - 1]}
            <line x1={p.x} y1={p.y} x2={n.x} y2={n.y} stroke={p.s === "done" ? "#4ade80" : n.s === "locked" ? "#38332e" : "#f97316"}
              stroke-opacity={p.s === "done" ? 0.85 : n.s === "locked" ? 1 : 0.55} stroke-width="4" stroke-linecap="round" stroke-dasharray="0.1 11" />
          {/if}
        {/each}
      </svg>

      {#each layout.banners as b (b.w.id)}
        <div class="absolute inset-x-6 flex items-baseline justify-between gap-4" style="top:{b.y}px">
          <div>
            <h2 class="text-xl font-bold">{b.w.title}</h2>
            <p class="text-sm text-dim">{b.w.blurb}</p>
          </div>
          <span class="font-bold text-rust-hi tabular-nums">{done(b.w)}/{b.w.quests.length}</span>
        </div>
      {/each}

      {#each nodes as n, i (n.q.id)}
        {@const boss = n.q.kind === "boss"}
        <button
          class="group absolute flex items-center justify-center border-2 text-lg font-bold transition focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-sky-300
            {boss ? 'rounded-2xl' : 'rounded-full'} {ring[n.s]}"
          style="left:{n.x - n.r}px;top:{n.y - n.r}px;width:{n.r * 2}px;height:{n.r * 2}px"
          aria-disabled={n.s === "locked"}
          tabindex={i === hl ? 0 : -1}
          data-node={i}
          title={title(n.q)}
          aria-label="{title(n.q)} — {n.s}"
          data-state={n.s}
          onclick={() => open(n)}
          onfocus={() => (hl = i)}
        >
          {#if boss}
            <svg viewBox="0 0 32 36" class="h-11 w-10" aria-hidden="true">
              <path d="M3 34V16a13 13 0 0 1 26 0v18z" fill="none" stroke={gate[n.s]} stroke-width="2.5" stroke-linejoin="round" />
              <path d="M9 34V17a7 7 0 0 1 14 0v17z" fill={gate[n.s]} fill-opacity={n.s === "locked" ? 0.12 : 0.35} stroke={gate[n.s]} stroke-width="1.5" />
              <circle cx="16" cy="25" r="1.8" fill={gate[n.s]} />
              <path d="M12 3l4-2 4 2" stroke={gate[n.s]} stroke-width="1.5" fill="none" stroke-linecap="round" />
            </svg>
          {:else}
            <span>{glyph[n.s] || (n.q.kind === "read" ? "📖" : n.q.index)}</span>
          {/if}
        </button>
        <div class="pointer-events-none absolute w-40 text-center text-xs leading-tight {n.s === 'locked' ? 'text-dim/40' : 'text-dim'}"
          style="left:{n.x - 80}px;top:{n.y + n.r + 5}px">{n.q.title}</div>
      {/each}

      <!-- character -->
      {#if pos}
        <div class="pointer-events-none absolute left-0 top-0 z-10" data-testid="hero" data-walking={walking}
          style="transform:translate({pos.x - 24}px,{pos.y - 31}px)">
          {#key hop}
            <div class={hop ? "hop" : ""}>
              <svg width="48" height="30" viewBox="0 0 16 10" shape-rendering="crispEdges" style="transform:scaleX({facing})" aria-label="Crab" role="img">
                {#each crab as p}<rect x={p.x} y={p.y} width="1" height="1" fill={p.fill} />{/each}
                <rect x={facing > 0 ? 6 : 5} y="2" width="1" height="1" fill="#1c1a18" />
                <rect x={facing > 0 ? 10 : 9} y="2" width="1" height="1" fill="#1c1a18" />
              </svg>
            </div>
          {/key}
          {#key puff}
            {#if puff}
              <div class="absolute left-0 top-[28px] h-3 w-12" aria-hidden="true">
                <span class="dust" style="left:8px;--dx:-9px"></span>
                <span class="dust" style="left:22px;--dx:0px;animation-delay:30ms"></span>
                <span class="dust" style="left:36px;--dx:9px"></span>
              </div>
            {/if}
          {/key}
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .hop { animation: hop 0.28s ease-out; }
  @keyframes hop { 0% { transform: translateY(0) } 40% { transform: translateY(-12px) } 100% { transform: translateY(0) } }
  .dust { position: absolute; top: 2px; width: 6px; height: 6px; border-radius: 9999px; background: #a89d92; opacity: 0; animation: dust 0.55s ease-out 0.12s both; }
  @keyframes dust { 0% { transform: translate(0, 0) scale(0.5); opacity: 0.7 } 100% { transform: translate(var(--dx), -7px) scale(1.4); opacity: 0 } }
  @media (prefers-reduced-motion: reduce) { .hop, .dust { animation: none } }
</style>
