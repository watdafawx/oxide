# UI ↔ core contract

The Svelte app talks to Rust only through these Tauri commands and events. Types mirror
`crates/oxide-core` (`content.rs`, `grader.rs`, `progress.rs`); serde uses the field names below.
Enums serialise as lowercase strings.

## Types (TypeScript)

```ts
type QuestKind = "exercise" | "boss" | "read";
type Runner = "cargo" | "lua" | "gradle";

interface Track { id: string; title: string; blurb: string; runner: Runner }

interface Objective { test: string; label: string }
interface Hint { text: string }
interface Quiz { question: string; choices: string[]; answer: number }

interface Quest {
  id: string;            // "rust/01-foundations/03-functions" (track/world/quest)
  flavour: string;       // "plain" | "games" | ...: the story this value tells (settings.flavour, or plain when the quest has no such flavour)
  track_id: string;
  world_id: string;      // "rust/01-foundations"
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
  points: Point[];       // optional micro-interactions for the Learn stage; empty = plain cards
}

// After lesson point `after` (1-based count of idea paragraphs/bullets; 0 = before the first) the learner must
// pick the right token (`pick` + `answer`) or type `blank` (shown inside `code` where `___` is) before Next unlocks.
interface Point { after: number; prompt: string; pick: string[]; answer: number | null; blank: string | null; code: string | null }

interface World { id: string; track_id: string; index: number; slug: string; title: string; blurb: string; quests: Quest[] }

interface FileEntry { path: string; bytes: number; read_only: boolean; primary: boolean }  // path uses "/"; read_only = the spec (tests); primary = open this one first

interface Settings {
  editor: string;    // external editor command ("code", "zed"); "" (default) = in-app editor only, start_quest opens nothing
  sound: boolean; focus_minutes: number;
  track: string;     // active track id, "" = first
  flavour: string;   // "games" (default) | "plain" | any name from list_flavours
  volume: number;    // 0-1, default 0.6; master volume for the synthesized sounds
  sounds: { objective: boolean; clear: boolean; level: boolean; achievement: boolean; crit: boolean }; // per-sound toggles, default all true; `sound` stays the master switch
  look: string;      // "workbench" (default) | "notebook" | "arcade": app layout and colors
  complete: boolean; // editor autocomplete, default true
  explain: boolean;  // plain-English note under compiler errors, default true
}

interface Profile {
  xp: number;
  level: number;
  xp_into_level: number;
  xp_for_next: number;
  streak_days: number;       // already adjusted for today (0 if a day was skipped)
  best_streak: number;
  last_active: string | null; // "YYYY-MM-DD"
  combo: number;             // consecutive first-time clears without a hint or reset; next clear gets x(1 + 0.05 * min(combo, 10)) XP
  freezes: number;           // streak freezes in stock (0-2), already minus those a pending skipped day will use; +1 every 7 streak days, a skipped day consumes one instead of resetting the streak
  achievements: string[];     // ids, see Achievement list
  completed: string[];        // quest ids
  started: string[];          // quest ids started but not completed
  settings: Settings;
}

interface Achievement { id: string; title: string; description: string; hidden: boolean }  // hidden: show "???" until the id is in profile.achievements

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
  crit: boolean;             // critical clear (12% on a first-time clear): xp_awarded is doubled
  combo: number;             // profile.combo after this clear (0 if a hint had been used)
  combo_mult: number;        // XP multiplier the combo gave this clear, 1 = none
}

interface QuestDetail {
  quest: Quest;
  lesson: string;             // markdown
  workspace: string | null;   // path once started
  hints_used: number;
  attempts: number;
  completed: boolean;
  last_check: CheckResult | null;
  has_program: boolean;       // the workspace has a binary (src/main.rs or [[bin]]) or src/main.lua: Run is possible
}

interface Home {
  track: Track;               // the active track (settings.track)
  next: Quest | null;         // the one card, within the active track
  alternate: Quest | null;    // one other available quest
  profile: Profile;
  resume: Quest | null;       // a started-but-unfinished quest, if any
}
```

## Commands (`invoke("name", { ...args })`)

