# Rust track: term ledger

Every term, keyword and piece of syntax, in the order the track introduces it, with the quest
that does. A lesson may use only rows at or above its own quest. Add a row when you introduce
something. Helpers a starter contains (marked *starter*) are shown, not taught.

| term | quest | how it is introduced |
|---|---|---|
| save runs the tests, objective ticks, red = "not yet" | 00-tutorial/01-hello | the edit-save-green loop |
| `todo!()` placeholder, a function with a gap (*starter*) | 00-tutorial/01-hello | "replace the line" |
| `format!`, `{name}` | 00-tutorial/01-hello | build a greeting |
| last line without `;` hands back the answer (copied habit, named only) | 00-tutorial/01-hello | "no semicolon" |
| `fn` shell, `name: &str`, `String` result (*starter*, not explained) | 00-tutorial/01-hello | the function is already written |
| test, objective, red/green, `assert_eq!` left/right, compiler errors above the objectives | 00-tutorial/02-fix-it | read a failing assert |
| boss, staged objectives | 00-tutorial/03-boss-name-tag | clear them top to bottom |
| `.to_uppercase()` method call (*starter*, shown in the lesson code) | 00-tutorial/03-boss-name-tag | text with a dot method |
| `format!("{}", call())` | 00-tutorial/03-boss-name-tag | put a function's answer in text |
| a function calling a function | 00-tutorial/03-boss-name-tag | `name_tag` uses `shout` |
| `let`, variable, `*`, `+`, `-` | 01-foundations/01-a-name-for-a-number | 12 items get a name |
| last line without `;` hands back the value (*starter*, named only) | 01-foundations/01-a-name-for-a-number | "leave it alone for now" |
| `+=`, `-=`, `mut`, immutable, `error[E0384]` | 01-foundations/02-the-count-changes | picking up items fails to build |
| `fn`, parameter `logs: u32`, `-> u32`, `u32`, call, body, expression vs statement | 01-foundations/03-a-rule-you-can-reuse | planks for any log count |
| shadowing (`let name = name...`) | 01-foundations/04-same-name-new-value | clean a typed name |
| `.trim()`, `.len()` (named in the Your turn step) | 01-foundations/04-same-name-new-value | trim then count letters |
| type, `u32` unsigned, `i32` signed, `u8`, `i8`, `let x: u32`, `error[E0308]`, literal out of range | 01-foundations/05-how-big-can-a-number-be | a drop is negative, 200 does not fit in `i8` |
| `/` on whole numbers, `%` remainder | 01-foundations/06-full-stacks-and-leftovers | full stacks and loose items |
| overflow, `panicked ... attempt to subtract with overflow`, `.saturating_sub` | 01-foundations/07-stop-at-zero | take more than there is |
| `f64`, `1.5`, `2.0`, `error[E0277]` for `f64 / integer` | 01-foundations/08-halves-and-decimals | 1.5 seconds per item |
| `as`, cast, `as f64`, cast cuts decimals | 01-foundations/09-turn-one-number-into-another | average of two counts |
| `bool`, `==` `!=` `<` `>` `<=` `>=`, `if` / `else` / `else if` as a value | 01-foundations/10-either-or | stack limit by item type |
| `for x in a..=b`, ranges (`..` vs `..=`), running total with `mut` | 01-foundations/11-count-the-steps | blocks in a staircase |
| `while`, a counter, `let mut p = p;` for a parameter, signed overshoot | 01-foundations/12-repeat-until | swings to break a block |
| tuple `(a, b)`, `.0`, destructuring `let (a, b) = t;`, returning a tuple | 01-foundations/13-a-pair | an inventory slot |
| array `[T; N]`, `a[i]`, `.len()`, out-of-bounds panic | 01-foundations/14-a-row | a crafting row |
| Option / Result as ideas, "no null", "no exceptions", `Some` `None` `Ok` `Err` named | 01-foundations/15-how-rust-thinks | read quest, previews 23 and 26 |
| `struct`, field, struct literal, `String`, `.to_string()`, `s.field`, `format!("{} x{}", a, b)` | 01-foundations/16-a-thing-with-parts | an item stack |
| `impl`, method, `&self`, `self.field`, call with a dot | 01-foundations/17-give-it-a-verb | `is_full` |
| associated function, `Type::new`, `::` | 01-foundations/18-start-from-scratch | a constructor |
| `&mut self`, `error[E0594]`, `let mut` on the caller | 01-foundations/19-change-it-in-place | take items out |
| `enum`, variant, `Enum::Variant`, `match`, arm, exhaustive, `error[E0004]` | 01-foundations/20-one-of-a-few | stack size by item kind |
| variant with data `Tool(u32)`, binding the data in an arm | 01-foundations/21-choices-that-carry-data | durability, servings |
| match on numbers, range pattern `1..=2`, `_` catch-all, first arm wins | 01-foundations/22-match-a-range | rarity by enchantments |
| `Option<T>`, `Some`, `None`, match on text | 01-foundations/23-maybe-nothing | recipe lookup |
| `.unwrap_or`, `.unwrap()` (named as one to avoid), `error[E0369]` for `Option + int` | 01-foundations/24-a-fallback | a number out of an Option |
| `if let Some(x) = ...` | 01-foundations/25-only-if-there-is-one | describe a recipe |
| `Result<T, String>`, `Ok`, `Err`, `Err(format!(..))` | 01-foundations/26-worked-or-not | reject an oversized stack |
| `.parse::<u32>()`, match on `Ok(n)` / `Err(_)` | 01-foundations/27-handle-both-paths | read a typed count |
| `Vec`, `Vec::new()`, `.push`, `for x in &list`, `return`, `break`, recursion | 01-foundations/28-boss-crafting-calculator | taught in the boss lesson, shaped in the TODOs |
| `split_once`, `div_ceil`, `starts_with`, `BTreeMap`, closures, `?`, `.iter().map().collect()` (*starter* / TODO only) | 01-foundations/28-boss-crafting-calculator | given helpers or named in a TODO, never taught |
