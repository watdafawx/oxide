// Fake backend for `pnpm dev` in a plain browser. Exercises every command and event.
import type { MockHandlers } from "./api";
import type {
  Achievement, CheckResult, Completion, EventMap, ObjectiveResult, Profile, Quest, QuestDetail, Track, World,
} from "./types";
import helloLesson from "../../content/worlds/00-tutorial/01-hello/lesson.md?raw";
import fixLesson from "../../content/worlds/00-tutorial/02-fix-it/lesson.md?raw";
import bossLesson from "../../content/worlds/00-tutorial/03-boss-name-tag/lesson.md?raw";

// ---------- event bus ----------
type Cb = (p: never) => void;
const listeners = new Map<string, Set<Cb>>();
export function mockListen<K extends keyof EventMap>(name: K, cb: (p: EventMap[K]) => void): () => void {
  if (!listeners.has(name)) listeners.set(name, new Set());
  listeners.get(name)!.add(cb as Cb);
  return () => listeners.get(name)?.delete(cb as Cb);
}
function emit<K extends keyof EventMap>(name: K, payload: EventMap[K]) {
  listeners.get(name)?.forEach((cb) => (cb as (p: EventMap[K]) => void)(payload));
}
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

// ---------- catalog ----------
type QuestInit = Partial<Quest> & Pick<Quest, "title" | "summary" | "kind" | "xp" | "minutes">;
function q(world: string, index: number, slug: string, p: QuestInit): Quest {
  const track_id = world.split("/")[0];
  return { id: `${world}/${slug}`, track_id, runner: track_id === "lua" ? "lua" : "cargo", world_id: world, index, slug, tags: [], requires: [], objectives: [], hints: [], quiz: [], ...p };
}
const obj = (test: string, label: string) => ({ test, label });

const tracks: Track[] = [
  { id: "rust", title: "Rust", blurb: "Systems programming with ownership, traits and fearless concurrency.", runner: "cargo" },
  { id: "lua", title: "Lua", blurb: "Small, fast scripting. Great for mods and game logic.", runner: "lua" },
];

