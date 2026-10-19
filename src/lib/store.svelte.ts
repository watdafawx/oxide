import { api, listen } from "./api";
import { ding } from "./sound";
import type { Achievement, CheckResult, Completion, Profile, Quest, RunStream, Track, World } from "./types";

export interface CheckState { running: boolean; result: CheckResult | null; tick: number }
export interface ConsoleLine { stream: RunStream | "input"; line: string }
export interface RunState { running: boolean; lines: ConsoleLine[]; code: number | null | undefined }
const MAX_LINES = 2000;
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
