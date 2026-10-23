<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { app } from "$lib/store.svelte";
  import { isTauri } from "$lib/api";
  import { mmss } from "$lib/ui";
  import Celebration from "$lib/components/Celebration.svelte";

  let { children } = $props();

  onMount(() => {
    void app.init();
  });

  const nav = [
    { href: "/", label: "Home" },
    { href: "/map", label: "Map" },
    { href: "/profile", label: "Profile" },
  ];
  // Top-level tracks, each followed by the project tracks that sit under it (`parent` in track.toml).
  const menu = $derived(
    app.tracks
      .filter((t) => !t.parent || !app.tracks.some((x) => x.id === t.parent))
      .flatMap((t) => [{ t, child: false }, ...app.tracks.filter((c) => c.parent === t.id).map((c) => ({ t: c, child: true }))]),
  );
  /** The quest a track's first quest still waits for, as "Track › World › Quest", or null when it is open. */
  function waitsFor(trackId: string): string | null {
    const first = app.catalog[trackId]?.[0]?.quests[0];
    const p = app.profile;
    const missing = first && p ? first.requires.find((r) => !p.completed.includes(r)) : undefined;
    return missing ? app.questPath(missing) : null;
  }

  const active = (href: string) => (href === "/" ? page.url.pathname === "/" || page.url.pathname.startsWith("/quest") : page.url.pathname.startsWith(href));

  function onKey(e: KeyboardEvent) {
    const el = e.target as HTMLElement | null;
    const typing = !!el && (el.isContentEditable || ["INPUT", "TEXTAREA", "SELECT"].includes(el.tagName));
    if (e.key === "t" && !e.ctrlKey && !e.metaKey && !e.altKey && !typing && app.tracks.length) {
      e.preventDefault();
      app.trackMenu = !app.trackMenu;
      return;
    }
    if (e.key === "Escape" && app.trackMenu) { app.trackMenu = false; return; }
    if (e.key === "Escape" && page.url.pathname.startsWith("/quest") && !app.celebration) void goto("/");
  }
</script>

<svelte:window onkeydown={onKey} onclick={(e) => { if (app.trackMenu && !(e.target as HTMLElement).closest("[data-track-menu]")) app.trackMenu = false; }} />

<div class="flex h-screen flex-col">
  <header class="flex items-center gap-2 border-b border-line bg-panel px-6 py-3">
    <a href="/" class="mr-4 text-xl font-extrabold tracking-tight text-rust">Oxide</a>
    <nav class="flex gap-1">
      {#each nav as n}
        <a href={n.href} class="rounded-lg px-4 py-1.5 font-medium transition {active(n.href) ? 'bg-panel-2 text-ink' : 'text-dim hover:text-ink'}">{n.label}</a>
      {/each}
    </nav>
    <div class="ml-auto flex items-center gap-3">
      {#if app.track}
        {@const cur = app.track}
        <div class="relative" data-track-menu>
          <button
            class="flex items-center gap-2 rounded-full border border-line bg-panel-2 px-3 py-1 text-sm font-semibold hover:border-rust/50"
            aria-haspopup="menu" aria-expanded={app.trackMenu} data-testid="track-pill"
            onclick={() => (app.trackMenu = !app.trackMenu)}
          >
            {cur.title} <span class="text-dim">▾</span><kbd class="rounded border border-line px-1 text-xs text-dim">t</kbd>
          </button>
          {#if app.trackMenu}
            <div class="pop absolute right-0 z-40 mt-2 w-72 rounded-2xl border border-line bg-panel p-2 shadow-xl" role="menu" data-testid="track-menu">
              {#each menu as { t, child } (t.id)}
                {@const lock = child ? waitsFor(t.id) : null}
                <button
                  role="menuitem"
                  class="flex w-full flex-col rounded-xl px-3 py-2 text-left hover:bg-panel-2 {t.id === cur.id ? 'bg-panel-2' : ''} {child ? 'ml-4 w-[calc(100%-1rem)] border-l-2 border-line rounded-l-none' : ''}"
                  data-testid={child ? "track-child" : undefined}
                  onclick={() => app.switchTrack(t.id)}
                >
                  <span class="flex items-center justify-between font-semibold">
                    <span>{#if lock}<span class="mr-1" aria-label="locked">🔒</span>{/if}{t.title}</span>{#if t.id === cur.id}<span class="text-rust-hi">✓</span>{/if}
                  </span>
                  <span class="text-sm text-dim">{t.blurb}</span>
                  {#if lock}<span class="mt-1 text-xs text-dim">Unlocks after {lock}</span>{/if}
                </button>
              {/each}
            </div>
          {/if}
        </div>
      {/if}
      {#if app.focus}
        <a href={app.focus.quest_id ? `/quest/${app.focus.quest_id}` : "/"} class="rounded-full border border-rust/40 bg-rust/10 px-3 py-1 font-mono text-sm text-rust-hi">
          ⏱ {mmss(app.focus.remaining)}
        </a>
      {/if}
      {#if !isTauri}<span class="rounded-full border border-line px-2 py-0.5 text-xs text-dim" title="Running against src/lib/mock.ts">mock</span>{/if}
      {#if app.profile}
        {#if app.profiles.length > 1 && app.activeProfile}
          <a href="/profile" class="text-sm font-semibold text-dim hover:text-ink" title="Switch profile on the Profile page" data-testid="profile-name">{app.activeProfile.name}</a>
        {/if}
        <span class="rounded-full bg-rust px-3 py-1 text-sm font-bold text-black">Lv {app.profile.level}</span>
      {/if}
    </div>
  </header>

  <main class="min-h-0 flex-1 overflow-y-auto">
    {@render children()}
  </main>
</div>

<Celebration />

<div class="pointer-events-none fixed right-4 bottom-4 z-50 flex flex-col gap-2">
  {#each app.toasts as t (t.id)}
    <div class="pop pointer-events-auto max-w-sm rounded-xl border px-4 py-3 shadow-xl {t.kind === 'error' ? 'border-bad/50 bg-[#2a1614] text-bad' : 'border-line bg-panel-2 text-ink'}">
      {t.text}
    </div>
  {/each}
</div>
