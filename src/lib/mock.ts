// Fake backend for `pnpm dev` in a plain browser. Exercises every command and event.
import type { MockHandlers } from "./api";
import type { ProfileInfo,
  Achievement, CheckResult, Settings, Completion, EventMap, ObjectiveResult, Profile, Quest, QuestDetail, Track, World,
} from "./types";
import helloLesson from "../../content/tracks/rust/worlds/00-tutorial/01-hello/lesson.md?raw";
import fixLesson from "../../content/tracks/rust/worlds/00-tutorial/02-fix-it/lesson.md?raw";
import starterRs from "../../content/tracks/rust/worlds/00-tutorial/01-hello/starter/src/lib.rs?raw";
import bossLesson from "../../content/tracks/rust/worlds/00-tutorial/03-boss-name-tag/lesson.md?raw";

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
  return { id: `${world}/${slug}`, track_id, flavour: "games", runner: track_id === "lua" || track_id === "basics" ? "lua" : "cargo", world_id: world, index, slug, tags: [], requires: [], objectives: [], hints: [], quiz: [], points: [], ...p };
}
const obj = (test: string, label: string) => ({ test, label });

const tracks: Track[] = [
  { id: "basics", title: "Programming 101", blurb: "Never programmed before? Start here. Optional: every quest is open, skip what you know.", runner: "lua" },
  { id: "rust", title: "Rust", blurb: "Systems programming with ownership, traits and fearless concurrency.", runner: "cargo" },
  { id: "lua", title: "Lua", blurb: "Small, fast scripting. Great for mods and game logic.", runner: "lua" },
  { id: "gpui", title: "System monitor app", blurb: "Build a Windows system monitor with GPUI: live CPU and GPU stats on the taskbar, a desktop widget, an FPS overlay for games.", runner: "cargo", parent: "rust" },
];

const B = "basics/01-what-a-program-is";
const T = "rust/00-tutorial", F = "rust/01-foundations", O = "rust/02-ownership", L = "lua/01-lua-core";
const C = "rust/03-collections", E = "rust/04-errors", TR = "rust/05-traits", SP = "rust/06-smart-pointers";
const quests: Quest[] = [
  q("gpui/00-setup", 1, "01-what-gpui-is", { track_id: "gpui", title: "What GPUI is", summary: "Zed's UI framework.", kind: "read", xp: 15, minutes: 3,
    requires: [`${TR}/01-entities`] }),
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
  q(C, 1, "01-inventory", { title: "Fill the inventory", summary: "A Vec of item stacks, counted with a HashMap.", kind: "exercise", xp: 60, minutes: 10,
    tags: ["vec", "hashmap"], requires: [`${O}/03-boss-stack`],
    objectives: [obj("add_item", "add() puts a stack in the first free slot"), obj("count_items", "count() totals one item across stacks"), obj("sort_by_rarity", "sorted by rarity, rarest first"), obj("loot_roll", "a loot table roll lands in the inventory")] }),
  q(E, 1, "01-load-manifest", { title: "Load the manifest", summary: "Parse a mod list with ? and a ManifestError.", kind: "exercise", xp: 60, minutes: 10,
    tags: ["result", "errors"], requires: [`${C}/01-inventory`],
    objectives: [obj("parses_line", "a good line gives a Mod"), obj("bad_version", "a bad version is a ManifestError"), obj("missing_name", "a missing name is reported, not a panic"), obj("loads_all", "load() returns every mod"), obj("reports_first_error", "load() stops at the first error")] }),
  q(TR, 1, "01-entities", { title: "Spawn some mobs", summary: "An Entity trait with a zombie, a creeper and more.", kind: "exercise", xp: 70, minutes: 12,
    tags: ["traits"], requires: [`${E}/01-load-manifest`],
    objectives: [obj("zombie_hits", "Zombie deals 3 damage"), obj("creeper_blows", "Creeper explodes once"), obj("villager_trades", "Villager never attacks"), obj("skeleton_shoots", "Skeleton shoots from range"), obj("pig_oinks", "Pig has 10 health")] }),
  q(SP, 1, "01-machines", { title: "Link the machines", summary: "Machines share belts through Rc<RefCell<Machine>>.", kind: "exercise", xp: 80, minutes: 15,
    tags: ["rc", "refcell"], requires: [`${TR}/01-entities`],
    objectives: [obj("miner_outputs", "the miner pushes ore onto its belt"), obj("belt_links", "a belt links two machines"), obj("smelter_runs", "the smelter turns ore into plates"), obj("back_link_weak", "back-links are Weak, no cycle leak"), obj("assembler_runs", "the assembler builds from two inputs"), obj("throughput", "the line moves 12 items per second")] }),
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
  q(B, 1, "01-instructions-in-order", { title: "Instructions in order", summary: "A program is text the computer follows, line by line, exactly as written.", kind: "read", xp: 15, minutes: 3,
    quiz: [{ question: "What is source code?", choices: ["what the program shows on the screen", "a secret key", "the instructions, written as text in a file"], answer: 2 }] }),
  q(B, 2, "02-say-hello", { title: "Say hello", summary: "Your first program: two lines on the shop's screen.", kind: "exercise", xp: 30, minutes: 3,
    tags: ["print"], objectives: [obj("greets_first", "The first line is: Welcome to the bike shop!"), obj("shows_the_hours_second", "The second line is: Open 9 to 5")] }),
];

