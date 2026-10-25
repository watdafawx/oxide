# Content audit, 2026-10-24

Nine auditors read every quest in all four tracks (31 worlds). Each one scored its worlds 1-10 on four things: clarity for a beginner, value for an experienced dev, coherence and order, and how well the task fits the lesson. The detailed reports are in this folder:

- `rust-00-01.md`, `rust-02-04.md`, `rust-05-07.md`, `rust-08-10.md`, `rust-11-12.md`
- `java.md`, `lua.md`
- `gpui-00-05.md`, `gpui-06-11.md`

## World ranking

| # | World | Beginner | Experienced | Coherence | Task fit | Overall |
|---|---|---|---|---|---|---|
| 1 | gpui/03-live | 7 | 8 | 8 | 8 | **7.5** |
| 1 | gpui/09-overlay | 7 | 8 | 8 | 7 | **7.5** |
| 3 | rust/00-tutorial | 6 | 7 | 8 | 8 | **7** |
| 3 | gpui/01-readings | 8 | 3 | 8 | 9 | **7** |
| 3 | gpui/04-sensors | 6 | 8 | 7 | 7 | **7** |
| 3 | java/01-basics | 7 | 4 | 7 | 7 | **7** |
| 7 | rust/01-foundations | 6 | 5 | 7 | 7 | **6.5** |
| 7 | rust/07-concurrency | 5 | 7 | 8 | 7 | **6.5** |
| 7 | rust/12-broadcast | 4 | 8 | 8 | 6 | **6.5** |
| 7 | lua/01-lua-core | 7 | 5 | 6 | 7 | **6.5** |
| 7 | java/00-setup | 6 | 5 | 7 | 7 | **6.5** |
| 7 | gpui/05-taskbar | 5 | 8 | 7 | 6 | **6.5** |
| 7 | gpui/07-settings | 6 | 7 | 7 | 6 | **6.5** |
| 7 | gpui/10-dashboard | 6 | 7 | 7 | 6 | **6.5** |
| 7 | gpui/11-screen-time | 6 | 7 | 8 | 5 | **6.5** |
| 16 | gpui/00-setup | 5 | 5 | 7 | 8 | **6** |
| 16 | gpui/02-views | 5 | 6 | 6 | 7 | **6** |
| 16 | gpui/06-interaction | 5 | 6 | 7 | 6 | **6** |
| 16 | gpui/08-widget | 6 | 6 | 7 | 6 | **6** |
| 20 | rust/08-async | 4 | 7 | 6 | 6 | **5.5** |
| 20 | rust/09-systems | 3 | 8 | 5 | 7 | **5.5** |
| 20 | lua/02-factorio-data | 3 | 8 | 5 | 7 | **5.5** |
| 20 | java/03-collections | 3 | 8 | 5 | 6 | **5.5** |
| 24 | rust/02-ownership | 4 | 6 | 5 | 5 | **5** |
| 24 | rust/03-collections | 3 | 7 | 4 | 6 | **5** |
| 24 | rust/05-traits | 4 | 6 | 6 | 7 | **5** |
| 24 | rust/06-smart-pointers | 3 | 6 | 5 | 6 | **5** |
| 24 | rust/11-mastery | 3 | 7 | 4 | 5 | **5** |
| 24 | java/02-oop | 3 | 8 | 5 | 6 | **5** |
| 30 | rust/10-apps | 3 | 6 | 4 | 6 | **4.5** |
| 31 | rust/04-errors | 3 | 7 | 4 | 6 | **4** |

The ranking follows one rule: a world that got the house-style rewrite (rust 00-01, java 00-01, lua 01) scores 6.5-7 for beginners. Every world that was not rewritten drops to a beginner score of 3-4.

## Problems found in every track

1. **Only the first worlds follow the house style.** After them, lessons turn into cheat sheets: one code block, bullet glossaries and "**Why now:**" labels. The beginner score drops sharply at rust 01→02, java 01→02 and lua 01→02.
2. **Core ideas are never taught.** No lesson explains what a function, method, class, parameter or call is, or what a macro (`!`) or attribute (`#[...]`) is. Nothing covers memory (stack, heap, pointers, freeing) or compile time vs run time. The Rust ownership and smart-pointer worlds can't be understood without these.
3. **Ideas are used before they are taught.** Some Rust examples:
   - closures are used in 02/03 and taught in 03/06;
   - traits are needed all through world 04 and taught in world 05;
   - `?` is taught in Collections;
   - serde is used in 08 and taught in 10.

   Java uses generics, lambdas and `new` early. Lua uses functions and `M.x` before teaching them. GPUI uses Rust from worlds it doesn't require.
