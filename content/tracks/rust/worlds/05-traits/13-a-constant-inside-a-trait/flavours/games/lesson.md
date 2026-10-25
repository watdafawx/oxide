The `/summon` command and the loot log need each mob's id, like `minecraft:zombie`. That id belongs to the type, not to one mob: every zombie has the same one, and the command should not need a live zombie to ask.

**A value with no self.** An **associated const** is a constant inside the trait: `const ID: &str;`. Each `impl` sets its own, `const ID: &str = "minecraft:zombie";`, the same way it filled in `type Loot`.

**Read it from the type.** You write `Zombie::ID`, or in generic code `M::ID`. No `self` is involved, so you don't need a mob to ask.

```rust
trait Block {
    const HARDNESS: u32;
}

struct Obsidian;

impl Block for Obsidian {
    const HARDNESS: u32 = 50;
}

fn mine_time<B: Block>(tool_speed: u32) -> u32 {
    B::HARDNESS / tool_speed
}
// mine_time::<Obsidian>(5) is 10
```

**Why not a method?** `fn id(&self) -> String` would work too, but it needs a mob and builds a new `String` on every call. A const is fixed when the game is built and costs nothing to read.

## Your turn

- In both `impl`s, set `ID`: `minecraft:zombie` and `minecraft:creeper`.
- Write `spawn_line`: `<ID>: <loot>`.