// the tutorial files predate the Why-now opener; give the mock one so the layout is visible
const why = (line: string, body: string) => (body.startsWith("**Why now") ? body : `**Why now:** ${line}

${body}`);
const lessons: Record<string, string> = {
  [`${T}/01-hello`]: why("every program is built from small functions, and this one proves your edit, save, green loop works.", helloLesson),
  [`${T}/02-fix-it`]: why("red tests are the normal state of programming; reading them calmly is the skill.", fixLesson),
  [`${T}/03-boss-name-tag`]: why("bosses combine the small pieces you already own.", bossLesson),
};
const fallbackLesson = (qq: Quest) => qq.runner === "lua" ? `**Why now:** ${qq.summary}

Lua builds strings with \`..\`, and a function hands a value back with \`return\`.

\`\`\`lua
local name = "Ada"
print("Hello, " .. name .. "!")
\`\`\`

## Your turn

- Open \`src/solution.lua\`.
- \`tests.lua\` is the spec.
- Make the objectives on the right go green.
` : `**Why now:** ${qq.summary}

\`\`\`rust
fn main() {
    let name = "Ferris";
    println!("Hello, {name}!");
}
\`\`\`

- \`let\` binds a name to a value.
- \`println!\` prints a line; \`{name}\` splices the variable in.
- The last expression of a function, without \`;\`, is what it returns.

A second block shows the other half:

\`\`\`rust
fn shout(s: &str) -> String {
    s.to_uppercase()
}
\`\`\`

## Your turn

- Open \`src/lib.rs\`.
- Make the objectives on the right go green.
- Save. Watch the tick.
`;

const worlds: World[] = [
  { id: B, track_id: "basics", index: 1, slug: "what-a-program-is", title: "What a program is", blurb: "Write a first program, run it, read its errors.", quests: [] },
  { id: T, track_id: "rust", index: 0, slug: "tutorial", title: "Tutorial", blurb: "How Oxide works in ten minutes.", quests: [] },
  { id: F, track_id: "rust", index: 1, slug: "foundations", title: "Foundations", blurb: "Variables, functions, control flow.", quests: [] },
  { id: O, track_id: "rust", index: 2, slug: "ownership", title: "Ownership", blurb: "The idea that makes Rust Rust.", quests: [] },
  { id: C, track_id: "rust", index: 3, slug: "collections", title: "Collections", blurb: "Inventory and loot.", quests: [] },
  { id: E, track_id: "rust", index: 4, slug: "errors", title: "Errors and Modules", blurb: "A modpack manifest loader.", quests: [] },
  { id: TR, track_id: "rust", index: 5, slug: "traits", title: "Traits and Generics", blurb: "An entity system.", quests: [] },
  { id: SP, track_id: "rust", index: 6, slug: "smart-pointers", title: "Smart Pointers", blurb: "A machine graph.", quests: [] },
  { id: "gpui/00-setup", track_id: "gpui", index: 0, slug: "setup", title: "Setup", blurb: "A GPUI window.", quests: [] },
  { id: L, track_id: "lua", index: 1, slug: "lua-core", title: "Lua Core", blurb: "Functions, tables and loops.", quests: [] },
];
worlds.forEach((w) => (w.quests = quests.filter((x) => x.world_id === w.id)));
// "plain" flavour: same quest, no game story
const plainTitles: Record<string, string> = { [`${T}/01-hello`]: "Write your first function" };
const skin = (x: Quest): Quest => (state.settings.flavour === "plain"
  ? { ...x, flavour: "plain", title: plainTitles[x.id] ?? x.title }
  : { ...x, flavour: "games" });