4. **The answer is handed to the learner.** In GPUI, the lesson's code block is the solution line in about 120 of 140 exercises. In rust 08-11, lua and java 02-03, TODO comments contain the exact code. Beginners just copy, and experienced devs have nothing to solve.
5. **Quiz answers are guessable.** 510 of 637 `[[points]]` have `answer = 0`, and `PointStep.svelte` doesn't shuffle the choices, so tapping the top choice wins.
6. **Boss quests are a cliff.** Six bosses ask for many tools that no lesson taught: rust/01, rust/04, lua/01 (a parser needing `while`, string patterns and forward declarations), java/01 (`charAt`), and gpui 06/08/10.
7. **Unlock gates are missing.** `lua/02` and `java/03` have `requires = []`.
8. **The term ledgers stop early.** `docs/tracks/rust-terms.md` has no rows for worlds 02-11, and `java-terms.md` stops at 01. That's why the "used before taught" problem went unnoticed.

## Suggested fixes, in order

**Quick, mechanical**
- Shuffle the `[[points]]` choices in `PointStep.svelte`. This fixes every quiz without touching content.
- Fix the unlock gates (`lua/02` should require `lua/01/23`; set the right gate for `java/03`).
- Fix the confirmed bugs listed in the reports:
  - `format!("{kit.output}")` in the rust/01 boss TODO doesn't compile;
  - the rust/01/05 lesson gives the wrong reason for the compile error;
  - the 11-10 toast test checks strings that no lesson mentions;
  - the boss lessons in rust 06/07 have broken `'\n'` code;
  - games-flavour text leaks into plain lessons;
  - "spike R3/R5" and "this PC" wording should go.
- Remove exact code from TODO comments. Keep the exact code only in the last hint, as `CONTENT.md` says.

**Content rewrites (biggest gain)**
- Rewrite these worlds the way rust 00-01 were rewritten, in this order: rust 02 → 04 → 03 → 05 → 06, then java 02-03, then lua 02.
- Extend the term ledgers as each world is rewritten.
- Move topics so nothing is used before it is taught: closures before iterators, traits before errors, `?` into Errors.
- Add 2-4 bridge quests before each boss that is a cliff.
- In GPUI, show a *similar* example in the lesson, not the solution line. Add one "anatomy" quest each for the builder chain, the listener closure and the `cx` types.

**Experienced devs**
- Add a "skip ahead" test at the start of each beginner world, or a fast lane.
- Add the depth topics the auditors listed (orphan rule, blanket impls, which traits can be used as `dyn`, arenas, bounded channels, and others).

## Proposal: a "Programming 101" track for complete beginners

Every auditor separately listed the same missing basics. A short track that comes before any language track would teach them once. The current tracks could then assume them, and experienced devs could skip it entirely.

Possible worlds, about 6-8 quests each, using a simple language (Lua is the natural fit, since the runner already exists):

1. **What a program is.** Source code vs running it, compiling vs interpreting, the terminal, reading an error message, what a test is and why green means done.
2. **Values and names.** Numbers, text, true/false, variables, why types exist, comments.
3. **Decisions.** `if`/`else`, comparisons, `and`/`or`/`not`.
4. **Repetition.** Loops, counting loops, `break`/`continue`, running totals, min/max.
5. **Functions.** Defining vs calling, parameters vs arguments, return values, scope, why to split code up.
6. **Collections.** Lists and lookup tables (maps), indexing, looping over them.
7. **How the computer holds things.** Memory, where values live, references vs copies, bits/bytes/hex. This is what Rust ownership, Java objects and systems work all build on.
8. **Building something.** A small multi-function program, plus reading docs and debugging by narrowing down.

Each language track's first world would then link to these concepts ("you met loops in 101/04; here's Rust's version") instead of teaching them half-way.