const T = "rust/00-tutorial", F = "rust/01-foundations", O = "rust/02-ownership", L = "lua/01-lua-core";
const quests: Quest[] = [
  q(T, 1, "01-hello", { title: "Say hello", summary: "Write one function and watch the tick turn green.", kind: "exercise", xp: 30, minutes: 3,
    tags: ["functions", "strings"], objectives: [obj("greets_by_name", 'greeting("Ferris") returns "Hello, Ferris!"')],
    hints: [{ text: '`format!` builds a String the same way `println!` prints one: `format!("Hi {name}")`.' }, { text: 'Replace `todo!()` with `format!("Hello, {name}!")`.' }] }),
  q(T, 2, "02-fix-it", { title: "Read the red", summary: "A test fails. Read what it says, fix the number.", kind: "exercise", xp: 20, minutes: 2,
    tags: ["tests"], requires: [`${T}/01-hello`], objectives: [obj("answer_is_right", "answer() returns what the test expects")],
    hints: [{ text: "Click the failed objective: the test prints `left` (what you returned) and `right` (what it wanted)." }] }),
  q(T, 3, "03-boss-name-tag", { title: "Boss: name tag", summary: "Two functions, two objectives. Your first boss.", kind: "boss", xp: 100, minutes: 5,
    tags: ["functions", "strings"], requires: [`${T}/02-fix-it`],
    objectives: [obj("shouts_the_name", 'shout("ferris") returns "FERRIS"'), obj("tag_shows_name_and_level", 'name_tag("ferris", 3) returns "[FERRIS] Lv 3"')],
    hints: [{ text: "`str` has a method that returns an uppercase `String`. Type `name.` in your editor and look at the completions." }, { text: '`name_tag` can call `shout`. Numbers go into `format!` just like text: `format!("Lv {level}")`.' }] }),
  q(F, 1, "01-variables", { title: "Variables and mutability", summary: "Why `let` is immutable by default, and when to say `mut`.", kind: "read", xp: 25, minutes: 4,
    tags: ["let", "mut"], requires: [`${T}/03-boss-name-tag`],
    quiz: [
      { question: "What happens if you assign twice to `let x = 1;`?", choices: ["It works silently", "Compile error: cannot assign twice", "x becomes 2 at runtime only", "It shadows x"], answer: 1 },
      { question: "Which keyword makes a binding changeable?", choices: ["var", "mut", "change", "dyn"], answer: 1 },
    ] }),
  q(F, 2, "02-functions", { title: "Functions with return values", summary: "Last expression, no semicolon, that's the return.", kind: "exercise", xp: 40, minutes: 6,
    tags: ["functions"], requires: [`${F}/01-variables`],
    objectives: [obj("double_works", "double(21) returns 42"), obj("clamp_works", "clamp(15, 0, 10) returns 10"), obj("sign_works", "sign(-4) returns -1")],
    hints: [{ text: "No semicolon on the last line means it is returned." }, { text: "`n.clamp(lo, hi)` exists on integers." }, { text: "`n.signum()` gives -1, 0 or 1." }] }),
  q(F, 3, "03-boss-calc", { title: "Boss: tiny calculator", summary: "Parse, match, compute. Everything from this world.", kind: "boss", xp: 120, minutes: 12,
    tags: ["match", "enums"], requires: [`${F}/02-functions`],
    objectives: [obj("parses_numbers", 'parse("1 + 2") returns Ok(3)'), obj("rejects_garbage", 'parse("x") returns Err'), obj("division_by_zero", "1 / 0 is an Err, not a panic")],
    hints: [{ text: "Split on whitespace, then `match` the operator." }] }),
  q(O, 1, "01-moves", { title: "Who owns this?", summary: "Moves: why `let b = a;` can make `a` unusable.", kind: "exercise", xp: 50, minutes: 8,
    tags: ["ownership"], requires: [`${F}/03-boss-calc`],
    objectives: [obj("no_use_after_move", "program compiles without cloning twice"), obj("prints_both", "both names print the same text")],
    hints: [{ text: "Borrow with `&` instead of moving." }, { text: "`.clone()` copies, but you only need it once." }] }),
  q(O, 2, "02-borrowing", { title: "Borrow it, don't take it", summary: "Shared and mutable references, and the one rule.", kind: "read", xp: 30, minutes: 5,
    tags: ["borrowing"], requires: [`${O}/01-moves`],
    quiz: [{ question: "How many `&mut` to the same value at once?", choices: ["Zero", "One", "Two", "As many as you like"], answer: 1 }] }),
  q(O, 3, "03-boss-stack", { title: "Boss: build a stack", summary: "A `Stack<T>` with push, pop and peek, no cloning.", kind: "boss", xp: 150, minutes: 20,
    tags: ["structs", "ownership"], requires: [`${O}/02-borrowing`],
    objectives: [obj("push_pop", "push then pop returns the item"), obj("peek_borrows", "peek returns a reference"), obj("empty_pop", "pop on empty is None")],
    hints: [{ text: "`Vec` already does the hard part." }] }),
  q(L, 1, "01-hello-lua", { title: "Print and return", summary: "Your first Lua function: build a string and return it.", kind: "exercise", xp: 30, minutes: 3,
    tags: ["functions", "strings"], objectives: [obj("greets", 'greet("Ada") returns "Hello, Ada!"')],
    hints: [{ text: "Lua joins strings with `..`: `\"Hi \" .. name`." }] }),
  q(L, 2, "02-tables", { title: "Tables are everything", summary: "Arrays and dictionaries are the same thing in Lua.", kind: "exercise", xp: 40, minutes: 6,
    tags: ["tables"], requires: [`${L}/01-hello-lua`],
    objectives: [obj("sum_works", "sum({1, 2, 3}) returns 6"), obj("keys_work", "count_keys({a = 1, b = 2}) returns 2")],
    hints: [{ text: "`#t` is the length of an array-style table." }, { text: "Loop with `for k, v in pairs(t) do`." }] }),
  q(L, 3, "03-arrays", { title: "Arrays start at 1", summary: "Index from one, map and filter by hand.", kind: "exercise", xp: 40, minutes: 8,
    tags: ["tables", "loops"], requires: [`${L}/02-tables`, `${F}/02-functions`],
    objectives: [obj("map_works", "map(double, {1, 2}) returns {2, 4}"), obj("filter_works", "filter(is_even, {1, 2, 3, 4}) returns {2, 4}")],
    hints: [{ text: "`table.insert(out, v)` appends." }] }),
];

