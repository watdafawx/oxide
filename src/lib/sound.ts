// Tiny synthesized "ding" (no assets).
let ctx: AudioContext | null = null;

export function ding(kind: "ok" | "level" = "ok") {
  try {
    ctx ??= new AudioContext();
    if (ctx.state === "suspended") void ctx.resume();
    const notes = kind === "level" ? [659.25, 880, 1318.5] : [880, 1318.5];
    const t0 = ctx.currentTime;
    notes.forEach((f, i) => {
      const o = ctx!.createOscillator();
      const g = ctx!.createGain();
      o.type = "sine";
      o.frequency.value = f;
      const t = t0 + i * 0.09;
      g.gain.setValueAtTime(0.0001, t);
      g.gain.exponentialRampToValueAtTime(0.18, t + 0.01);
      g.gain.exponentialRampToValueAtTime(0.0001, t + 0.45);
      o.connect(g).connect(ctx!.destination);
      o.start(t);
      o.stop(t + 0.5);
    });
  } catch {
    /* audio unavailable: ignore */
  }
}
