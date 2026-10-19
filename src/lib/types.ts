// Mirrors docs/API.md. Keep in sync with crates/oxide-core.
export type QuestKind = "exercise" | "boss" | "read";

export interface Objective { test: string; label: string }
export interface Hint { text: string }
export interface Quiz { question: string; choices: string[]; answer: number }

export interface Quest {
  id: string;
  world_id: string;
  index: number;
  slug: string;
  title: string;
  summary: string;
  kind: QuestKind;
  xp: number;
  minutes: number;
  tags: string[];
  requires: string[];
  objectives: Objective[];
  hints: Hint[];
  quiz: Quiz[];
}

export interface World { id: string; index: number; slug: string; title: string; blurb: string; quests: Quest[] }

export interface Settings { editor: string; sound: boolean; focus_minutes: number }

export interface Profile {
  xp: number;
  level: number;
  xp_into_level: number;
  xp_for_next: number;
  streak_days: number;
  best_streak: number;
  last_active: string | null;
  achievements: string[];
  completed: string[];
  started: string[];
  settings: Settings;
}

export interface Achievement { id: string; title: string; description: string }

export type ObjectiveStatus = "passed" | "failed" | "notrun";
export interface ObjectiveResult { test: string; label: string; status: ObjectiveStatus; output: string | null }
export interface Diagnostic { level: "error" | "warning"; message: string; rendered: string; file: string | null; line: number | null }
export interface CheckResult {
  ok: boolean;
  compiled: boolean;
  objectives: ObjectiveResult[];
  diagnostics: Diagnostic[];
  other_failures: string[];
  raw_tail: string;
  duration_ms: number;
}

export interface Completion {
  quest_id: string;
  xp_awarded: number;
  level_before: number;
  level_after: number;
  new_achievements: string[];
  focus_bonus: boolean;
  clean: boolean;
  first_time: boolean;
}

export interface QuestDetail {
  quest: Quest;
  lesson: string;
  workspace: string | null;
  hints_used: number;
  attempts: number;
  completed: boolean;
  last_check: CheckResult | null;
}

export interface Home {
  next: Quest | null;
  alternate: Quest | null;
  profile: Profile;
  resume: Quest | null;
}

// Event payloads
export interface CheckStarted { quest_id: string }
export interface CheckResultEvent { quest_id: string; result: CheckResult; completion: Completion | null }
export interface FocusTick { remaining_s: number; total_s: number; quest_id: string }
export interface FocusDone { quest_id: string }

export interface EventMap {
  "check:started": CheckStarted;
  "check:result": CheckResultEvent;
  "profile:changed": Profile;
  "focus:tick": FocusTick;
  "focus:done": FocusDone;
}
