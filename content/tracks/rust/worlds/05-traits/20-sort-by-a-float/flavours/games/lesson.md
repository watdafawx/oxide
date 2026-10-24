The spawn list shows mobs sorted by speed, a decimal like 0.23 blocks per tick. `max_by_key` and `sort` from Collections turn decimals down, and after the last quests you can see why.

**Try the obvious thing.** `mobs.iter().max_by_key(|m| m.speed)` stops the build:

```text
error[E0277]: the trait bound `f64: Ord` is not satisfied
```

**Ord is the total one.** **`Ord`** is the trait for types where any two values have an order: its `cmp` returns an `Ordering`, never `None`. `max_by_key`, `sort` and `BTreeMap` keys need it, and `f64` is only `PartialOrd`, because of NaN.

**A total order for decimals.** `a.total_cmp(&b)` compares two `f64`s and always answers, by putting NaN after every number. It returns a plain `Ordering`, exactly what a comparison needs.

**Bring your own comparison.** `max_by`, `min_by` and `sort_by` take a closure that gets two items, `|a, b|`, and returns their `Ordering`. Inside, compare the field with `total_cmp`.

```rust
struct Player {
    name: String,
    health: f64,
}

fn weakest(players: &[Player]) -> Option<&Player> {
    players.iter().min_by(|a, b| a.health.total_cmp(&b.health))
}
```

## Your turn

- Write `fastest`: the mob with the highest speed, or `None` for no mobs.
- Write `sort_by_speed`: sort in place, slowest first.