const lessons: Record<string, string> = {
  [`${T}/01-hello`]: helloLesson,
  [`${T}/02-fix-it`]: fixLesson,
  [`${T}/03-boss-name-tag`]: bossLesson,
};
const fallbackLesson = (qq: Quest) => qq.runner === "lua" ? `${qq.summary}

\`\`\`lua
local name = "Ada"
print("Hello, " .. name .. "!")
\`\`\`

## Your turn

- Open \`src/solution.lua\`.
- \`tests.lua\` is the spec.
- Make the objectives on the right go green.
` : `${qq.summary}

\`\`\`rust
fn main() {
    let name = "Ferris";
    println!("Hello, {name}!");
}
\`\`\`

## Your turn

- Open \`src/lib.rs\`.
- Make the objectives on the right go green.
- Save. Watch the tick.
`;

const worlds: World[] = [
  { id: T, track_id: "rust", index: 0, slug: "tutorial", title: "Tutorial", blurb: "How Oxide works in ten minutes.", quests: [] },
  { id: F, track_id: "rust", index: 1, slug: "foundations", title: "Foundations", blurb: "Variables, functions, control flow.", quests: [] },
  { id: O, track_id: "rust", index: 2, slug: "ownership", title: "Ownership", blurb: "The idea that makes Rust Rust.", quests: [] },
  { id: L, track_id: "lua", index: 1, slug: "lua-core", title: "Lua Core", blurb: "Functions, tables and loops.", quests: [] },
];
worlds.forEach((w) => (w.quests = quests.filter((x) => x.world_id === w.id)));
const byId = (id: string) => {
  const x = quests.find((z) => z.id === id);
  if (!x) throw new Error(`unknown quest ${id}`);
  return x;
};

const achievements: Achievement[] = [
  { id: "first-quest", title: "First Spark", description: "Complete your first quest." },
  { id: "level-2", title: "Level 2", description: "Reach level 2." },
  { id: "boss-slayer", title: "Boss Slayer", description: "Defeat a world boss." },
  { id: "clean-run", title: "Clean Compile", description: "Clear a quest without a compiler error." },
  { id: "streak-3", title: "On a Roll", description: "Keep a 3-day streak." },
  { id: "streak-7", title: "Week of Rust", description: "Keep a 7-day streak." },
  { id: "focus-1", title: "Deep Focus", description: "Finish a focus timer." },
  { id: "no-hints", title: "No Peeking", description: "Clear a quest without using a hint." },
];

// ---------- state ----------
const iso = (d: Date) => d.toISOString().slice(0, 10);
function mkDates(): string[] {
  const out: string[] = [];
  const now = new Date();
  for (let i = 0; i < 84; i++) {
    const d = new Date(now.getTime() - i * 86400000);
    if (i < 3 || (i * 7919) % 5 < 2) out.push(iso(d));
  }
  return out;
}
const dates = mkDates();

const state = {
  xp: 0,
  completed: [] as string[],
  started: [] as string[],
  achievements: ["streak-3"] as string[],
  settings: { editor: "code", sound: true, focus_minutes: 25, track: "rust" },
  hints: {} as Record<string, number>,
  attempts: {} as Record<string, number>,
  last: {} as Record<string, CheckResult | null>,
  focusTimer: null as ReturnType<typeof setInterval> | null,
};

