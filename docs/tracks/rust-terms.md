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
| `Deserializer::from_slice(..).into_iter()`, `byte_offset`, `is_eof`, `drain(..n)` | 12-broadcast/01-frames-on-the-wire | `split_frames` |
| an envelope struct, `#[serde(alias = ..)]`, `match` on a name, `Other(String)` | 12-broadcast/02-one-event-shape | `normalize` |
| tokio `TcpStream::read`, `Ok(0)` as closed, `mpsc::Sender::send(..).await?`; backoff (*starter*) | 12-broadcast/03-connect-and-retry | `read_frames` |
| let-chains over a remembered value, `saturating_sub` on `Duration` | 12-broadcast/04-one-goal-one-trigger | `GoalDedup::is_repeat` |
| `Option::is_some_and` | 12-broadcast/05-grace-period | `Grace::allows` |
| fire-once flags, `HashMap::entry` counts, arrays indexed by team | 12-broadcast/06-moments | `MomentTracker::step` |
| `#[serde(tag = "kind")]`, `#[serde(untagged)]`, `serde_json::Error::line` | 12-broadcast/07-trigger-config | `load_triggers` |
| OSC: padded strings, type tags, `to_be_bytes`; `UdpSocket::send_to` | 12-broadcast/08-osc-by-hand | `encode` |
| axum `WebSocket`, `broadcast::Receiver::recv`, `RecvError::Lagged` / `Closed` | 12-broadcast/09-overlay-socket | `forward` |
| `csv::Reader::headers`, `position` by header name, `ok_or_else` | 12-broadcast/10-roster-from-a-sheet | `read_roster` |
| Levenshtein with one row, `usize::from(bool)` | 12-broadcast/11-names-vs-files | `levenshtein` |
| `image::RgbaImage`, `enumerate_pixels`, `imageops::crop_imm`; `par_iter` (*starter*) | 12-broadcast/12-player-cuts | `trim_transparent` |
| tokio vs rayon vs `std::thread`, crossbeam channels, smol (read) | 12-broadcast/13-tokio-or-rayon | quiz |
| `tokio::task::spawn_blocking`, `Send + 'static` closures | 12-broadcast/14-off-the-runtime | `off_the_runtime` |
| `#[tauri::command]` with `State<Mutex<_>>`, poisoned locks with `into_inner` | 12-broadcast/15-tauri-commands | `hud_state` |
| `Emitter::emit`, `Listener::listen_any`, `AppHandle<R: Runtime>` | 12-broadcast/16-tauri-events | `publish_moment` |
| golden files, `Vec::get` past the end | 12-broadcast/17-replay-check | `diff_lines` |
| semver with a build number, update channels, updater signatures (read) | 12-broadcast/18-update-channels | quiz |
| a step function joining feed, gates, moments and triggers | 12-broadcast/19-boss-hud-server | `Pipeline::step` |
