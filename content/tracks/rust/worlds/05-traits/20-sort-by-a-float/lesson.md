The layers panel sorts by opacity, a decimal like 0.75. `max_by_key` and `sort` from Collections turn decimals down, and after the last quests you can see why.

**Try the obvious thing.** `layers.iter().max_by_key(|l| l.opacity)` stops the build:

```text
error[E0277]: the trait bound `f64: Ord` is not satisfied
```

**Ord is the total one.** **`Ord`** is the trait for types where any two values have an order: its `cmp` returns an `Ordering`, never `None`. `max_by_key`, `sort` and `BTreeMap` keys need it, and `f64` is only `PartialOrd`, because of NaN.

**A total order for decimals.** `a.total_cmp(&b)` compares two `f64`s and always answers, by putting NaN after every number. It returns a plain `Ordering`, exactly what a comparison needs.

**Bring your own comparison.** `max_by`, `min_by` and `sort_by` take a closure that gets two items, `|a, b|`, and returns their `Ordering`. Inside, compare the field with `total_cmp`.

```rust
struct Note {
    title: String,
    score: f64,
}

fn lowest(notes: &[Note]) -> Option<&Note> {
    notes.iter().min_by(|a, b| a.score.total_cmp(&b.score))
}
```

## Your turn

- Write `most_opaque`: the layer with the highest opacity, or `None` for no layers.
- Write `sort_by_opacity`: sort in place, lowest opacity first.