const levelStart = (lv: number) => 25 * lv * (lv - 1); // L1 0, L2 50, L3 150, L4 300
function profile(): Profile {
  const xp = state.xp;
  let level = 1;
  while (xp >= levelStart(level + 1)) level++;
  return {
    xp, level, xp_into_level: xp - levelStart(level), xp_for_next: levelStart(level + 1) - levelStart(level),
    streak_days: 3, best_streak: 7, last_active: iso(new Date()),
    achievements: [...state.achievements], completed: [...state.completed], started: [...state.started], settings: { ...state.settings },
  };
}
const unlocked = (qq: Quest) => qq.requires.every((r) => state.completed.includes(r));
const pushProfile = () => setTimeout(() => emit("profile:changed", profile()), 30);

function detail(id: string): QuestDetail {
  const qq = byId(id);
  const started = state.started.includes(id) || state.completed.includes(id);
  return {
    quest: qq, lesson: lessons[id] ?? fallbackLesson(qq),
    workspace: started ? `C:\\Users\\you\\oxide-quests\\${id.replaceAll("/", "\\")}` : null,
    hints_used: state.hints[id] ?? 0, attempts: state.attempts[id] ?? 0,
    completed: state.completed.includes(id), last_check: state.last[id] ?? null,
  };
}

function complete(qq: Quest, clean: boolean): Completion {
  const before = profile();
  const first = !state.completed.includes(qq.id);
  const newA: string[] = [];
  if (first) {
    state.completed.push(qq.id);
    state.started = state.started.filter((s) => s !== qq.id);
    state.xp += qq.xp;
    const add = (id: string) => { if (!state.achievements.includes(id)) { state.achievements.push(id); newA.push(id); } };
    add("first-quest");
    if (qq.kind === "boss") add("boss-slayer");
    if (clean) add("clean-run");
    if (!state.hints[qq.id]) add("no-hints");
    if (profile().level >= 2) add("level-2");
  }
  const after = profile();
  pushProfile();
  return {
    quest_id: qq.id, xp_awarded: first ? qq.xp : 0, level_before: before.level, level_after: after.level,
    new_achievements: newA, focus_bonus: false, clean, first_time: first,
  };
}

// ---------- fake grading ----------
type Mode = "fail" | "compile" | "partial" | "pass";
function grade(qq: Quest, mode: Mode): CheckResult {
  const dur = 900 + Math.round(Math.random() * 1400);
  if (mode === "compile" && qq.runner === "lua") {
    return {
      ok: false, compiled: false, duration_ms: dur, other_failures: [], raw_tail: "lua: error",
      objectives: qq.objectives.map((o) => ({ ...o, status: "notrun", output: null })),
      diagnostics: [{
        level: "error", message: "attempt to concatenate a nil value (local 'name')", file: "src/solution.lua", line: 3,
        rendered: "lua: src/solution.lua:3: attempt to concatenate a nil value (local 'name')\nstack traceback:\n\tsrc/solution.lua:3: in function 'greet'\n\ttests.lua:5: in main chunk",
      }],
    };
  }
  if (mode === "compile") {
    return {
      ok: false, compiled: false, duration_ms: dur, other_failures: [], raw_tail: "error: could not compile",
      objectives: qq.objectives.map((o) => ({ ...o, status: "notrun", output: null })),
      diagnostics: [{
        level: "error", message: "mismatched types", file: "src/lib.rs", line: 4,
        rendered: "error[E0308]: mismatched types\n --> src/lib.rs:4:5\n  |\n3 | pub fn greeting(name: &str) -> String {\n  |                                ------ expected `String` because of return type\n4 |     name\n  |     ^^^^ expected `String`, found `&str`\n  |\nhelp: try using a conversion method: `.to_string()`",
      }, {
        level: "warning", message: "unused variable: `x`", file: "src/lib.rs", line: 9,
        rendered: "warning: unused variable: `x`\n --> src/lib.rs:9:9\n  |\n9 |     let x = 5;\n  |         ^ help: if this is intentional, prefix it with an underscore: `_x`",
      }],
    };
  }
  const objectives: ObjectiveResult[] = qq.objectives.map((o, i) => {
    const pass = mode === "pass" || (mode === "partial" && i === 0);
    return {
      ...o, status: pass ? "passed" : "failed",
      output: pass ? null : `thread '${o.test}' panicked at src/lib.rs:12:5:\nassertion \`left == right\` failed\n  left: 41\n right: 42\nnote: run with \`RUST_BACKTRACE=1\``,
    };
  });
  return { ok: mode === "pass", compiled: true, objectives, diagnostics: [], other_failures: [], raw_tail: "test result", duration_ms: dur };
}

