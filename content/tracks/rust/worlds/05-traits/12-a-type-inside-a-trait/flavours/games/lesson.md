Every mob drops something different: a zombie drops a **count** of rotten flesh, a creeper drops an **item name**. The `Mob` trait has to let each mob say what type its loot is.

**Try a fixed type.** `fn loot(&self) -> u32` suits the zombie's count, but the creeper's drop is a name, a `String`. The trait can't pick one return type that suits every mob.

**Let each impl choose.** An **associated type** is a type slot inside the trait: `type Loot;`. Each `impl` fills it in, `type Loot = u32;`, and the methods name it as `Self::Loot`.

**Why not Mob<Loot>?** A generic trait could carry the type too, but then `Zombie` could implement `Mob<u32>` *and* `Mob<String>`, and every call would have to say which one it meant. An associated type means one answer per mob, so `z.loot()` needs no extra words.

```rust
use std::fmt::Display;

trait Breed {
    type Baby;                         // each impl picks one
    fn breed(&self) -> Self::Baby;
}

struct Cow;

impl Breed for Cow {
    type Baby = u32;
    fn breed(&self) -> u32 { 1 }
}

fn show<B>(b: &B) -> String
where
    B: Breed,
    B::Baby: Display,                  // a bound on the slot
{
    format!("born: {}", b.breed())
}
```

**In generic code.** In `fn drop_line<M: Mob>`, `M::Loot` means "whatever `M` chose". To print it, put a bound on it: `where M::Loot: Display`.

**You will see it everywhere.** `Iterator` has `type Item` and `Add` has `type Output`. The next quests use both.

## Your turn

- In `impl Mob for Zombie`, write `loot`: 2 rotten flesh plus one per looting level.
- In `impl Mob for Creeper`, write `loot`: `"head"` when charged, `"gunpowder"` otherwise.
- Write `drop_line`: `drops: <loot>`.