| command | args | returns | notes |
|---|---|---|---|
| `home` | | `Home` | everything the home screen needs in one call, for the active track |
| `list_tracks` | | `Track[]` | for the track pill |
| `list_flavours` | | `string[]` | `["plain", "games", ...]` for the flavour switch; every quest-returning command already applies `settings.flavour` |
| `list_worlds` | `{ track?: string }` | `World[]` | for the map; omitted = active track |
| `get_quest` | `{ id }` | `QuestDetail` | |
| `start_quest` | `{ id }` | `QuestDetail` | copies starter, records start, opens editor, starts the watcher |
| `open_editor` | `{ id }` | `void` | open the external editor; errors when `settings.editor` is empty |
| `list_files` | `{ id }` | `FileEntry[]` | the started workspace's files (no target/build/.gradle), specs read-only, one `primary` |
| `read_file` | `{ id, path }` | `string` | file text (≤ 512 KB) |
| `write_file` | `{ id, path, text }` | `void` | save from the in-app editor; the watcher re-grades like any save; refuses read-only paths and paths outside the workspace |
| `reset_quest` | `{ id }` | `QuestDetail` | wipes the workspace copy, keeps progress; resets `combo` (a hint on an uncleared quest does too) |
| `run_checks` | `{ id }` | `CheckResult` | manual run; the watcher also runs on save |
| `answer_quiz` | `{ id, answers: number[] }` | `Completion \| null` | read quests; null when wrong |
| `use_hint` | `{ id }` | `number` | hints now revealed |
| `read_point` | `{ id, index }` | `number` | XP credited, once per quest+index (0 when already read; does not start the quest). `index` 0-99 = Learn card (`i`) or active point (`50 + i`), +2 XP; 100-199 = Build task (`100 + task`), +1 XP; 200+ = Check stage, +5 XP. Emits `profile:changed` when XP was credited |
| `get_profile` | | `Profile` | |
| `list_achievements` | | `Achievement[]` | static list with titles; `hidden` ones (early-bird, night-owl, one-save, comeback, crit-first) are shown as "???" until earned |
| `start_focus` | `{ id?: string, minutes }` | `number` | session id; engine emits `focus:tick` |
| `stop_focus` | `{ completed: boolean }` | `void` | |
| `run_program` | `{ id }` | `void` | runs the project (`cargo run`, or `src/main.lua` in-process); output streams as `run:output`; stops a previous run first |
| `run_input` | `{ text }` | `void` | one line to the running program's stdin |
| `stop_program` | | `void` | kills the running program |
| `get_settings` / `set_settings` | `{ settings }` | `Settings` | switching track = `set_settings` with a new `track`, then reload `home` |
| `active_dates` | | `string[]` | `YYYY-MM-DD`, for the streak calendar |
| `list_profiles` | | `ProfileInfo[]` | `{ id, name, created, active }`. Each profile has its own progress database and quests folder. Saves live in AppData, or in `data\` next to oxide.exe for the portable zip; a save from before profiles becomes "Player", its files left where they were |
| `create_profile` | `{ name }` | `ProfileInfo[]` | name 1-40 characters |
| `rename_profile` | `{ id, name }` | `ProfileInfo[]` | |
| `delete_profile` | `{ id }` | `ProfileInfo[]` | deletes the profile's folder; refuses the active profile |
| `switch_profile` | `{ id }` | `Profile` | stops focus, a running program and the watcher, then swaps progress and quests folder; emits `profile:changed`. The UI reloads after it |
| `export_profile` | `{ id }` | `string` | path of the `.oxide-profile.zip` (Documents\Oxide exports, or `exports\` next to the portable exe), shown in Explorer. Holds a progress snapshot and quest code, no build output |
| `import_profile` | `{ data }` | `ProfileInfo[]` | `data` = the zip as base64; added as a new profile ("Name (2)" when taken) |

Errors: commands reject with a string message. Show it in a toast, no stack traces.

## Events (`listen("name", e => e.payload)`)

| event | payload | when |
|---|---|---|
| `check:started` | `{ quest_id }` | watcher saw a save, cargo is running |
| `check:result` | `{ quest_id, result: CheckResult, completion: Completion \| null }` | cargo finished; `completion` set the first time it goes green |
| `profile:changed` | `Profile` | after any XP/streak/achievement change |
| `focus:tick` | `{ remaining_s, total_s, quest_id }` | once a second while a focus timer runs |
| `focus:done` | `{ quest_id }` | timer hit zero |
| `run:output` | `{ quest_id, stream: "stdout" \| "stderr" \| "build" \| "system", line }` | one line of program output (`system` = the command itself; `build` = cargo/rustc output before the program's first stdout line, fold it) |
| `run:done` | `{ quest_id, code: number \| null }` | the program exited |

## Dev without Tauri

`pnpm dev` in a plain browser has no `window.__TAURI__`. The UI wraps `invoke`/`listen` in
`src/lib/api.ts`; when Tauri is absent it uses `src/lib/mock.ts` with a fake catalog and a
fake check that flips objectives green two seconds after `run_checks`. Build all screens against
the mock first.

## Rewards ("juice")

XP for a first-time clear = `quest.xp` x combo multiplier x 1.5 (focus) x 1.1 (clean) x 2 (crit), rounded.
The combo multiplier uses the combo *before* the clear. Repeat clears give 0 XP and never crit.
Sounds are synthesized in `src/lib/sound.ts` and gated by `settings.sound` (master), `settings.sounds.*` and `settings.volume`.
When `check:result` arrives the UI ticks objectives over one at a time (120 ms apart, one scale note each), then plays the verdict
(chord on green, a soft click on red), then shows the XP toast (`CRIT! 2x` when `completion.crit`) and queues achievement toasts (2.5 s each).
