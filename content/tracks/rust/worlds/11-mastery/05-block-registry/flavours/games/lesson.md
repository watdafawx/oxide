**Why now:** the chunk indexer keeps a table of biomes: name and numeric id, dozens of rows. Writing `Biome { name: "plains", id: 1 }` forty times is noise. Two small macros turn the table into `"plains" => 1, "desert" => 2`, the shape of the data itself.

```rust
#[derive(Debug, PartialEq)]
pub struct Biome { pub name: &'static str, pub id: u16 }

#[macro_export]
macro_rules! biome {
    ($name:expr => $id:expr) => {
        $crate::Biome { name: $name, id: $id }
    };
}

#[macro_export]
macro_rules! biomes {
    ($($name:expr => $id:expr),* $(,)?) => {
        vec![ $( $crate::biome!($name => $id) ),* ]
    };
}

let table = biomes!["plains" => 1, "desert" => 2,];
```

- Any tokens that are not `$...` in a pattern must appear literally. Here `=>` separates name and id.
- A repetition can capture **several** things per round: both `$name` and `$id` repeat together.
- `$crate::` always resolves to your crate, wherever the macro is used.
- The macros are small, but the table at the call site now reads like a table.

## Your turn

In `src/lib.rs`: write `biome!` and `biomes!` (the `Biome` struct is given).
