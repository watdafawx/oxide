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
| memory, address, stack, heap, allocate, free, pointer, leak, double free, use after free, garbage collector, owner (read) | 02-ownership/01-where-values-live | where a `String`'s letters live; why Rust needs no collector |
| block `{ }`, scope, dropping, `error[E0425]`, a block/`if` hands its value out | 02-ownership/02-gone-at-the-brace | a greeting built inside an `if` vanishes |
| move, `let b = a;` on a `String`, `error[E0382]`, moving into a struct field, `String::from`, `usize` | 02-ownership/03-hand-it-over | `len()` after the text moved into a struct |
| a `String` parameter takes ownership, `.clone()`, clone costs a heap copy | 02-ownership/04-a-second-copy | `name_length(name)` keeps the name |
| attribute `#[...]`, `#[derive(...)]`, `Debug`, `PartialEq`, `Clone`, `Copy`, the word trait (named only), every field must be Copy | 02-ownership/05-copy-your-own-type | `(p, p)` on a struct of numbers |
| reference, borrowing, `&x`, `&T` parameter, many readers, macro (`!`), `vec![..]` | 02-ownership/06-lend-it | count a backpack without taking it |
| `&mut x`, `&mut T` parameter, `push_str`, `Vec::pop` returns `Option` | 02-ownership/07-lend-to-change | add text, push twice |
| one writer or many readers, `error[E0502]`, a borrow ends at its last use, `&mut` to two different values | 02-ownership/08-one-writer-at-a-time | read the first item while pushing |
| `*` dereference, `*r += n`, `*r > n`, auto-deref on `.` methods and fields, `&&T` / `**r` (named) , `error[E0368]` | 02-ownership/09-follow-the-arrow | add xp through `&mut u32` |
| `for x in &mut list`, `*=`, `for x in list` moves the list | 02-ownership/10-change-every-item | change every item in place |
| the three borrow-checker questions, non-lexical borrows (read) | 02-ownership/11-the-borrow-checker-is-your-friend | trace six lines |
| `&str` vs `String`, literals are `&str`, `&String` -> `&str` coercion, `'static` (named, for the curious) | 02-ownership/12-owned-or-borrowed-text | greet takes literals and Strings |
| `.as_str()` for `match` on a `String`, `error[E0308]` | 02-ownership/13-match-on-text | rank by role |
| dangling reference, `error[E0515]`, return the owned `String` | 02-ownership/14-new-text-is-owned | shout can't return `&loud` |
| string slice `&s[a..b]`, `..i` / `i..`, `str::find` -> `Option<usize>`, slices count bytes (for the curious) | 02-ownership/15-part-of-the-text | first word of a line |
| list slice `&[T]`, `&Vec<T>` -> `&[T]`, empty slice `&[]` | 02-ownership/16-part-of-a-list | middle and last three |
| `error[E0507]` cannot move out of index, `Vec::remove(i)` hands over the owned item, `for i in 0..v.len()` with `return` | 02-ownership/17-take-it-out | take the lamp out of the bag |
| struct fields own (`String` over `&str`), `error[E0106]`, ownership nests | 02-ownership/18-structs-own-their-fields | a note that outlives its typed text |
| lifetime `'a`, `struct S<'a> { f: &'a str }`, `'_` in a return type | 02-ownership/19-a-struct-that-borrows | a view that borrows the title |
| `fn f<'a>(a: &'a str, b: &'a str) -> &'a str`, lifetime elision rules (for the curious) | 02-ownership/20-which-input-is-it | the longer of two titles |
| `Vec::join`, `!` on a `bool`, `is_empty`, `split_once`, `*line` on `&&str`; `split_whitespace`, `parse::<i32>` (*starter* `parse_pos`, games) | 02-ownership/21-boss-text-adventure | taught in the boss lesson, shaped in the TODOs |
| `vec![...]`, macro (the `!`), `println!` named, `Vec<T>` read as "a Vec of T" | 03-collections/01-a-list-of-items | write the opening stock down at once |
| `{:?}`, Debug vs Display, **trait** (a skill a type has or not), `error[E0277]` doesn't implement Display | 03-collections/02-see-whats-inside | a log line for a whole list |
| reference from `for n in list` (`&u32`), `*` dereference, `error[E0308]` expected `&u32`; where Rust follows the arrow for you | 03-collections/03-follow-the-reference | count low shelves |
| `usize` (positions and lengths), `.get(i)` → `Option<&T>`, `.copied()` | 03-collections/04-a-slot-by-number | count on shelf N, safely |
| `remove(i)` hands the item back, `insert(i, x)`, sliding cost | 03-collections/05-move-an-item | move a product to the front |
| `sort()`, `dedup()` (neighbours only), sort-then-dedup, capitals sort first | 03-collections/06-tidy-the-pile | tidy the order list |
| standard library `std`, `use std::collections::HashMap;`, `HashMap<K, V>`, key/value, `insert`, `get`, no order | 03-collections/07-look-up-by-name | price by product name |
| `entry(k).or_insert(0)` → `&mut V`, `*... += 1`, why owned String keys | 03-collections/08-count-with-entry | count sales per product |
| closure `\|p: u32\| ...`, capturing, `error[E0434]` can't capture in a fn item | 03-collections/09-a-rule-without-a-name | a tax rule that uses `percent` |
| closure as a parameter, `impl Fn(u32) -> bool`, `Fn` is a trait, **impl Trait**; `move` closures (for the curious) | 03-collections/10-hand-over-a-rule | one `count_where` for every rule |
| `retain(closure)`, closure gets `&T` | 03-collections/11-keep-what-matches | drop sold-out products |
| iterator, `.iter()`, `.map(closure)`, `.collect()` told by the return type | 03-collections/12-change-every-item | sale prices for the whole list |
| lazy iterators, adaptor vs consumer, "unused Map that must be used" warning (read) | 03-collections/13-iterators-are-lazy | quiz |
| `.filter(closure)`, `&&T` in filter, `.copied()` before filter, field access auto-derefs, `Vec<&T>` | 03-collections/14-keep-some | what can I buy for 5 euros |
| `.sum()`, `.count()` (a `usize`), sum overflow | 03-collections/15-one-number-answers | total units, low count |
| turbofish `::<T>` (`sum::<u32>()`), `_` "you work it out" (`collect::<Vec<_>>()`), `error[E0283]` type annotations needed | 03-collections/16-name-the-type | average price |
| `iter()` / `iter_mut()` / `into_iter()` = lend / lend to change / give away; `for x in &v` / `&mut v` / `v`; `error[E0507]` cannot move out ... behind a shared reference | 03-collections/17-look-change-or-take | raise prices in place, take the names |
| `.enumerate()`, `(usize, item)` pairs, tuple pattern in a closure `\|(i, x)\|`, `_` half; `zip`, `windows` (for the curious) | 03-collections/18-number-the-slots | numbered stock sheet, low shelf positions |
| `.take(n)`, `.skip(n)`, pages, chain order | 03-collections/19-one-page-at-a-time | ten lines per page |
| `.find(closure)` → Option, `.any`, `.all` (empty: false / true); `.position` (for the curious) | 03-collections/20-the-first-match | first affordable price, anything sold out |
| `.fold(start, \|acc, x\| ..)`, accumulator | 03-collections/21-carry-a-running-value | stacked discounts |
| `.max_by_key`, `.min_by_key`, key, `Option<&T>`, ties | 03-collections/22-the-biggest | longest name, fewest left |
| `sort_by(\|a, b\| ..)`, `Ord` trait, `Ordering` (`Less`/`Equal`/`Greater`), `a.x.cmp(&b.x)`, swap for biggest first; `sort_by_key` (for the curious) | 03-collections/23-biggest-first | most units first |
| `f64` has no `Ord` (NaN), `error[E0277]` `f64: Ord`, `total_cmp`, `min_by` / `max_by` | 03-collections/24-decimals-compare-differently | sort decimal prices, cheapest item |
| `split(sep)`, `.trim()` per piece, `.lines()`, `.is_empty()`, pieces are `&str` slices | 03-collections/25-cut-text-into-pieces | cut a CSV line into fields |
| `.ok()` (Result → Option), `.filter_map`, `parse` target from the collected type, `.is_err()` | 03-collections/26-keep-what-parses | keep the counts that parse |
| UTF-8, `.len()` counts bytes, `.chars()`, `char` `'a'`, `is_ascii_uppercase` / `is_ascii_digit`, slicing mid-letter panics | 03-collections/27-letter-by-letter | letter count, product codes |
| `join(sep)`, `"\n"`, `concat` | 03-collections/28-glue-it-back | shelf label, order sheet |
| format width `{:<10}` `{:>5}`, precision `{:.2}`, `{:>8.2}`; `{:^10}`, `{:>w$}` (for the curious) | 03-collections/29-line-up-columns | a tidy table row |
| `HashSet`, collect into a set, `contains`, `intersection`; `union`, `difference` (for the curious) | 03-collections/30-no-repeats | distinct products, shared stock |
| `BTreeMap` (sorted keys), `map.iter()` gives `(k, v)` pairs | 03-collections/31-keys-in-order | sales report in a stable order |
| `?` on `Result` and `Option` (early return), `std::num::ParseIntError`, error type must fit | 03-collections/32-pass-the-error-up | qty times price from text |
| `map_err(\|_\| ..)`, `error[E0277]` `?` couldn't convert the error to `String`, `map_err(...)?` | 03-collections/33-say-what-went-wrong | bad qty / bad price messages |
| collect `Result`s into `Result<Vec<_>, _>` (first `Err` wins), `Option<Vec<_>>` | 03-collections/34-all-or-nothing | a delivery note, all or nothing |
| everything above in one tool; `panic::catch_unwind` in `main.rs` (*starter*) | 03-collections/35-boss-csv-stats | CSV stock stats (games: modpack stats) |
| errors are values, exceptions (what Rust does not have), panic previewed | 04-errors/01-errors-are-values | read quest: a broken todo line / manifest line |
| error enum (one variant per failure, data in variants), `return Err(..)` inside `if` and `match` arms, `.is_empty()` (named in the TODO) | 04-errors/02-a-reason-per-failure | `parse_priority` / `parse_version` |
| **trait**, method signature without a body, `impl Trait for Type`, `#[derive(Debug, PartialEq)]` = compiler-written impls, `Debug` / `{:?}`, `PartialEq` / `==`, `error[E0599]` | 04-errors/03-a-promise-of-methods | `impl Label for PriorityError` (starter fails to compile) |
| `std::fmt::Display`, `fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result`, formatter, `write!`, `use std::fmt;` (path, named only), `.to_string()` and `{}` from Display | 04-errors/04-teach-it-to-print | messages for `PriorityError` / `ManifestError`, `explain` |
| `panic!`, `.unwrap()` (quiet panic), `.expect("why")`, rule: user can cause it -> `Err`, only a bug can -> panic | 04-errors/05-panic-or-result | read quest |
| `?` refresher, `std::num::ParseIntError`, `From<T>` trait, `fn from(e) -> Self`, `Self`, angle brackets fill a blank in a trait, `?` calls `From::from`, `Into` for free (for the curious) | 04-errors/06-convert-with-from | `total_priority` / `pack_fingerprint` |
| `Box<T>`, heap (memory asked for while running), pointer of fixed size, `dyn Error`, `std::error::Error`, `Box<dyn Error>`, `.into()` on `&str`, `split_once` (in the TODO and the example), `ParseFloatError` messages; `main -> Result`, `+ Send + Sync` (for the curious) | 04-errors/07-one-box-for-any-error | `parse_estimate` / `parse_pack_header` |
| `impl std::error::Error for T {}`, default methods, `Error` needs `Debug` + `Display`, `error[E0277]` "`?` couldn't convert the error", `source()` (for the curious) | 04-errors/08-your-error-fits-the-box | starter fails to compile |
| crate, crates.io, Cargo, `Cargo.toml` `[package]` `[dependencies]` `[dev-dependencies]`, semver and caret ranges, `Cargo.lock`, `cargo add` | 04-errors/09-crates-and-cargo | read quest |
| attribute `#[...]`, thiserror: `#[derive(Error)]`, `#[error("... {0}")]`, `#[from]` | 04-errors/10-thiserror | `TaskError` / `ManifestError` |
| anyhow: `anyhow::Error`, `anyhow::Result<T>`, `bail!`, macro (`!` = code that writes code), library vs app errors | 04-errors/11-anyhow-and-bail | `parse_hours` / `parse_ram_percent` |
| `.context(..)`, `Context` trait (a crate's trait adds methods to `Result`), root cause, `err.root_cause()`, `{err:#}` chain, `.with_context(\|\| ..)` is lazy; `Context` on `Option` (for the curious) | 04-errors/12-add-context | `priority_on_line` / `version_of_mod` |
| `mod name { }`, module, private by default, `pub`, `error[E0603]` | 04-errors/13-a-room-for-code | starter fails to compile |
| `super::`, `self::`, `crate::`, `use path::item;`, `use super::*;` in tests (`*` = everything) | 04-errors/14-paths-between-rooms | `minutes_for` / `library_mb` |
| `mod name;` loads `src/name.rs`, `pub mod`, siblings via `crate::`, `error[E0433]` | 04-errors/15-one-file-per-module | starter fails to compile |
| file, `std::path::Path`, `&Path`, `Path::new`, `std::fs::read_to_string`, `std::io::Error`, `path.display()`, `text.lines()`, `!` on a bool (named in the TODO) | 04-errors/16-read-a-file | `load_text`, `count_tasks` / `count_mods` |
| `e.kind()`, `io::ErrorKind::NotFound`, match guard `Err(e) if .. =>` | 04-errors/17-a-missing-file-is-fine | `load_or_empty` |
| `std::fs::write`, `()` the empty value, `'\n'`, `String::push_str` / `push` (named in the TODO), write replaces the file | 04-errors/18-write-it-back | `save_tasks` / `save_mods` |
| `for x in &mut list` to change items in place, `!` flips a bool, `.lines().enumerate()` pairs, `join("\n")` (named in the TODO), `impl Default` (*starter*) | 04-errors/19-boss-list-manager | taught in the boss cards; `split_verb`, `format_line`, `parse_line` given |
| trait, method signature, `impl Trait for Type`, no classes / no inheritance | 05-traits/01-first-trait | circles and squares both need `describe` |
| required vs default method, override, a default calling `self.other()` | 05-traits/02-default-methods | `greet` written once, `shout` on top |
| generic function `fn f<T>`, type parameter, trait bound `T: Shape`, "one copy per type" (named only) | 05-traits/03-one-function-any-type | the same area loop for circles and squares |
| `error[E0369]` on unbounded `T`, `PartialOrd`, `Display` as bounds, `+` between bounds, `where` clause | 05-traits/04-no-bound-no-greater-than | `largest` for pixels, decimals and names |
| generic struct `Pair<T>`, `impl<T> Pair<T>`, `Self`, method taking `self` by value (moves) | 05-traits/05-a-pair-of-anything | `swap` uses up the old pair |
| bounded impl block `impl<T: PartialOrd>`, "method exists but bounds not satisfied"; blanket impl / `ToString` (for the curious) | 05-traits/06-methods-for-some-types | `larger` only for comparable pairs |
| `error[E0308]` mixed `vec!`, slots of one size, heap, `Box::new`, `dyn Trait`, `Box<dyn Shape>`, trait object | 05-traits/07-a-list-of-mixed-shapes | one canvas list of circles and rects |
| `&dyn Shape`, `&Rect` coerces to `&dyn Shape`, `box.as_ref()`, Box vs reference | 05-traits/08-borrow-a-trait-object | the layers-panel line |
| monomorphisation, static vs dynamic dispatch, vtable, "two addresses", dyn compatibility / no `dyn Clone` (for the curious) | 05-traits/09-static-vs-dynamic-dispatch | read quest, after both were written |
| `Default`, `fn default() -> Self`, `#[derive(Default)]` field defaults, hand-written `impl Default`, `unwrap_or_default` | 05-traits/10-a-sensible-default | a pen 0 wide draws nothing |
| `Hash`, `Eq` vs `PartialEq`, NaN != NaN, `HashSet`/`HashMap` key bounds, `error[E0277] Cell: Eq`; equal values hash equal (for the curious) | 05-traits/11-equal-and-hashable | cells a brush touched |
| associated type `type Output`, `Self::Output`, `C::Output`, bound on an associated type, why not `Convert<Output>` | 05-traits/12-a-type-inside-a-trait | f64 for degrees, usize for word counts |
| associated const `const NAME: &str`, `Type::NAME`, `T::NAME`, `f::<Type>()` call | 05-traits/13-a-constant-inside-a-trait | converter names in a menu |
| `Iterator`, `next`, `type Item`, why `&mut self`, `None` ends it | 05-traits/14-build-your-own-iterator | a 3, 2, 1 countdown |
| adaptors are default methods of `Iterator`; `IntoIterator` / why `for` works (for the curious) | 05-traits/15-adaptors-for-free | `filter`/`map`/`sum` on your countdown |
| operator traits `std::ops::{Add, Sub, Mul, Neg}`, `type Output` on `Add`, `self` by value + `Copy` | 05-traits/16-make-plus-work | `point + offset` |
| `Mul<Rhs>`, right-hand side as a generic vs `Output` associated, `impl Mul<Vec2> for f64`; orphan rule / coherence (for the curious) | 05-traits/17-times-a-number | `v * 2.5` and `2.5 * v` |
| `PartialOrd` must agree with `==`, `partial_cmp`, `Option<Ordering>`, `Ordering::{Less, Equal, Greater}`, NaN has no order, newtype key `Length(f64)` | 05-traits/18-which-is-closer | snap to the closer point |
| `impl Trait` in argument and return position, unnameable iterator types, one concrete type (`error[E0308]` if/else) | 05-traits/19-impl-trait | `shout`, `evens` |
| `Ord` vs `PartialOrd`, `error[E0277] f64: Ord`, `total_cmp`, `max_by`, `min_by`, `sort_by` with a comparison closure | 05-traits/20-sort-by-a-float | sort layers by opacity |
| a boss combining trait, trait objects, generic helpers and `impl<S: Shape> Shape for Scaled<S>`; formulas as doc-comment spec | 05-traits/21-boss-shape-renderer | ASCII shape renderer |
| stack, heap, dropped/freed (one-card refresher of world 02), compile time, pointer = address, `Box<T>`, `Box::new`, `error[E0072]` infinite size | 06-smart-pointers/01-box | a plan that contains a plan has no size |
| recursion, base case, stack overflow, `&Box<T>` passed as `&T`, `*` to read a referenced number | 06-smart-pointers/02-walk-the-chain | total minutes of a plan |
| `Deref`, `type Target`, `deref(&self)`, smart pointer, deref coercion (why `&String` works as `&str`), `error[E0614]` | 06-smart-pointers/03-deref | a home-made box |
| scope, `Drop`, `drop(&mut self)`, reverse drop order, `error[E0040]` | 06-smart-pointers/04-drop | a temp file deletes itself at the brace |
| `drop(x)` ends a value early and its borrows, `error[E0499]` while a value holds `&mut` | 06-smart-pointers/05-drop-early | delete the temp file before logging "saved" |
| allocation, `Cow<str>`, `Cow::Borrowed` / `Cow::Owned`, `.into_owned()`, `matches!`, `Cow<'_, str>` | 06-smart-pointers/06-cow | tidy tabs only when there are any |
| `Rc<T>`, reference counted, `Rc::new`, `Rc::clone`, `Rc::strong_count`, `error[E0382]` for two owners; `Arc` named for world 07 | 06-smart-pointers/07-rc | one note in two outlines |
| why `&` cannot mutate (a Vec reallocating under a reader), runtime vs compile time check, interior mutability, `RefCell`, borrow panic `already borrowed`, `try_borrow_mut`; `Cell` (for the curious) | 06-smart-pointers/08-interior-mutability | read quest |
| `RefCell::new`, `borrow()` / `borrow_mut()`, `Ref` / `RefMut`, `*cell.borrow_mut() += 1`, borrow ends at `;` | 06-smart-pointers/09-refcell | count opens through `&self` |
| `try_borrow_mut`, `try_borrow`, `is_err` / `is_ok`, readers block a writer | 06-smart-pointers/10-busy-or-free | autosave asks first |
| `Rc<RefCell<T>>`, handle edits shared value, field write through `borrow_mut()`, type alias `type NoteRef = ...` | 06-smart-pointers/11-shared-and-changeable | a tree of notes |
| recursion over `Rc<RefCell>` children with a `for` loop, many readers at once | 06-smart-pointers/12-total-of-a-tree | word count of a branch |
| reference cycle, memory leak, `Weak<T>`, `Rc::downgrade`, strong vs weak count, `upgrade()` -> `Option<Rc<T>>`, `Rc::weak_count` | 06-smart-pointers/13-weak | a link that lets go |
| owning direction `Rc`, back-pointer `Weak`, `RefCell` on one field, `Weak::new()` | 06-smart-pointers/14-pointing-back-up | parent title for a breadcrumb |
| `while let Some(x) = ...`, walking up parents, `.reverse()`, one borrow per statement | 06-smart-pointers/15-climb-to-the-root | full breadcrumb, has_ancestor |
| `Rc::ptr_eq` (same value vs `==` equal), `retain` with `ptr_eq`, take the parent in its own `let` before `borrow_mut()` | 06-smart-pointers/16-remove-that-exact-one | detach one of two look-alike notes |
| ids into a `HashMap<u32, NodeRef>`; arenas / ids instead of `Rc<RefCell>` (for the curious); `Option::cloned`, `str::repeat`, `push_str`, `HashMap::retain`, `map[&key]` (named in TODOs) | 06-smart-pointers/17-boss-tree-editor | the outliner engine |
| thread, process, main thread, CPU core, output order varies run to run, when threads pay off | 07-concurrency/01-two-things-at-once | read quest: one cook vs several cooks |
| `std::thread`, `thread::spawn(\|\| ..)`, `JoinHandle<T>`, `handle.join()` returns `Result`, blocks until done | 07-concurrency/02-spawn-and-join | checksum a file on a second thread |
| `error[E0373]` closure may outlive the function, `move \|\| ..`, the closure owns what it uses, `'static` named | 07-concurrency/03-move-closures | a label built on a thread |
| start every thread before joining any, `Vec<JoinHandle<T>>`, join in order keeps input order | 07-concurrency/04-start-them-all | checksum all files in parallel |
| a panic ends only its own thread, `join()` gives `Err(..)`, `Result::ok()` to `Option` | 07-concurrency/05-a-crash-stays-put | average size survives an empty folder |
| channel, `mpsc::channel()`, `Sender` / `Receiver`, `tx.send(v)` moves the value, `rx.recv()` waits, `for x in rx`, channel closes when every `Sender` is gone | 07-concurrency/06-channels | stream names to a collector |
| multiple producer single consumer, `tx.clone()` per thread, `drop(tx)` so the channel closes; `mpsc::sync_channel(n)`, backpressure (for the curious) | 07-concurrency/07-many-senders | workers report to one receiver |
| `+=` is read-add-write, lost update, data race (prevented at compile time), race condition, deadlock (not prevented) | 07-concurrency/08-data-races | read quest: two hands on one number |
| `Arc<T>`, atomically reference counted, `Arc::new` / `Arc::clone` / `Arc::strong_count`, `error[E0277]` for `Rc` across threads, Arc is read-only | 07-concurrency/09-arc | one shared list read by workers |
| marker traits, `Send` (may move to a thread), `Sync` (may be shared by `&`), the Send/Sync table (`Rc`, `RefCell`, `Cell`, `Mutex`, `Arc`), `T: Send + Sync`, reading E0277 | 07-concurrency/10-send-and-sync | read quest |
| `Mutex<T>`, mutual exclusion, `lock()`, guard, `*guard += 1`, drop unlocks, self-deadlock (lock twice) | 07-concurrency/11-mutex | a counter and a log through `&` |
| poisoned lock, `PoisonError`, `lock().unwrap()` passes the panic on, `unwrap_or_else(\|e\| e.into_inner())` | 07-concurrency/12-poisoned-lock | read a counter after a crash |
| `Arc<Mutex<T>>` (thread-safe `Rc<RefCell<T>>`), one clone per thread, join then read | 07-concurrency/13-arc-mutex | count and gather from workers |
| `RwLock<T>`, read-write lock, `read()` / `write()`, `try_read()` / `try_write()`, writers wait for readers, Mutex vs RwLock | 07-concurrency/14-rwlock | a shared table of known hashes |
| `error[E0521]` borrowed data escapes, `thread::scope(\|s\| ..)`, `s.spawn`, scoped threads may borrow, joined at the end of the scope | 07-concurrency/15-scoped-threads | sum two halves of a slice |
| `error[E0499]` on two `&mut` halves, `split_at_mut(mid)` → `(left, right)`, `iter_mut()`, `*x *= ..` | 07-concurrency/16-split-at-mut | change both halves in place |
| crate dependency in `Cargo.toml`, rayon, thread pool, `use rayon::prelude::*`, `par_iter()` / `into_par_iter()` / `par_iter_mut()`, order kept, the closure must be Send/Sync | 07-concurrency/17-rayon | swap `.iter()` for `.par_iter()` |
| atomic (indivisible), `std::sync::atomic`, `AtomicUsize` / `AtomicBool`, `fetch_add`, `load`, change through `&self`, `Ordering::Relaxed` / `SeqCst`, atomics vs two related fields; Acquire/Release (for the curious) | 07-concurrency/18-atomics | a files-done counter with no lock |
| check-then-act race condition, `compare_exchange(false, true, ..)` → `Ok` / `Err`, success and failure orderings | 07-concurrency/19-claim-once | only one thread writes the report |
| `std::fs::read_dir`, directory entries, `entry.path()`, `PathBuf` vs `&Path`, `path.is_dir()`, recursion over subfolders, sort at the end | 07-concurrency/20-walk-a-folder | every file in a folder |
| hexadecimal, `0x` / `0b` literals, `_` digit separators, XOR `^` / `^=`, `wrapping_mul`, `u64::from(byte)`, `fs::read`, FNV-1a | 07-concurrency/21-hash-the-bytes | a fingerprint of bytes |
| `{:x}`, `{:016x}` zero padding, `path.display()` | 07-concurrency/22-print-in-hex | a sha256sum-style line |
| worker pool, job queue, `Arc<Mutex<Receiver<T>>>` read inside out, lock for one job only (`let job = rx.lock().unwrap().recv();`), `loop` / `break`, results channel | 07-concurrency/23-a-worker-pool | a few workers share many jobs |
| putting it together: pool in `thread::scope`, `&Progress` atomic shared by borrow, sort for deterministic results, rayon as a cross-check | 07-concurrency/24-boss-parallel-file-hasher | the parallel file hasher |
| CPU work vs waiting on I/O, thread cost (2 MB stack, OS switch), future, task, runtime, tokio (read) | 08-async/01-waiting-is-not-working | 40 episodes, mostly waiting |
| `async fn`, future, `.await`, `error[E0277]` future isn't Display, `error[E0728]` await outside async, `#[tokio::test]` (named); `impl Future<Output = T>` (for the curious) | 08-async/02-a-future-is-a-ticket | `episode_file` / `describe` |
| runtime, `tokio::runtime::Runtime::new`, `block_on`, attribute macro `#[tokio::main]` / `#[tokio::test]`, worker threads, "Cannot start a runtime from within a runtime" | 08-async/03-who-runs-the-future | `describe_now` from plain code |
| lazy futures, poll, waker, unused-future warning; state machine (for the curious) | 08-async/04-futures-are-lazy | read quest |
| `tokio::join!`, sequential awaits, `tokio::time::sleep`, `Duration`; `join_all` (for the curious) | 08-async/05-two-at-once | two size checks at once |
| blocking, giving the thread back, `std::thread::sleep` vs `tokio::time::sleep`, `tokio::fs` (named), `spawn_blocking` (named) | 08-async/06-dont-block-the-runtime | read quest |
| `tokio::spawn`, task, `JoinHandle<T>`, `async { }` / `async move { }` block, `'static`, `Send`, spawn first then await | 08-async/07-spawn-a-task | `spawn_file_name`, `all_sizes` |
| panic stays in its task, `Err(JoinError)`, `.ok()` on a handle, `is_panic` (named) | 08-async/08-when-a-task-panics | `part_size` |
| async closure `async \|x\| ..`, `async move \|x\| ..`, `impl AsyncFn(u32) -> String`, block vs closure, `\|x\| async move {}` older spelling | 08-async/09-a-closure-that-waits | `labelled`, `labeller` |
| `tokio::sync::mpsc::channel(n)`, `tx`/`rx`, mpsc, bounded, backpressure, `tx.clone()`, `recv().await` → `None`, `drop(tx)` | 08-async/10-tasks-that-talk | `drain`, `gather` |
| `tokio::time::timeout`, `Err(Elapsed)`, dropping a future cancels it | 08-async/11-give-up-in-time | `with_deadline` |
| `tokio::select!`, `pattern = fut => value` branches, losers dropped; `biased;` (for the curious) | 08-async/12-first-one-wins | `first_answer` |
| `tokio::sync::oneshot`, receiver is a future, `&mut rx` in `select!` | 08-async/13-a-cancel-button | `download_or_cancel` |
| dropped future stops at its next await and undoes nothing, `.part` files, spawned tasks outlive handles; cancel safety (for the curious) | 08-async/14-which-future-wins | read quest |
| `tokio::sync::Mutex`, `lock().await`, guard across `.await`, why a std guard can't (not `Send`, blocks the worker), lost update, `yield_now` (*starter*) | 08-async/15-one-counter-many-tasks | `record_done`, `finish_all` |
| `tokio::sync::RwLock`, `read().await` / `write().await`, `.cloned()` out of a guard | 08-async/16-many-readers-one-writer | `TitleCache` |
| stream, `StreamExt`, `stream::iter`, `next().await`, `filter` with `ready(..)`, `take` on an endless stream; `fold` (for the curious) | 08-async/17-a-stream-of-chunks | `big_episodes_in_mb`, `first_n_chunk_offsets` |
| `tokio_util::sync::CancellationToken`, `cancel` / `cancelled`, loop around `select!`, cancel then await handles | 08-async/18-stop-every-task | `spawn_worker`, `shutdown` |
| HTTP request/response, method GET/POST, path, status 200/404/500/503, 4xx vs 5xx, header, body, JSON object/array/string/number/null, (de)serialize (read) | 08-async/19-requests-and-json | read quest |
| `reqwest::Client`, `client.get(url).send().await?`, `error_for_status`, `.json()`, `serde_json::Value`, `v["key"]`; `.bytes()`, `.text()` (named) | 08-async/20-ask-a-server | `fetch_json` |
| serde, `#[derive(Deserialize)]`, `.json::<Vec<T>>()`, missing field is Err, unknown fields skipped; `Option` field, `#[serde(rename)]` (for the curious) | 08-async/21-json-into-a-struct | `fetch_feed` |
| `tokio::sync::Semaphore`, permit, `acquire_owned` on `Arc<Semaphore>`, `let _` vs `let _permit` | 08-async/22-at-most-n-at-once | `download_limited` |
| `tokio::task::JoinSet`, `spawn`, `join_next` in finish order, `abort_all`, `is_cancelled`, dropping a set aborts | 08-async/23-a-set-of-tasks | `finish_order`, `first_and_cancel_rest` |
| `type Error = Box<dyn Error + Send + Sync>`, `error[E0282]` for `?` in an async block, `Ok::<T, Error>(..)`, `result??` | 08-async/24-errors-from-a-task | `total_size` |
| boss: `reqwest::get` shortcut, `tokio::fs::write` / `rename`, `.part` then rename, `Path::join`, `to_path_buf`, `std::fs::metadata` / `len`, `let _ = progress.send(..)` (named in cards/TODOs) | 08-async/25-boss-mod-downloader | the episode / mod downloader |
| bit, binary, place values, byte (8 bits, 0-255), `u8` = one byte, `u16`/`u32` sizes in bytes, `0b` literals, `_` digit spacer (read) | 09-systems/01-bits-and-bytes | a server answers in bytes |
| hexadecimal, `0x` literals, two hex digits = one byte, `{:x}` / `{:02x}` / `{:02X}` (refresher of 07's hex printing) | 09-systems/02-hex-dump | print a reply like protocol docs do |
| endianness, big-endian / little-endian, network byte order, why both exist, `to_be_bytes` / `to_le_bytes`, `from_be_bytes` / `from_le_bytes` | 09-systems/03-which-byte-first | a length that reads as 738 million |
| array vs slice, `slice.get(..4)` -> `Option<&[u8]>`, `try_into()` slice -> array (`Result`), `.ok()?` chain; `TryFrom` / `TryInto`, `u8::try_from` (for the curious) | 09-systems/04-the-first-four-bytes | read a u32 from a short reply |
| `const`, flag, mask, `\|` OR, `&` AND, `!` NOT on integers, `x & mask == mask`, `count_ones()`; `&` vs `!=` precedence vs C (for the curious) | 09-systems/05-flags-in-a-byte | status bits online/full/maintenance |
| `<<` / `>>` shifts, `>>=`, widen with `u32::from(u8)`, `as u8` keeps the low 8 bits | 09-systems/06-shift-and-pack | four bytes of an IPv4 address in one u32 |
| cursor (slice + position), check before moving, `&self.data[a..b]`, `Ok(self.take(1)?[0])` | 09-systems/07-a-cursor | read a reply front to back |
| header, building `[b[0], b[1]]` after `take`, `?` inside a struct literal (fields in written order) | 09-systems/08-read-a-header | an 8-byte header |
| `unsafe` block, what it unlocks (raw pointer, `unsafe fn`, C calls; `static mut`, `unsafe impl` named only), `# Safety` docs, `// SAFETY:` comments, safe wrapper, undefined behaviour (read, aside) | 09-systems/09-the-unsafe-contract | quiz |
| raw pointer `*const T` / `*mut T`, references as checked pointers, `*p` in `unsafe`, null / dangling / unaligned, `is_null`, `std::ptr::null` (tests) | 09-systems/10-through-a-raw-pointer | write a count through an address |
| slice = address + length, `as_ptr`, `std::slice::from_raw_parts`, safe function over an unsafe one | 09-systems/11-a-slice-from-parts | sum from raw parts |
| C, the C standard library, FFI, `unsafe extern "C" { fn ...; }`, `c_int` (`std::ffi`), every C call is unsafe; std's `TcpStream::connect` ends in C `connect` (for the curious) | 09-systems/12-call-into-c | `c_abs` |
| C strings, NUL `\0`, `c_char`, `CString::new` (fails on interior NUL), `as_ptr` lifetime, `CStr::from_ptr`, `to_string_lossy` -> `Cow<str>`, `into_owned`, `as_bytes_with_nul` | 09-systems/13-c-strings | `c_strlen`, `from_c` |
| client, server, IP address, `127.0.0.1` / localhost, port (`u16`, port 0 = any free one), TCP connection, stream not messages, UDP datagram, socket, `TcpListener` / `TcpStream` / `UdpSocket` named (read) | 09-systems/14-ports-and-connections | quiz |
| `TcpStream::connect`, `SocketAddr`, `write_all` vs `write`, `read_exact`, `Read` / `Write` traits in scope, `vec![0u8; n]`, byte string `b"..."` | 09-systems/15-connect-and-send | `client_roundtrip` |
| `read` returns `n`, only `buf[..n]` is real, `Ok(0)` = closed, `loop { }` with `return` / `break`, echo server | 09-systems/16-echo-until-closed | `handle` |
| `TcpListener::bind`, `accept()` blocks and gives `(stream, addr)`, a thread per connection (`thread::spawn(move ..)`, `JoinHandle::join`, refresher of world 07); async for thousands (for the curious) | 09-systems/17-one-thread-per-client | `serve` |
| `UdpSocket::bind`, `send_to`, `recv_from` -> `(n, from)`, reply to the sender, `to_ascii_uppercase` / `to_vec` on byte slices | 09-systems/18-datagrams | `serve_once`, `request` |
| lost datagrams, `set_read_timeout(Some(..))`, `std::time::Duration`, `from_millis` / `from_secs`; TCP `set_write_timeout`, `TcpStream::connect_timeout` | 09-systems/19-dont-wait-forever | `request` gives up after 500 ms |
| VarInt / LEB128, 7-bit groups + continuation bit `0x80`, `& 0x7f`, `\|= 0x80`, `value as u32` for negatives (sign bit shifts in), at most 5 bytes | 09-systems/20-seven-bits-at-a-time | `encode_varint` |
| VarInt decode, `<< (7 * i)`, `\|=` into a result, `result as i32`, returning bytes used, `for (i, &b) in ...enumerate()` (`&b` pattern copies the byte) | 09-systems/21-read-a-varint-back | `decode_varint` |
| hostile input, `attempt to shift left with overflow` panic, length limit check before use | 09-systems/22-a-broken-varint | `TooLong` |
| JSON (object, array, string, number, bool, null), serde_json crate, `serde_json::from_str`, `serde_json::Value`, `v["a"]["b"]` (missing = `Null`), `as_u64` / `as_i64` / `as_str`, `Option::ok_or`; `ok_or_else` (for the curious), raw string `r#"..."#`; typed serde named for world 10 | 09-systems/23-read-the-json | `players_online` |
| `Value` is an enum (`Null`, `Bool`, `Number`, `String`, `Array`, `Object`), `match &value`, borrowed parts in patterns, missing vs wrong shape | 09-systems/24-a-list-in-the-json | `sample_names` |
| packet framing `[VarInt len][VarInt id][payload]`, protocol String, `extend_from_slice`, `(1..=MAX).contains(&len)`, `io::Cursor` + `position()`, `io::Error::new(InvalidData)` (*starter* `invalid`), recursion over text components | 09-systems/25-boss-minecraft-server-pinger | the Server List Ping client |
| database, table, column, row, `INTEGER PRIMARY KEY`, `NOT NULL`, `TEXT`/`INTEGER`, SQL, `CREATE TABLE` / `INSERT` / `SELECT` / `UPDATE` / `DELETE`, `WHERE`, `ORDER BY` (`DESC`), SQLite, 0/1 for bools (read) | 10-apps/01-what-a-database-is | why a database instead of a text file |
| rusqlite, `Connection::open_in_memory`, `Connection::open(path)`, connection, `conn.execute(sql, [])`, `rusqlite::Result`, the `bundled` first build | 10-apps/02-your-first-table | create the quests / mods table |
| SQL injection, placeholder `?1` `?2`, `params![..]`, `last_insert_rowid()`, why never `format!` values into SQL | 10-apps/03-never-glue-values-in | "Rust's traits" / "Xaero's Minimap" breaks a glued insert |
| `query_row(sql, params, \|row\| ..)`, `row.get(0)` (columns count from 0, type from the target), `COUNT(*)`, no row = `Err(QueryReturnedNoRows)` | 10-apps/04-read-one-row | title by id, count by xp / enabled |
| `conn.prepare(sql)`, statement, `let mut stmt`, `stmt.query_map`, `row.get(n)?` inside `Ok(Struct { .. })`, collect into `Result<Vec<_>>`, `ORDER BY` is needed for an order | 10-apps/05-read-every-row | quest list, biggest XP first / A to Z |
| transaction, `conn.transaction()` (needs `&mut Connection`), `tx.commit()`, drop = rollback, `CHECK (..)`, speed of one commit; `tx.prepare` once (for the curious) | 10-apps/06-all-or-nothing | a whole world / modpack or nothing |
| `PRAGMA user_version`, schema version, migration steps `if v < N`, `ALTER TABLE .. ADD COLUMN .. DEFAULT`, `tx.pragma_update`, pragma, `commit()` as last line | 10-apps/07-upgrade-old-files | version-1 files gain a done / favourite column |
| JSON: object, array, string, number, `true`/`false`, `null`, keys in double quotes, mapping to struct/Vec/Option, serialize / deserialize, serde (read) | 10-apps/08-what-json-is | the Rust engine and the JavaScript screen are two programs |
| `#[derive(Serialize, Deserialize)]`, serde `derive` feature, `serde_json::to_string(&v)`, `serde_json::from_str(text)` (target type from return type / `let` / turbofish), `serde_json::Result`, `None` = `null`, round-trip test | 10-apps/09-structs-to-json | Profile / ModpackProfile to JSON and back |
| `#[serde(default)]` (needs `Default`), attribute read by a derive, "missing field" error, per-struct (nested structs need their own) | 10-apps/10-old-saves-still-load | an old save with only xp / mod_count |
| `#[serde(rename_all = "lowercase")]`, `#[serde(rename = "type")]`, `type` is a keyword; `r#type` raw identifier (for the curious) | 10-apps/11-spell-it-their-way | "notrun" / "notinstalled", the key "type" |
| HTTP, client, server, request, response, method (`GET` `POST` `PUT` `DELETE`), path, body, status code (200, 201, 204, 400, 404, 405, 500), route, API, port, `localhost` (read) | 10-apps/12-what-a-request-is | the screen asks the engine for XP |
| axum, handler (`async fn`, runs on tokio), `&'static str` response, `Router::new().route(path, get(handler))`, free 404/405, testing with `oneshot` (no port), `TcpListener::bind` + `axum::serve` (named) | 10-apps/13-hello-axum | GET /health, GET /status |
| path gap `{xp}`, extractor, `Path(xp): Path<u32>`, a bad path is 400 before the handler runs, `Json(value)` response (needs `Serialize`) | 10-apps/14-read-the-path | GET /level/{xp}, GET /tier/{mb} |
| `Json(req): Json<T>` body extractor (needs `Deserialize`), 422 wrong shape / 400 not JSON, `Json` must be the last parameter | 10-apps/15-json-in | POST /xp-total, POST /total-size |
| handler returning `Result<Json<T>, (StatusCode, Json<ApiError>)>`, `(StatusCode, body)` pair, `StatusCode::BAD_REQUEST` / `NOT_FOUND` / `CREATED`, error body | 10-apps/16-say-no-with-a-status | an empty list is 400 |
| `State(state): State<AppState>`, `.with_state(..)`, state is cloned per request so `Arc` shares it, `AtomicU32` `fetch_add` / `load` (refresher of world 07), `(StatusCode::CREATED, Json(..))`, `get(..).post(..)` on one path | 10-apps/17-remember-between-requests | POST /xp, POST /downloads |
| `Connection` is `Send` not `Sync`, `Arc<Mutex<Connection>>` in `State`, `db.lock().unwrap()`, don't hold a std guard across `.await`, `fn internal(rusqlite::Error) -> StatusCode`, `.map_err(internal)?`, a `Vec` as a JSON array | 10-apps/18-share-the-database | POST and GET /quests, /mods |
| `rusqlite::OptionalExtension`, `.optional()` (no row = `Ok(None)`), `match` an `Option` into 404, `execute` returns rows changed, 204 vs 404; `map(Json).ok_or(..)` (for the curious) | 10-apps/19-not-found-is-404 | GET and DELETE /quests/{id}, /mods/{id} |
| frame, event, event loop, widget, toolkit, retained mode, immediate mode (egui), why immediate mode tests easily, webview, sandbox, Tauri (read) | 10-apps/20-how-a-screen-is-drawn | how Oxide / the mod manager could draw a screen |
| view model (text from plain functions); `if` as a value, `filter(\|(_, d)\| *d).count()` (refreshers) | 10-apps/21-card-text | `[x] Loops`, `2 of 3 quests done` |
| `fn ui(&mut self, ui: &mut egui::Ui)`, `ui.label`, `ui.button(..).clicked()`, `Response`, `ui.horizontal(\|ui\| ..)`, egui_kittest `Harness`, `get_by_label`, `.click()`, `harness.run()` | 10-apps/22-a-screen-you-can-test | daily goal / render distance picker |
| `f32`, `as f32`, `f32` division by zero gives infinity / NaN (no panic), `.clamp(0.0, 1.0)`, `egui::ProgressBar::new`, `ui.add(widget)` | 10-apps/23-fill-the-bar | XP bar / download bar |
| `error[E0502]` reading `self.list` in a loop while calling a `&mut self` method, "click now, act after": a local `Option<String>`, `clone()`, act after the loop | 10-apps/24-click-then-act | open a quest / select a mod |
| `egui::Modal::new(egui::Id::new(..)).show(ctx, \|ui\| ..)`, `ui.heading`, local close flag, `response.should_close()`, `egui::Context` / `ui.ctx().clone()` | 10-apps/25-a-pop-up | Level up! / Tier up! |
| webview sandbox, command, JS `invoke("name", { .. })`, `#[tauri::command]`, arguments by name through serde, `Result<T, String>` rejects the promise, `.manage(..)`, `tauri::State<'_, Mutex<T>>`, `.invoke_handler(tauri::generate_handler![..])`, thin wrappers, `tauri::test::mock_builder`, commands stay private | 10-apps/26-tauri-commands | award_xp / add_download |
| dev vs release profile, `cargo build --release`, `[profile.release]` `lto` `codegen-units` `strip`, `cargo tauri build` installers, `dirs::data_dir()`, `create_dir_all`, app data never next to the exe (read) | 10-apps/27-shipping | ship it to a friend |
| `trait Api` as the seam, a scripted `FakeApi`, state methods vs `_ui` methods, `ui.separator()`, `ui.input(\|i\| i.key_pressed(egui::Key::Escape))` | 10-apps/28-boss-oxide-in-rust | Oxide's home / quest screens (games: the mod manager's home / pack screens) |
| macro (refresher), `macro_rules!`, arm, pattern => template, `$x:expr`, expression, a pasted expression stays one; why a macro and not a function | 11-mastery/01-your-first-macro | `square!` for whole numbers and decimals |
| side effect, a template pastes code not values, `{{ let v = $x; ... }}` (why two pairs of braces), hygiene | 11-mastery/02-pasted-twice-runs-twice | `square!(next())` ran twice |
| repetition `$( ... ),*` in pattern and template, `*` vs `+`, separators | 11-mastery/03-any-number-of-arguments | `strings![..]` |
| `$(,)?`, `?` = zero or once, `rustfmt`, "unexpected end of macro invocation" | 11-mastery/04-a-trailing-comma | starter fails to compile |
| literal tokens in a pattern (`=>`, `;`), only `=>` `,` `;` may follow an `expr` capture | 11-mastery/05-a-word-in-the-pattern | `item!("apples" => 3)` |
| template names are looked up at the call site, `$crate::`, leading `::std::`, E0422 / E0433 from a macro | 11-mastery/06-paths-that-work-anywhere | starter fails to compile |
| several captures per repetition round, a macro calling another macro, unused captures are dropped | 11-mastery/07-a-registry-macro | `stock![..]` / `registry![..]` |
| recursion, base case, arms tried top to bottom, expansion happens at compile time, 128-level limit | 11-mastery/08-a-macro-that-calls-itself | `max!` / `tallest!` |
| token, token stream, parse, syntax tree, procedural macro, syn, quote | 11-mastery/09-code-as-tokens | read quest |
| derive / attribute / function-like proc macros, why a macro crate of its own, `proc-macro = true`, workspace, `target` folder, `path` dependency | 11-mastery/10-a-crate-for-the-macro | read quest |
| `#[proc_macro_derive]`, `parse_macro_input!`, `DeriveInput` (`ident`, `generics`, `data`), `quote!`, `#name` splicing, `proc_macro2` vs `proc_macro` TokenStream, `.into()`, E0599 | 11-mastery/11-write-a-derive | starter fails to compile |
| impl generics vs type generics, where clause, `split_for_impl`, E0107 | 11-mastery/12-generics-in-a-derive | starter fails to compile |
| `#[proc_macro_attribute]` (args, item), `ItemFn` (`attrs`, `vis`, `sig`, `block`), syn `full` feature, `#(#attrs)*`, a `static` atomic counter (refresher of 07) | 11-mastery/13-wrap-a-function | `#[counted]` |
| `format_ident!`, `Ident` vs `String` in `quote!`, uppercase statics, E0428 | 11-mastery/14-a-name-per-function | starter fails to compile |
| target, WebAssembly, `wasm32`, `target_arch`, `target_os`, `#[cfg(..)]`, `not(..)`, `#[cfg(test)]` explained | 11-mastery/15-only-on-some-targets | starter fails to compile |
| `#[cfg_attr(..)]`, wasm-bindgen, JS glue, `crate-type` cdylib / rlib, wasm-pack, target-specific dependencies | 11-mastery/16-wasm-ready-crate | `word_count` / `chunk_index` |
| `#![no_std]`, `core`, `#[cfg(test)] extern crate std;`, const generic `const N: usize`, `[T; N]`, ring buffer, `%` wrap | 11-mastery/17-no-std-taste | `RingBuffer` / `LoadQueue` |
| release build, sampling profiler, hot spot, flamegraph (width is time), linear scan, O(n), O(n²), big-O | 11-mastery/18-read-profiling | read quest |
| allocator, `#[global_allocator]`, `GlobalAlloc`, `System`, `thread_local!`, `const { }` (named), `impl FnOnce() -> R` | 11-mastery/19-counting-allocations | `allocations_in` |
| capacity vs length, doubling growth (4, 8, ... 128), `Vec::with_capacity`, `String::with_capacity`, `join` sizes first | 11-mastery/20-reserve-once | `squares`, `join_words` |
| `entry(..).or_default()`, one lookup vs contains + get + insert, `entry` needs an owned key | 11-mastery/21-one-lookup-not-three | `word_count` with a `Counter` meter |
| hold `&T` while searching, clone only the result | 11-mastery/22-clone-the-winner | `longest` with a `Name` meter |
| `&str` slices are free, only `collect` allocates | 11-mastery/23-borrow-dont-copy | `count_long_words` |
| reuse a buffer: `clear` keeps capacity, `extend`, `flat_map(char::to_lowercase)`, `get_mut(buf.as_str())`, insert a clone only for a new key | 11-mastery/24-one-buffer-for-every-word | `count_words` / `tally_blocks` |
| `sort_unstable_by`, `then_with`, `take(n)`, `.cloned().unwrap_or_default()` (toolkit) | 11-mastery/25-boss-optimise-a-slow-crate | boss toolkit card |
| the booth: Stats API (TCP :49123), BakkesMod SOS, HUD, scorebug, stinger, OBS browser source, cue, OSC, Notch, Resolume, Companion, Stream Deck, producer, `triggers.json` (read) | 12-broadcast/01-the-booth | a picture of the path game → hud-server → outputs |
| frame, `Deserializer::from_slice(..).into_iter()`, `next()` → `Some(Ok)`/`Some(Err)`/`None`, `byte_offset`, `is_eof`, `loop` + `match` with `return` arms, `drain(..n)` (*starter*) | 12-broadcast/02-frames-on-the-wire | `split_frames`; example sums a stream of scores |
| an envelope struct, `#[serde(alias = ..)]`, `serde_json::from_value(..).ok()?`, `match` on a `&str` with a catch-all name, `Other(String)`; `stats_api_name` (*starter*) | 12-broadcast/03-one-event-shape | `normalize`; example picks a team from a colour |
| tokio `TcpStream::read(&mut chunk).await`, `Ok(0)` as closed, or-pattern `Ok(0) \| Err(_)`, `&chunk[..n]`, `mpsc::Sender::send(..).await?`; backoff (*starter*) | 12-broadcast/04-connect-and-retry | `read_frames`; example counts bytes |
| let-chains `if let .. && ..`, `*` on borrowed tuple parts, `saturating_sub` on `Duration`, the feed's clock (`now` passed in) | 12-broadcast/05-one-goal-one-trigger | `GoalDedup::is_repeat`; example on a roster |
| `Option::is_some_and` | 12-broadcast/06-grace-period | `Grace::allows`; example on the fastest goal |
| compare-then-remember (edge), fire-once armed flag, step function | 12-broadcast/07-thirty-seconds-left | `MomentTracker::step`, UpdateState arm |
| per-player count with `entry().or_insert(0)` and `== 3`, array indexed by team (`team as usize`, `.min(1)`), one `match` arm per event | 12-broadcast/08-count-the-moments | GoalScored / Demolish arms |
| `#[serde(tag = "kind")]`, `#[serde(untagged)]`, `serde_json::Error::line` with `map_err` | 12-broadcast/09-trigger-config | `load_triggers` |
| OSC: address, type tags, padded strings, `String::from(",")` + `push(match ..)`, `to_be_bytes`, `extend_from_slice`, datagram; `UdpSocket::send_to` (*starter*) | 12-broadcast/10-osc-by-hand | `encode`; example builds a team-letter string |
| axum `WebSocket`, `broadcast` vs `mpsc`, `loop { match rx.recv().await }`, `RecvError::Lagged` / `Closed`, `continue`, `Message::Text(text.into())` | 12-broadcast/11-overlay-socket | `forward`; example logs updates |
| `csv::Reader::from_reader`, `headers()`, `records()`, `position` by header name, `ok_or_else`, a lookup closure, `record.get(i).unwrap_or("")` | 12-broadcast/12-roster-from-a-sheet | `read_roster` |
| edit distance (Levenshtein), dynamic programming, the table, three neighbours, two rows (read) | 12-broadcast/13-edit-distance | `fen` / `fan` table by hand |
| `(0..=n).collect()`, two-row loop (`previous` / `row`), `x.min(y).min(z)`, `usize::from(bool)`, `chars()` into `Vec<char>` | 12-broadcast/14-names-vs-files | `levenshtein`; example builds Pascal's triangle |
| RGBA, alpha, transparent, `pixel[3]`, `enumerate_pixels()` → `(x, y, pixel)`, running min/max with `u32::MAX` / `0` start | 12-broadcast/15-the-visible-box | `visible_box`; example earliest/latest goal time |
| `imageops::crop_imm(..).to_image()`, `right - left + 1`, `match` on an `Option` box; `square`, `cut`, `par_iter` `cut_folder` (*starter*) | 12-broadcast/16-player-cuts | `trim_transparent` |
| tokio vs rayon vs `std::thread`, crossbeam channels, smol (read) | 12-broadcast/17-tokio-or-rayon | quiz |
| `tokio::task::spawn_blocking`, the handle's `Result` (Err = panic), `Send + 'static` closures, `move` | 12-broadcast/18-off-the-runtime | `off_the_runtime`; example reads a file |
| Electron vs Tauri/WebView2, `#[tauri::command]` with `State<Mutex<_>>`, `lock()` + `clone()`, poisoned locks with `unwrap_or_else(\|p\| p.into_inner())` | 12-broadcast/19-tauri-commands | `hud_state` |
| event vs command, polling, `Emitter::emit`, `tauri::Result`, `Listener::listen_any`, `AppHandle<R: Runtime>` | 12-broadcast/20-tauri-events | `publish_moment`; example emits a score |
| golden files, `Vec::get` past the end, `match` on a tuple of `Option`s with a guard | 12-broadcast/21-replay-check | `diff_lines`; example lines up two teams |
| semver with a build number, update channels, updater signatures (read) | 12-broadcast/22-update-channels | quiz |
| a step function joining feed, gates, moments and triggers | 12-broadcast/23-boss-hud-server | `Pipeline::step` |