const byId = (id: string) => {
  const x = quests.find((z) => z.id === id);
  if (!x) throw new Error(`unknown quest ${id}`);
  return skin(x);
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
  { id: "early-bird", title: "Early bird", description: "Clear a quest before 07:00" },
  { id: "night-owl", title: "Night owl", description: "Clear a quest after 23:00" },
  { id: "one-save", title: "One save", description: "Go green on your first save" },
  { id: "comeback", title: "Comeback", description: "Clear a quest after 7+ days away" },
  { id: "crit-first", title: "Lucky strike", description: "Land your first critical clear" },
].map((a) => ({ ...a, hidden: ["early-bird", "night-owl", "one-save", "comeback", "crit-first"].includes(a.id) }));

// ---------- state ----------
/** dev-console knobs: `__oxideMock.forceCrit = true` makes the next first-time clear a crit */
const dbg = { forceCrit: false, get state() { return state; } };
if (typeof window !== "undefined") (window as unknown as { __oxideMock: unknown }).__oxideMock = dbg;
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

let mockProfiles: ProfileInfo[] = [{ id: "player", name: "Player", created: "2026-10-19", active: true }];

const state = {
  xp: 0,
  completed: [] as string[],
  started: [] as string[],
  achievements: ["streak-3"] as string[],
  settings: {
    editor: "", sound: true, focus_minutes: 25, track: "rust", flavour: "games", volume: 0.6,
    sounds: { objective: true, clear: true, level: true, achievement: true, crit: true },
    look: "workbench", complete: true, explain: true,
  } as Settings,
  combo: 0,
  freezes: 1,
  lastActive: null as string | null, // null = today
  hints: {} as Record<string, number>,
  points: new Set<string>(),
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
    streak_days: 3, best_streak: 7, last_active: state.lastActive ?? iso(new Date()), combo: state.combo, freezes: state.freezes,
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
    has_program: qq.kind === "boss",
  };
}

