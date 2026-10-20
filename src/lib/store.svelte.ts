import { goto } from "$app/navigation";
import { api, listen } from "./api";
import { ding } from "./sound";
import type { Achievement, CheckResult, Completion, Profile, Quest, RunStream, Track, World } from "./types";

export interface CheckState { running: boolean; result: CheckResult | null; tick: number }
export interface ConsoleLine { stream: RunStream | "input"; line: string }
export interface RunState { running: boolean; lines: ConsoleLine[]; code: number | null | undefined }
const MAX_LINES = 2000;
export interface NextTarget { from: string; quest: Quest | null; label: string }
export interface Toast { id: number; kind: "error" | "info"; text: string }

class App {
  profile = $state<Profile | null>(null);
  achievements = $state<Achievement[]>([]);
  checks = $state<Record<string, CheckState>>({});
  focus = $state<{ remaining: number; total: number; quest_id: string } | null>(null);
  toasts = $state<Toast[]>([]);
  tracks = $state<Track[]>([]);
  /** worlds of every track, keyed by track id; static, used for cross-track lookups */
  catalog = $state<Record<string, World[]>>({});
  /** program console per quest id (Run on the quest page) */
  runs = $state<Record<string, RunState>>({});
  trackMenu = $state(false);
  celebration = $state<Completion | null>(null);
  /** what to offer after `from` was cleared; quest null = nothing unlocked */
  next = $state<NextTarget | null>(null);
  /** quests whose starter run (red) is still to come; the quest page must not treat it as a failed attempt */
  fresh = new Set<string>();
  private started = false;
  private tid = 0;

  async init() {
    if (this.started) return;
    this.started = true;
    await listen("profile:changed", (p) => (this.profile = p));
    await listen("check:started", ({ quest_id }) => {
      this.checks[quest_id] = { running: true, result: this.checks[quest_id]?.result ?? null, tick: this.checks[quest_id]?.tick ?? 0 };
    });
    await listen("check:result", ({ quest_id, result, completion }) => {
      this.checks[quest_id] = { running: false, result, tick: (this.checks[quest_id]?.tick ?? 0) + 1 };
      if (result.ok) this.ding();
      if (completion?.first_time) this.celebrate(completion);
    });
    await listen("focus:tick", (t) => (this.focus = { remaining: t.remaining_s, total: t.total_s, quest_id: t.quest_id }));
    await listen("focus:done", () => {
      this.focus = null;
      this.toast("info", "Focus session done. Nice.");
      this.ding("level");
    });
    await listen("run:output", ({ quest_id, stream, line }) => this.runLine(quest_id, { stream, line }));
    await listen("run:done", ({ quest_id, code }) => {
      const r = this.runs[quest_id];
      if (!r) return;
      r.running = false;
      r.code = code;
      this.runLine(quest_id, { stream: "system", line: code === null ? "stopped" : `exited with code ${code}` });
    });
    try {
      [this.profile, this.achievements, this.tracks] = await Promise.all([api.getProfile(), api.listAchievements(), api.listTracks()]);
      const all = await Promise.all(this.tracks.map((t) => api.listWorlds(t.id)));
      this.tracks.forEach((t, i) => (this.catalog[t.id] = all[i]));
    } catch (e) {
      this.error(e);
    }
  }

  /** clear the console and mark the program as running */
  startRun(id: string) {
    this.runs[id] = { running: true, lines: [], code: undefined };
  }
  runLine(id: string, l: ConsoleLine) {
    const r = this.runs[id];
    if (!r) return;
    r.lines.push(l);
    if (r.lines.length > MAX_LINES) r.lines.splice(0, r.lines.length - MAX_LINES);
  }

  /** active track: settings.track, "" = first */
  get track(): Track | null {
    const id = this.profile?.settings.track;
    return this.tracks.find((t) => t.id === id) ?? this.tracks[0] ?? null;
  }

  async switchTrack(id: string) {
    this.trackMenu = false;
    if (!this.profile || id === this.track?.id) return;
    try {
      await api.setSettings({ ...this.profile.settings, track: id });
      this.profile = await api.getProfile();
    } catch (e) {
      this.error(e);
    }
  }

  quest(id: string): Quest | null {
    for (const ws of Object.values(this.catalog)) for (const w of ws) for (const q of w.quests) if (q.id === id) return q;
    return null;
  }

  /** "Rust › Foundations › Functions" */
  questPath(id: string): string {
    const q = this.quest(id);
    if (!q) return id;
    const t = this.tracks.find((x) => x.id === q.track_id)?.title ?? q.track_id;
    const w = this.catalog[q.track_id]?.find((x) => x.id === q.world_id)?.title ?? q.world_id;
    return `${t} › ${w} › ${q.title}`;
  }

  ding(kind: "ok" | "level" = "ok") {
    if (this.profile?.settings.sound !== false) ding(kind);
  }

  celebrate(c: Completion) {
    this.celebration = c;
    void this.refreshNext(c.quest_id);
  }

  /** ask `home` what comes after `from` */
  async refreshNext(from: string) {
    try {
      const h = await api.home();
      const n = h.next && h.next.id !== from ? h.next : null;
      let label = "Next quest →";
      if (n && n.world_id !== this.quest(from)?.world_id) {
        const w = this.catalog[n.track_id]?.find((x) => x.id === n.world_id)?.title ?? n.world_id;
        label = `World cleared · next: ${w}`;
      }
      this.next = { from, quest: n, label };
    } catch (e) {
      this.error(e);
    }
  }

  /** start (copy the starter, run it once) and remember that its first red result is not the learner's */
  async beginQuest(id: string) {
    this.fresh.add(id);
    try {
      return await api.startQuest(id);
    } catch (e) {
      this.fresh.delete(id);
      throw e;
    }
  }

  async goNext() {
    const n = this.next?.quest;
    if (!n) return;
    try {
      const p = this.profile;
      if (n.kind !== "read" && !p?.started.includes(n.id) && !p?.completed.includes(n.id)) await this.beginQuest(n.id);
      await goto(`/quest/${n.id}`);
    } catch (e) {
      this.error(e);
    }
  }

  toast(kind: Toast["kind"], text: string) {
    const id = ++this.tid;
    this.toasts.push({ id, kind, text });
    setTimeout(() => (this.toasts = this.toasts.filter((t) => t.id !== id)), kind === "error" ? 6000 : 3500);
  }
  error(e: unknown) {
    this.toast("error", e instanceof Error ? e.message : String(e));
  }

  achievementTitle(id: string) {
    return this.achievements.find((a) => a.id === id)?.title ?? id;
  }
}

export const app = new App();
