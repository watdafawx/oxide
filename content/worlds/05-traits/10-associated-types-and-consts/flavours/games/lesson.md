**Why now:** every mob drops something different (a zombie drops a **count** of rotten flesh, a creeper drops an **item name**) and every mob has an id like `"zombie"`. The trait itself must name that drop type and id. That is what an **associated type** and an **associated const** are for.

```rust
trait Mob {
    const ID: &str;                        // associated const: one value per type
    type Loot;                             // associated type: one type per impl
    fn loot(&self) -> Self::Loot;
}

struct Pig;

impl Mob for Pig {
    const ID: &str = "pig";
    type Loot = u32;
    fn loot(&self) -> u32 { 3 }
}

fn first_drop<M: Mob>(m: &M) -> M::Loot { m.loot() }   // M::Loot is the u32 of Pig
// first_drop(&Pig) == 3,  Pig::ID == "pig"
```

- **`type Loot`**: each implementor picks its own return type.
- **`const ID`**: a value attached to the type, read as `Type::ID` or `M::ID`.
- In generic code, `M::Loot` means "whatever `M` chose".

## Your turn

- Open `src/lib.rs`.
- For `Zombie` and `Creeper`: set `ID` and write `loot`.
- Write `label` so it works for any `Mob` whose drop can be displayed.
