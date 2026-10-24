// Mirrors docs/API.md. Keep in sync with crates/oxide-core.
export type QuestKind = "exercise" | "boss" | "read";
export type Runner = "cargo" | "lua" | "gradle";

export interface Track { id: string; title: string; blurb: string; runner: Runner; /** listed under this track in the menu */ parent?: string | null }

export interface Objective { test: string; label: string }
export interface Hint { text: string }
export interface Quiz { question: string; choices: string[]; answer: number }

export interface Point { after: number; prompt: string; pick: string[]; answer: number | null; blank: string | null; code: string | null }

export interface Quest {
  id: string;
  track_id: string;
  flavour: string;
  world_id: string;
  runner: Runner;
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
  points: Point[];
}

export interface World { id: string; track_id: string; index: number; slug: string; title: string; blurb: string; quests: Quest[] }

export interface FileEntry { path: string; bytes: number; read_only: boolean; primary: boolean }

export type SoundKey = "objective" | "clear" | "level" | "achievement" | "crit";
export type SoundToggles = Record<SoundKey, boolean>;
export interface Settings {
  editor: string; sound: boolean; focus_minutes: number; track: string; flavour: string;
  /** master volume 0-1 */
  volume: number;
  sounds: SoundToggles;
  /** "workbench" | "notebook" | "arcade" */
  look: Look;
  /** editor suggests names as you type */
  complete: boolean;
  /** plain-English note under compiler errors */
  explain: boolean;
}
export type Look = "workbench" | "notebook" | "arcade";

/** One saved profile (list_profiles). */
export interface ProfileInfo { id: string; name: string; created: string; active: boolean }

export interface Profile {
  xp: number;
  level: number;
  xp_into_level: number;
  xp_for_next: number;
  streak_days: number;
  best_streak: number;
  last_active: string | null;
  combo: number;
  freezes: number;
  achievements: string[];
  completed: string[];
  started: string[];
  settings: Settings;
}

export interface Achievement { id: string; title: string; description: string; hidden: boolean }

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
  crit: boolean;
  combo: number;
  combo_mult: number;
}

export interface QuestDetail {
  quest: Quest;
  lesson: string;
  workspace: string | null;
  hints_used: number;
  attempts: number;
  completed: boolean;
  last_check: CheckResult | null;
  has_program: boolean;
}

export interface Home {
  track: Track;
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

export type RunStream = "stdout" | "stderr" | "system" | "build";
export interface RunOutput { quest_id: string; stream: RunStream; line: string }
export interface RunDone { quest_id: string; code: number | null }

export interface EventMap {
  "check:started": CheckStarted;
  "check:result": CheckResultEvent;
  "profile:changed": Profile;
  "focus:tick": FocusTick;
  "focus:done": FocusDone;
  "run:output": RunOutput;
  "run:done": RunDone;
}
