import { api, listen } from "./api";
import { ding } from "./sound";
import type { Achievement, CheckResult, Completion, Profile } from "./types";

export interface CheckState { running: boolean; result: CheckResult | null; tick: number }
export interface Toast { id: number; kind: "error" | "info"; text: string }

class App {
  profile = $state<Profile | null>(null);
  achievements = $state<Achievement[]>([]);
  checks = $state<Record<string, CheckState>>({});
  focus = $state<{ remaining: number; total: number; quest_id: string } | null>(null);
  toasts = $state<Toast[]>([]);
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
    try {
      [this.profile, this.achievements] = await Promise.all([api.getProfile(), api.listAchievements()]);
    } catch (e) {
      this.error(e);
    }
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
