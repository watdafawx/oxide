# Writing quests

A quest is a folder. The engine (`crates/oxide-core/src/content.rs`) enforces everything below;
`cargo run -p content-check` tells you what you got wrong.

## Project first (the rule that matters most)

The learner has ADHD and gets hooked by building things for games, not by "this is an int".
So a world is **one project that grows**, quest by quest, and a concept shows up only because the
next feature needs it. The reader should always know what they are building and why this
quest's idea is the tool for it.

- `world.toml` names the project (`project = "Crafting calculator"`), and every quest adds a
  visible feature to it. The boss ships the finished thing.
- The lesson opens with **Why now**: one or two sentences tying the concept to the feature.
  "A stack holds 0 to 64 items, never -3: that is why stack sizes are `u32`, not `i32`."
- Function and type names come from the project, never from textbooks: `stack_size`,
  `Recipe`, `Inventory`, `Entity`, `ChunkPos`, `Command`, not `add`, `Foo`, `greet`.
- Tests read like features: `crafting_a_pickaxe_needs_three_ingots`, not `test_struct`.
- Game flavour is the user's own: Minecraft (mods, servers, modpacks), Factorio (belts,
  recipes, techs), launchers and download tools. Pick one spine per world and stay in it.
- Reads (quizzes) stay short and also use the project's vocabulary.

Project spines per track are listed in `docs/TRACKS.md`; a content session uses the spine
its brief names.

### Flavours: the same quest, two stories

Not everyone wants the game. A quest's folder is the **plain** version (textbook names, a
short real-world "why now"). It may carry a **games** flavour beside it:

```
01-hello/
  quest.toml, lesson.md, starter/, solution/     plain
  flavours/games/
    quest.toml      optional: title, summary, [[objectives]] (same count and order; this flavour's own test names and labels), [[hints]], [[quiz]]
    lesson.md       required
    starter/        required for exercise/boss: the game-named code
    solution/       required for exercise/boss
```

- Every flavour has the **same number of objectives in the same order** (objective N is the
  same step in every story); test names, labels, code names, lesson and story are the
  flavour's own, so a games test is `pickaxe_needs_three_ingots`, never `circle_area_uses_pi`.
  Progress is per quest, so one map, one progress record, one XP value.
- The learner picks the flavour in settings (`games` by default); a quest without that flavour
  shows plain. Switching mid-quest keeps the workspace; Reset copies the current flavour.
- content-check builds and runs every flavour's starter and solution.
- Worked example: `content/worlds/00-tutorial/01-hello/flavours/games`.

## Layout

```
content/tracks/<track>/track.toml   title, blurb, runner = "cargo" | "lua" | "gradle"
content/tracks/<track>/lib/         (optional) modules on every Lua quest's require path
content/tracks/<track>/worlds/
  01-foundations/
    world.toml               title = "Foundations"  blurb = "one sentence"
    01-variables/
      quest.toml
      lesson.md
      starter/               cargo project the learner gets (compiles, tests fail)
      solution/              same project, all green (never shipped; checked by CI)
    02-functions/
    ...
    12-boss-guessing-game/
```

- Folder names are `NN-slug`: two-digit index, lowercase slug, `a-z0-9-` only.
- Quest id = `<track>/<world folder>/<quest folder>`, e.g. `rust/01-foundations/03-functions`. In `requires`, a two-segment id (`01-foundations/03-functions`) means "in my own track"; use three segments to require a quest from another track (`java/02-oop/05-records`).
- The Rust track still lives at `content/worlds/` (read as track `rust`) until the content sessions finish; the engine accepts both layouts.
- Order in the UI = folder order. The boss is the last quest of its world.

## quest.toml

```toml
title = "Lend, don't give"
summary = "Pass a String to a function without losing it."   # one line, shown on the card
kind = "exercise"      # exercise | boss | read
xp = 50                # exercise 30-80, boss 300-500, read 15
minutes = 10           # honest estimate for a beginner; shown on the card, used for the speedrun badge
tags = ["borrowing"]
# runner = "lua"                         # defaults to the track's runner
# starter_fails_to_compile = true        # only when the lesson is about a compile error
# requires = ["02-ownership/02-moves"]   # omit = previous quest (first quest of a world = last of the previous world)
#                                        # requires = [] = no prerequisite

[[objectives]]                 # one per test function, in the order to solve them
test = "borrows_without_moving"    # suffix-matched: `tests::borrows_without_moving` also works
label = "Function takes &String and the caller can still use it"

[[hints]]                      # progressive: hint 1 nudges, the last one nearly gives it away
text = "Change the parameter type to `&String`."
[[hints]]
text = "Call it with `greet(&name)`."

# kind = "read" only:
[[quiz]]
question = "Which of these moves `s`?"
choices = ["let t = &s;", "let t = s;", "let t = s.len();"]
answer = 1
```