let busy = false;
async function runCycle(id: string): Promise<CheckResult> {
  const qq = byId(id);
  const n = (state.attempts[id] = (state.attempts[id] ?? 0) + 1);
  // run 1: compile error, run 2: partial (multi-objective only), then green
  const mode: Mode = n === 1 ? "compile" : n === 2 && qq.objectives.length > 1 ? "partial" : "pass";
  emit("check:started", { quest_id: id });
  busy = true;
  await sleep(2000);
  busy = false;
  const result = grade(qq, mode);
  state.last[id] = result;
  const completion = result.ok ? complete(qq, n === 1) : null;
  emit("check:result", { quest_id: id, result, completion });
  return result;
}

// ---------- handlers ----------
export const mockHandlers: MockHandlers = {
  async home() {
    const track = tracks.find((t) => t.id === state.settings.track) ?? tracks[0];
    const mine = quests.filter((x) => x.track_id === track.id);
    const open = mine.filter((x) => !state.completed.includes(x.id) && unlocked(x));
    const started = mine.find((x) => state.started.includes(x.id)) ?? null;
    const next = open.find((x) => x.id !== started?.id) ?? null;
    const alt = open.filter((x) => x.id !== next?.id && x.id !== started?.id)[0] ?? null;
    return { track, next: next ?? started, alternate: alt, profile: profile(), resume: next ? started : null };
  },
  async list_tracks() { return tracks; },
  async list_worlds({ track }) {
    const t = track || state.settings.track || tracks[0].id;
    return worlds.filter((w) => w.track_id === t);
  },
  async get_quest({ id }) { return detail(id); },
  async start_quest({ id }) {
    const qq = byId(id);
    if (!state.started.includes(id) && !state.completed.includes(id)) state.started.push(id);
    pushProfile();
    if (qq.kind !== "read" && !state.last[id]) {
      // the engine runs the starter once: everything red
      emit("check:started", { quest_id: id });
      setTimeout(() => {
        const result = grade(qq, "fail");
        state.last[id] = result;
        emit("check:result", { quest_id: id, result, completion: null });
      }, 2000);
    }
    return detail(id);
  },
  async open_editor() { console.info("[mock] open_editor"); },
  async reset_quest({ id }) { state.last[id] = null; state.attempts[id] = 0; return detail(id); },
  async run_checks({ id }) {
    if (busy) throw new Error("A check is already running.");
    return runCycle(id);
  },
  async answer_quiz({ id, answers }) {
    const qq = byId(id);
    const ok = qq.quiz.length > 0 && qq.quiz.every((z, i) => answers[i] === z.answer);
    return ok ? complete(qq, true) : null;
  },
  async use_hint({ id }) {
    const qq = byId(id);
    state.hints[id] = Math.min((state.hints[id] ?? 0) + 1, qq.hints.length);
    return state.hints[id];
  },
  async get_profile() { return profile(); },
  async list_achievements() { return achievements; },
  async start_focus({ id, minutes }) {
    if (state.focusTimer) clearInterval(state.focusTimer);
    const total = minutes * 60;
    let remaining = total;
    const qid = id ?? "";
    emit("focus:tick", { remaining_s: remaining, total_s: total, quest_id: qid });
    state.focusTimer = setInterval(() => {
      remaining--;
      emit("focus:tick", { remaining_s: remaining, total_s: total, quest_id: qid });
      if (remaining <= 0) {
        clearInterval(state.focusTimer!);
        state.focusTimer = null;
        emit("focus:done", { quest_id: qid });
      }
    }, 1000);
    return 1;
  },
  async stop_focus() { if (state.focusTimer) clearInterval(state.focusTimer); state.focusTimer = null; },
  async get_settings() { return { ...state.settings }; },
  async set_settings({ settings }) { state.settings = { ...settings }; pushProfile(); return { ...state.settings }; },
  async active_dates() { return dates; },
};
