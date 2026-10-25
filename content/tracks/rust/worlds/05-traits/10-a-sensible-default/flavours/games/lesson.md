Every new mob type needs stats and a drop table, and most mods never change them. **`Default`** is the standard trait for "the value you get when nobody says otherwise".

**One trait, one method.** `Default` has a single method, `fn default() -> Self`, with no `self` at all: it makes a value from nothing. You call it on the type, `MobStats::default()`.

**Derive it when zero is right.** `#[derive(Default)]` fills every field with that field's own default: `0` for numbers, `""` for `String`, `false` for `bool`, an empty `Vec`. A drop table with no xp and no items is exactly right for a new mob.

**Write it when zero is wrong.** A mob with 0 health dies the tick it spawns. Then you implement `Default` by hand, like any other trait, and return the value you mean.

```rust
#[derive(Debug, Default)]
struct Spawner {
    count: u32,                  // Spawner::default().count is 0
}

struct Light {
    level: u32,
}

impl Default for Light {
    fn default() -> Self {
        Light { level: 15 }      // full daylight, not darkness
    }
}
```

**Why a trait, not just new().** Generic code can ask for it without knowing your type. `Option::unwrap_or_default` on a `None` calls `MobStats::default()`, and a struct that holds `MobStats` can `derive(Default)` because `MobStats` has one.

## Your turn

- Add `Default` to the derive line on `Drops`, and return `Drops::default()` from `no_drops`.
- In `impl Default for MobStats`, return 20 health and 0 armor.
