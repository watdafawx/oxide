# Blind playtest 2026-10-20b: Rust worlds 10-apps and 11-mastery

Method: `docs/PLAYTEST-CHECKLIST.md`. I am an LLM, so "solved" means the solution path was derivable from
lesson + labels + starter + tests; I read those first, then audited `solution/` against what the lesson
(and earlier worlds 00-09) had taught. I did not hand-run a separate blind `cargo test` per quest: the
solutions are compiled and tested by `content-check` (see bottom). Several fixes were things I knew from
experience that a beginner would not, so "ok" below is generous for Rust-fluent players; hints used: 0
(I read hints only to judge them).

Network/display/toolchain: no quest needs a display or the wasm target. Every quest needs crates.io on
the first build (rusqlite `bundled` builds C code; egui/tauri trees take minutes). 09-wasm-ready-crate does
not need `wasm32` installed (the attribute is `cfg_attr`'d out natively).

| Quest | Solved | Verdict | Missing before | Fixed |
|---|---|---|---|---|
| 10/01 rusqlite-basics | yes | unclear | plain: `.collect()` of `query_map` rows into `Result<Vec<_>>` never shown; first build compiles C | lesson bullet on `.collect()`; minutes 4 -> 6 |
| 10/02 transactions-migrations | yes | unclear | `execute_batch`, `pragma_update`, `commit()` as final `Result` (plain); games also lacked prepare-once `stmt.execute` and `pragma_update` | both lessons extended |
| 10/03 serde-models | yes | ok | none | - |
| 10/04 choosing-ui-stack (read) | n/a | broken quiz | right answer was the longest choice in both questions, both flavours | choices rewritten, wrong ones longer, answers moved off the same index |
| 10/05 axum-hello | yes | ok | none (async/Router explained) | - |
| 10/06 axum-json-api | yes | unclear | plain: `Path(x): Path<T>` destructuring, atomic `load`, tuple `(CREATED, Json)`; games: `fetch_add`/`load`, `.route(..).post(..)`, `.with_state`, `post` | both lessons extended; games TODOs now spell the calls; wrong "Err is a 400" sentence fixed |
| 10/07 axum-sqlite-crud | yes | unclear | `collect::<Result<Vec<_>,_>>()`, `execute` row count, `last_insert_rowid` (plain); games had no list/`optional` example and vague TODOs | both lessons + games TODOs |
| 10/07 mod-card-text | yes | unclear | `if` as a value, tuple pattern in `filter` closure (plain) | lesson snippet |
| 10/08 egui-viewmodel | yes | unclear | `ui.horizontal(|ui| ..)`, `ui.label`, `saturating_sub` call form (plain). Games lesson contained the whole `ui()` solution | plain lesson extended; games example replaced by a different `Greeter` widget |
| 10/09 tauri-commands | yes | ok | games lesson's `install_mod_inner` example is the solution minus the blank check (left, see below) | - |
| 10/10 shipping (read) | n/a | broken quiz | right answer longest in both questions, both flavours | rewritten |
| 10/11 boss oxide-in-rust | yes | unclear | `clamp`, `as_ref`, `map_or`, `is_some_and`, let-else reminder, `ui.heading/separator/ProgressBar/key_pressed`, `egui::Modal`; broken markdown table swallowed the "Stage your work" paragraph | toolkit section; table fixed |
| 11/01 macro-rules-basics | yes | broken | both lessons' example was the solution (`let v = $x; v * v`); TODO did not say to change `{ }` to `{{ }}` | examples replaced (`clamp_chunk!`/`top_block!`); TODOs name the double braces |
| 11/02 macro-rules-repetition | yes | unclear | plain: no repetition-in-expansion example; games: example was `block_names!` solution verbatim, `=>` literal and recursion arms not shown | examples replaced/added; TODOs name the expansions |
| 11/03 proc-macros-overview (read) | n/a | broken quiz | Q2 right answer longest by 2x, both flavours | rewritten (right is now the shortest) |
| 11/04 derive-macro | yes | unclear | `match &input.data` over `Data::{Struct,Enum,Union}` never shown, only named in hints/TODO | match snippet in both lessons |
| 11/05 attribute-macro | yes | unclear | `static X: AtomicUsize`, `load`/`fetch_add` with full `::std` paths inside `quote!`, `#counter` splicing | snippet in plain, bullet in games |
| 11/05 block-registry | yes | unclear | plain: literal `=>`, `$crate::Struct {..}`, macro call inside `$(..),*`. Games lesson was the solution verbatim | plain note; games example replaced by `Tag`/`tag!`/`total_weight!`; TODOs name the expansion |
| 11/06 counting-allocations | yes | ok | `with_capacity` had no code (plain), no `join`/`with_capacity` code (games) | snippets; TODOs name `Vec::with_capacity(input.len())` / `names.join(" ")` |
| 11/07 read-profiling (read) | n/a | broken quiz | right answer longest in both questions, both flavours | rewritten |
| 11/08 fast-paths | yes | unclear | `longest` by reference pattern unshown; games lesson held all four solutions | plain: ref-compare example + owned-key note; games block replaced with different-data examples; TODOs name the calls |
| 11/09 wasm-ready-crate | yes | unclear | Fibonacci/two-variable loop and `(a, b) = (b, a + b)` tuple assignment never shown; games lesson was the solution (`slimes_after`) | plain: tuple-assign example; games: `doubling` example + tuple-assign |
| 11/10 no-std-taste | yes | ok | none | - |
| 11/11 boss optimise-slow-crate | yes | unclear | `get_mut(&str)`, `unwrap_or_default`, `sort_unstable_by(..then_with..)`, `take` only in hints | Toolkit snippet in both lessons |

## Left as is, on purpose

- `10/09` games lesson shows an `install_mod_inner` with the duplicate check (it IS the pattern; blank-id check and wrappers remain the learner's work).
- Hints that name the exact call were kept; they only appear after the lesson.
- Minutes: only 10/01 changed. 11/08 (15) and the bosses (90/75) look tight for a beginner but I did not time them.
- Folder numbering has two `07-` (axum-sqlite-crud, mod-card-text) and two `05-` (attribute-macro, block-registry) quests; order by `requires` is intact, not renamed.

## Also seen (not mine)

- While I ran `content-check`, `rust/04-errors/10-boss-list-manager/quest.toml` had a TOML parse error at line 52 (sibling session mid-edit); it blocks the whole catalog load.