function complete(qq: Quest, clean: boolean): Completion {
  const before = profile();
  const first = !state.completed.includes(qq.id);
  const newA: string[] = [];
  let crit = false, comboMult = 1, xp = 0;
  if (first) {
    state.completed.push(qq.id);
    state.started = state.started.filter((s) => s !== qq.id);
    const hour = new Date().getHours();
    crit = dbg.forceCrit || Math.random() < 0.12;
    comboMult = 1 + 0.05 * Math.min(state.combo, 10);
    xp = Math.round(qq.xp * comboMult * (crit ? 2 : 1));
    state.combo = state.hints[qq.id] ? 0 : state.combo + 1;
    state.xp += xp;
    const add = (id: string) => { if (!state.achievements.includes(id)) { state.achievements.push(id); newA.push(id); } };
    add("first-quest");
    if (qq.kind === "boss") add("boss-slayer");
    if (clean) add("clean-run");
    if (!state.hints[qq.id]) add("no-hints");
    if (profile().level >= 2) add("level-2");
    if (hour < 7) add("early-bird");
    if (hour >= 23) add("night-owl");
    if ((state.attempts[qq.id] ?? 0) <= 1 && qq.kind !== "read") add("one-save");
    if (state.lastActive && Date.now() - new Date(state.lastActive).getTime() >= 7 * 86400000) add("comeback");
    if (crit) add("crit-first");
  }
  const after = profile();
  pushProfile();
  return {
    quest_id: qq.id, xp_awarded: first ? xp : 0, level_before: before.level, level_after: after.level,
    new_achievements: newA, focus_bonus: false, clean, first_time: first,
    crit, combo: state.combo, combo_mult: comboMult,
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
    const pass = mode === "pass" || (mode === "partial" && i < Math.ceil(qq.objectives.length / 2));
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

// ---------- fake workspace files ----------
const files: Record<string, Map<string, string>> = {};
const LUA_SRC = 'local M = {}\n\nfunction M.greet(name)\n  return "Hello, " .. name\nend\n\nreturn M\n';
const RS_FALLBACK = "pub fn answer() -> u32 {\n    todo!()\n}\n";
const CARGO = '[package]\nname = "quest"\nversion = "0.1.0"\nedition = "2021"\n';
function filesOf(id: string): Map<string, string> {
  if (!files[id]) {
    files[id] = byId(id).runner === "lua"
      ? new Map([["src/solution.lua", LUA_SRC], ["tests.lua", "-- the spec\n"]])
      : new Map([["src/lib.rs", id.endsWith("01-hello") ? starterRs : RS_FALLBACK], ["Cargo.toml", CARGO], ["tests/spec.rs", "// the spec\n"]]);
  }
  return files[id];
}
const primaryOf = (id: string) => (byId(id).runner === "lua" ? "src/solution.lua" : "src/lib.rs");
const readOnly = (p: string) => p === "tests.lua" || p.startsWith("tests/");
let writeTimer: ReturnType<typeof setTimeout> | null = null;

// ---------- fake program run ----------
const run = { timers: [] as ReturnType<typeof setTimeout>[], id: "", live: false };
function endRun(code: number | null) {
  if (!run.live) return;
  run.live = false;
  run.timers.forEach(clearTimeout);
  run.timers = [];
  emit("run:done", { quest_id: run.id, code });
}
function startRun(id: string) {
  endRun(null);
  run.id = id;
  run.live = true;
  const out = (stream: "stdout" | "stderr" | "system" | "build", line: string, at: number) =>
    run.timers.push(setTimeout(() => run.live && emit("run:output", { quest_id: id, stream, line }), at));
  const lua = byId(id).runner === "lua";
  out("system", lua ? "lua src/main.lua" : "cargo run", 0);
  if (!lua) {
    out("build", "   Compiling quest v0.1.0", 150);
    out("build", "warning: unused variable: `x`", 300);
    out("build", "    Finished `dev` profile in 0.8s", 600);
    out("stderr", "(debug) seed = 7", 900);
  }
  out("stdout", "Welcome to the guessing game!", 800);
  out("stdout", "I'm thinking of a number between 1 and 100.", 1100);
  out("stdout", "Type a guess:", 1500);
  // waits for input; gives up after 8 s
  run.timers.push(setTimeout(() => endRun(0), 8000));
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
    const s = (x: Quest | null) => (x ? skin(x) : null);
    return { track, next: s(next ?? started), alternate: s(alt), profile: profile(), resume: next ? s(started) : null };
  },
  async list_flavours() { return ["plain", "games"]; },
  async list_tracks() { return tracks; },
  async list_worlds({ track }) {
    const t = track || state.settings.track || tracks[0].id;
    return worlds.filter((w) => w.track_id === t).map((w) => ({ ...w, quests: w.quests.map(skin) }));
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
  async open_editor() {
    if (!state.settings.editor) throw new Error("No external editor is set.");
    console.info("[mock] open_editor");
  },
  async list_files({ id }) {
    return [...filesOf(id)].map(([path, text]) => ({ path, bytes: text.length, read_only: readOnly(path), primary: path === primaryOf(id) }));
  },
  async read_file({ id, path }) {
    const t = filesOf(id).get(path);
    if (t === undefined) throw new Error(`no such file: ${path}`);
    return t;
  },
  async write_file({ id, path, text }) {
    const f = filesOf(id);
    if (!f.has(path)) throw new Error(`no such file: ${path}`);
    if (readOnly(path)) throw new Error(`${path} is read only`);
    f.set(path, text);
    if (path !== primaryOf(id)) return;
    // the watcher: re-grade 1.5 s after the last write
    if (writeTimer) clearTimeout(writeTimer);
    const qq = byId(id);
    emit("check:started", { quest_id: id });
    writeTimer = setTimeout(() => {
      writeTimer = null;
      let result: CheckResult;
      if (text.includes("oops")) {
        const line = text.slice(0, text.indexOf("oops")).split("\n").length;
        result = grade(qq, "compile");
        result.diagnostics = [{
          level: "error", message: "cannot find value `oops` in this scope", file: primaryOf(id), line,
          rendered: "error[E0425]: cannot find value `oops` in this scope\n --> " + primaryOf(id) + ":" + line + ":5",
        }];
      } else if (/(format!|to_string|String::from)\(.*\);\s*\n\}/.test(text.split("#[cfg(test)]")[0]) && qq.runner === "cargo") {
        // last line of a function ends in `;`: the classic "returns ()" error
        const line = text.slice(0, text.search(/\);\s*\n\}/)).split("\n").length;
        result = grade(qq, "compile");
        result.diagnostics = [{
          level: "error", message: "mismatched types", file: primaryOf(id), line,
          rendered: "error[E0308]: mismatched types\n --> " + primaryOf(id) + ":" + line + ":5\n  |\n  = note: expected struct `String`\n          found unit type `()`\nhelp: remove this semicolon to return this value",
        }];
      } else result = grade(qq, text.includes("format!") ? "pass" : "fail");
      state.attempts[id] = (state.attempts[id] ?? 0) + 1;
      state.last[id] = result;
      const completion = result.ok ? complete(qq, true) : null;
      emit("check:result", { quest_id: id, result, completion });
    }, 1500);
  },
  async reset_quest({ id }) { state.combo = 0; pushProfile(); delete files[id]; state.last[id] = null; state.attempts[id] = 0; return detail(id); },
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
    if (!state.completed.includes(id)) { state.combo = 0; pushProfile(); }
    return state.hints[id];
  },
  async read_point({ id, index }) {
    byId(id);
    if (state.points.has(`${id}#${index}`)) return 0;
    state.points.add(`${id}#${index}`);
    const xp = index < 100 ? 2 : index < 200 ? 1 : 5;
    state.xp += xp;
    pushProfile();
    return xp;
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
  async run_program({ id }) { startRun(id); },
  async run_input({ text }) {
    if (!run.live) throw new Error("Nothing is running.");
    const id = run.id;
    run.timers.push(setTimeout(() => emit("run:output", { quest_id: id, stream: "stdout", line: `You guessed: ${text}` }), 150));
    run.timers.push(setTimeout(() => { emit("run:output", { quest_id: id, stream: "stdout", line: "Too low. Bye!" }); endRun(0); }, 400));
  },
  async stop_program() { endRun(null); },
  async get_settings() { return { ...state.settings }; },
  async set_settings({ settings }) { state.settings = { ...settings }; pushProfile(); return { ...state.settings }; },
  async active_dates() { return dates; },
  async list_profiles() { return mockProfiles.map((p) => ({ ...p })); },
  async create_profile({ name }) {
    if (!name.trim()) throw new Error("a profile needs a name");
    mockProfiles.push({ id: name.trim().toLowerCase().replace(/[^a-z0-9]+/g, "-"), name: name.trim(), created: new Date().toISOString().slice(0, 10), active: false });
    return mockProfiles.map((p) => ({ ...p }));
  },
  async rename_profile({ id, name }) {
    const p = mockProfiles.find((x) => x.id === id);
    if (p) p.name = name.trim();
    return mockProfiles.map((p) => ({ ...p }));
  },
  async delete_profile({ id }) {
    if (mockProfiles.find((x) => x.id === id)?.active) throw new Error("switch to another profile before deleting this one");
    mockProfiles = mockProfiles.filter((x) => x.id !== id);
    return mockProfiles.map((p) => ({ ...p }));
  },
  async switch_profile({ id }) {
    mockProfiles.forEach((p) => (p.active = p.id === id));
    return profile();
  },
  async export_profile({ id }) { return `C:\\Users\\you\\Documents\\Oxide exports\\${id}.oxide-profile.zip`; },
  async import_profile() {
    mockProfiles.push({ id: "imported", name: "Imported", created: "2026-10-01", active: false });
    return mockProfiles.map((p) => ({ ...p }));
  },
};
