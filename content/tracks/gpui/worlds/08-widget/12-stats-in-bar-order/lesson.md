The boss draws the whole widget, and its first step is "which stats does the bar show?". The bar's `order` mixes stats and dividers, and the widget has no use for dividers. `bar_stats` picks out just the stats, in the bar's order, so the widget lists them the way you arranged the bar.

**Keep some, change them.** You want fewer items than you started with, and different ones: `StatKind`s out of `Slot`s. `filter_map` from the Rust track does both in one step. Its closure gives back `Some(value)` to keep a value, or `None` to drop that item. See [Keep what parses](/quest/rust/03-collections/26-keep-what-parses).

**A match decides.** Inside the closure, a `match` on the slot answers both questions at once: a `Slot::Stat(kind)` keeps its kind, and a `Slot::Divider(_)` gives `None`. The `_` says "whatever style this divider has, ignore it".

**Why the star.** `order.iter()` lends each slot instead of handing it over, so the closure gets a `&Slot`, and matching on a reference gives `kind` as a `&StatKind`, a reference to the kind inside the slot. The list you build wants plain `StatKind` values. `*kind` reads the value the reference points at; since `StatKind` is `Copy`, that is a copy. The Rust track's [Follow the arrow](/quest/rust/02-ownership/09-follow-the-arrow) has the details.

**The other half.** Here the same shape keeps the dividers' styles instead of the stats. `collect()` builds the `Vec` because the `let` names the type:

```rust
let styles: Vec<DividerStyle> = order
    .iter()
    .filter_map(|slot| match slot {
        Slot::Divider(style) => Some(*style),
        Slot::Stat(_) => None,
    })
    .collect();
```

**Seen it already.** The widget's `render` in `widget.rs` does the same in one line. Now you know why each piece is there, and the boss can call your function instead.

## Your turn

- In `bar_stats`, go over `order` with `filter_map`, keeping each stat's kind and dropping the dividers.
- `collect()` the kinds and give them back. Save.
