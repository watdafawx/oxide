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
  const active = (href: string) => (href === "/" ? page.url.pathname === "/" || page.url.pathname.startsWith("/quest") : page.url.pathname.startsWith(href));

  function onKey(e: KeyboardEvent) {
    if (e.key === "Escape" && page.url.pathname.startsWith("/quest") && !app.celebration) void goto("/");
  }
</script>

<svelte:window onkeydown={onKey} />

<div class="flex h-screen flex-col">
  <header class="flex items-center gap-2 border-b border-line bg-panel px-6 py-3">
    <a href="/" class="mr-4 text-xl font-extrabold tracking-tight text-rust">Oxide</a>
    <nav class="flex gap-1">
      {#each nav as n}
        <a href={n.href} class="rounded-lg px-4 py-1.5 font-medium transition {active(n.href) ? 'bg-panel-2 text-ink' : 'text-dim hover:text-ink'}">{n.label}</a>
      {/each}
    </nav>
    <div class="ml-auto flex items-center gap-3">
      {#if app.focus}
        <a href={app.focus.quest_id ? `/quest/${app.focus.quest_id}` : "/"} class="rounded-full border border-rust/40 bg-rust/10 px-3 py-1 font-mono text-sm text-rust-hi">
          ⏱ {mmss(app.focus.remaining)}
        </a>
      {/if}
      {#if !isTauri}<span class="rounded-full border border-line px-2 py-0.5 text-xs text-dim" title="Running against src/lib/mock.ts">mock</span>{/if}
      {#if app.profile}
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
