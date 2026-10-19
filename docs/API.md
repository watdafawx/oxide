# UI ↔ core contract

The Svelte app talks to Rust only through these Tauri commands and events. Types mirror
`crates/oxide-core` (`content.rs`, `grader.rs`, `progress.rs`); serde uses the field names below.
Enums serialise as lowercase strings.

## Types (TypeScript)

```ts
type QuestKind = "exercise" | "boss" | "read";

interface Objective { test: string; label: string }
interface Hint { text: string }
interface Quiz { question: string; choices: string[]; answer: number }

interface Quest {
  id: string;            // "01-foundations/03-functions"
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

interface World { id: string; index: number; slug: string; title: string; blurb: string; quests: Quest[] }

interface Settings { editor: string; sound: boolean; focus_minutes: number }

interface Profile {
  xp: number;
  level: number;
  xp_into_level: number;
  xp_for_next: number;
  streak_days: number;       // already adjusted for today (0 if a day was skipped)
  best_streak: number;
  last_active: string | null; // "YYYY-MM-DD"
  achievements: string[];     // ids, see Achievement list
  completed: string[];        // quest ids
  started: string[];          // quest ids started but not completed
  settings: Settings;
}

interface Achievement { id: string; title: string; description: string }

type ObjectiveStatus = "passed" | "failed" | "notrun";
interface ObjectiveResult { test: string; label: string; status: ObjectiveStatus; output: string | null }
interface Diagnostic { level: "error" | "warning"; message: string; rendered: string; file: string | null; line: number | null }
interface CheckResult {
  ok: boolean;
  compiled: boolean;
  objectives: ObjectiveResult[];
  diagnostics: Diagnostic[];
  other_failures: string[];
  raw_tail: string;
  duration_ms: number;
}

interface Completion {
  quest_id: string;
  xp_awarded: number;        // 0 when the quest was already done
  level_before: number;
  level_after: number;
  new_achievements: string[];
  focus_bonus: boolean;
  clean: boolean;
  first_time: boolean;
}

interface QuestDetail {
  quest: Quest;
  lesson: string;             // markdown
  workspace: string | null;   // path once started
  hints_used: number;
  attempts: number;
  completed: boolean;
  last_check: CheckResult | null;
}

interface Home {
  next: Quest | null;         // the one card
  alternate: Quest | null;    // one other available quest
  profile: Profile;
  resume: Quest | null;       // a started-but-unfinished quest, if any
}
```

## Commands (`invoke("name", { ...args })`)

| command | args | returns | notes |
|---|---|---|---|
| `home` | | `Home` | everything the home screen needs in one call |
| `list_worlds` | | `World[]` | for the map |
| `get_quest` | `{ id }` | `QuestDetail` | |
| `start_quest` | `{ id }` | `QuestDetail` | copies starter, records start, opens editor, starts the watcher |
| `open_editor` | `{ id }` | `void` | re-open without resetting |
| `reset_quest` | `{ id }` | `QuestDetail` | wipes the workspace copy, keeps progress |
| `run_checks` | `{ id }` | `CheckResult` | manual run; the watcher also runs on save |
| `answer_quiz` | `{ id, answers: number[] }` | `Completion \| null` | read quests; null when wrong |
| `use_hint` | `{ id }` | `number` | hints now revealed |
| `get_profile` | | `Profile` | |
| `list_achievements` | | `Achievement[]` | static list with titles |
| `start_focus` | `{ id?: string, minutes }` | `number` | session id; engine emits `focus:tick` |
| `stop_focus` | `{ completed: boolean }` | `void` | |
| `get_settings` / `set_settings` | `{ settings }` | `Settings` | |
| `active_dates` | | `string[]` | `YYYY-MM-DD`, for the streak calendar |

Errors: commands reject with a string message. Show it in a toast, no stack traces.

## Events (`listen("name", e => e.payload)`)

| event | payload | when |
|---|---|---|
| `check:started` | `{ quest_id }` | watcher saw a save, cargo is running |
| `check:result` | `{ quest_id, result: CheckResult, completion: Completion \| null }` | cargo finished; `completion` set the first time it goes green |
| `profile:changed` | `Profile` | after any XP/streak/achievement change |
| `focus:tick` | `{ remaining_s, total_s, quest_id }` | once a second while a focus timer runs |
| `focus:done` | `{ quest_id }` | timer hit zero |

## Dev without Tauri

`pnpm dev` in a plain browser has no `window.__TAURI__`. The UI wraps `invoke`/`listen` in
`src/lib/api.ts`; when Tauri is absent it uses `src/lib/mock.ts` with a fake catalog and a
fake check that flips objectives green two seconds after `run_checks`. Build all screens against
the mock first.
