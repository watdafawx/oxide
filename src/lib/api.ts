// Typed wrappers over Tauri commands/events (docs/API.md).
// Without Tauri (plain `pnpm dev`) everything is routed to ./mock.
import type {
  Achievement, CheckResult, Completion, EventMap, Home, Profile, QuestDetail, Settings, World,
} from "./types";
import { mockHandlers, mockListen } from "./mock";

export const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

type NoArgs = Record<string, never>;

export interface Commands {
  home: { args: NoArgs; ret: Home };
  list_worlds: { args: NoArgs; ret: World[] };
  get_quest: { args: { id: string }; ret: QuestDetail };
  start_quest: { args: { id: string }; ret: QuestDetail };
  open_editor: { args: { id: string }; ret: void };
  reset_quest: { args: { id: string }; ret: QuestDetail };
  run_checks: { args: { id: string }; ret: CheckResult };
  answer_quiz: { args: { id: string; answers: number[] }; ret: Completion | null };
  use_hint: { args: { id: string }; ret: number };
  get_profile: { args: NoArgs; ret: Profile };
  list_achievements: { args: NoArgs; ret: Achievement[] };
  start_focus: { args: { id?: string; minutes: number }; ret: number };
  stop_focus: { args: { completed: boolean }; ret: void };
  get_settings: { args: NoArgs; ret: Settings };
  set_settings: { args: { settings: Settings }; ret: Settings };
  active_dates: { args: NoArgs; ret: string[] };
}

export type MockHandlers = {
  [K in keyof Commands]: (args: Commands[K]["args"]) => Promise<Commands[K]["ret"]> | Commands[K]["ret"];
};

async function call<K extends keyof Commands>(cmd: K, args: Commands[K]["args"]): Promise<Commands[K]["ret"]> {
  if (isTauri) {
    const { invoke } = await import("@tauri-apps/api/core");
    try {
      return (await invoke(cmd, args as Record<string, unknown>)) as Commands[K]["ret"];
    } catch (e) {
      throw new Error(typeof e === "string" ? e : ((e as Error)?.message ?? String(e)));
    }
  }
  return mockHandlers[cmd](args);
}

export async function listen<K extends keyof EventMap>(name: K, cb: (payload: EventMap[K]) => void): Promise<() => void> {
  if (isTauri) {
    const { listen: tl } = await import("@tauri-apps/api/event");
    return tl<EventMap[K]>(name, (e) => cb(e.payload));
  }
  return mockListen(name, cb);
}

const E: NoArgs = {};
export const api = {
  home: () => call("home", E),
  listWorlds: () => call("list_worlds", E),
  getQuest: (id: string) => call("get_quest", { id }),
  startQuest: (id: string) => call("start_quest", { id }),
  openEditor: (id: string) => call("open_editor", { id }),
  resetQuest: (id: string) => call("reset_quest", { id }),
  runChecks: (id: string) => call("run_checks", { id }),
  answerQuiz: (id: string, answers: number[]) => call("answer_quiz", { id, answers }),
  useHint: (id: string) => call("use_hint", { id }),
  getProfile: () => call("get_profile", E),
  listAchievements: () => call("list_achievements", E),
  startFocus: (minutes: number, id?: string) => call("start_focus", { id, minutes }),
  stopFocus: (completed: boolean) => call("stop_focus", { completed }),
  getSettings: () => call("get_settings", E),
  setSettings: (settings: Settings) => call("set_settings", { settings }),
  activeDates: () => call("active_dates", E),
};
