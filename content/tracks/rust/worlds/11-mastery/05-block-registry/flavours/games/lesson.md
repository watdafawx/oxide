**Why now:** the chunk indexer keeps a table of biomes: name and numeric id, dozens of rows. Writing `Biome { name: "plains", id: 1 }` forty times is noise. Two small macros turn the table into `"plains" => 1, "desert" => 2`, the shape of the data itself.

```rust
#[derive(Debug, PartialEq)]
pub struct Tag { pub label: &'static str, pub weight: u16 }

#[macro_export]
macro_rules! tag {
    ($label:expr => $w:expr) => {
        $crate::Tag { label: $label, weight: $w }     // $crate:: = "this crate", from anywhere
    };
}

// several pairs per round: both captures repeat together
macro_rules! total_weight {
    ($($label:expr => $w:expr),* $(,)?) => { 0 $( + $w )* };
}

let t = tag!("heavy" => 9);
let sum = total_weight!["a" => 1, "b" => 2,];            // 0 + 1 + 2
```

- A macro list expands into `vec![ ... ]` like in the last quest, and you can call another macro of yours inside the repetition with `$crate::name!(..)`.

- Any tokens that are not `$...` in a pattern must appear literally. Here `=>` separates name and id.
- A repetition can capture **several** things per round: both `$name` and `$id` repeat together.
- `$crate::` always resolves to your crate, wherever the macro is used.

## Your turn

In `src/lib.rs`: write `biome!` and `biomes!` (the `Biome` struct is given).