Rules: exercise/boss need `objectives`, `starter/`, `solution/`. Read quests need `quiz` and nothing else. Duplicate objective tests are an error.

## lesson.md

Markdown, rendered inside the app, one screen. Shape:

1. **Why now**: one or two sentences tying the concept to the feature you are adding to the
   world's project (see "Project first").
2. The idea, in 3-6 short paragraphs or bullets. One runnable example in a ```rust block.
3. `## Your turn` — what to do in the starter, in 2-4 bullets, naming the file and function.

Tone: direct, no fluff, no "in this lesson we will". Use `you`. A reader with ADHD will skim: bold the one term that matters, keep paragraphs to 3 lines. Never put the solution in the lesson.

## starter/

- A normal cargo project. `Cargo.toml` `name` = the quest slug with underscores, **unique across the whole catalog** (all quests share one build cache; prefix with the world slug if it clashes), `edition = "2024"`, and an empty `[workspace]` table at the end (keeps it out of the repo workspace). No `Cargo.lock`.
- Prefer a `lib.rs` with `#[cfg(test)] mod tests` at the bottom, or `tests/*.rs`. Binaries are fine for boss quests; test them through a lib.
- It **must compile** as shipped. Fill-in spots are `todo!()` or a wrong value, with a `// TODO:` comment saying what to write. Tests then fail at runtime, so the objectives list shows red ticks, not a wall of compiler errors. Exception: quests whose point is a compile error (borrow checker): set `starter_fails_to_compile = true` in quest.toml and say so in the lesson.
- Tests are the spec. Every objective test name must exist. Test names describe behaviour (`greet_returns_hello_name`), not `test1`.
- Keep test output helpful: `assert_eq!` over `assert!`, and a message when it is not obvious.
- No external crates unless the quest is about them. When needed, pin a version that exists on crates.io and matches what the solution uses.
- The learner edits `src/`. Don't put logic in tests they are meant to change.

## Lua quests (`runner = "lua"`)

```
starter/
  tests.lua        the spec; `local t = require("oxide.test")`, one `t.test(name, fn)` per objective
  src/<name>.lua   what the learner edits; `require("<name>")` in tests.lua
solution/          same, green
```

- Runs in an embedded Lua 5.2 (Factorio's version): no integers, `goto`, `table.unpack`. No `io`, no `os.exit`, 5 s limit per run.
- `require` resolves `src/<name>.lua`, then `<name>.lua` in the quest root, then the track's `lib/`.
- Harness asserts: `t.eq`, `t.ne`, `t.deep_eq`, `t.truthy`, `t.falsy`, `t.is_nil`, `t.near`, `t.is_type`, `t.contains`, `t.raises(fn, pattern?)`, `t.skip(reason)`; all take an optional trailing message. `t.group("prefix_", fn)` prefixes test names.
- The starter's functions return `nil` or a wrong value with a `-- TODO:` comment; the file must load without a syntax error so objectives show red ticks, not "could not run".
- Worked example: `content/tracks/lua/worlds/01-lua-core/03-arrays`.

## solution/

Identical to starter with the TODOs filled. `cargo test` green, `cargo clippy` clean. content-check runs both.

## Sizing

| kind | minutes | xp | objectives |
|---|---|---|---|
| exercise | 3-15 | 30-80 | 1-4 |
| boss | 30-90 | 300-500 | 4-10, staged (first objective passes early) |
| read | 2-5 | 15 | quiz: 1-3 |

First quest of a world: under 5 minutes. A world is 6-12 quests + boss. Every third quest or so, lower the XP and the minutes: an easy win keeps the streak alive.

## Checking

```
cargo run -p content-check -- --fast           # catalog only, instant
cargo run -p content-check -- 01-foundations   # one world (prefix of the id, with or without the track)
cargo run -p content-check -- lua/             # one track
cargo run -p content-check                     # everything
```
