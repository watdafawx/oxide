The player right-clicks a villager and asks what bread costs. With a `Vec` you would walk every trade until the name matches. A **`HashMap`** jumps straight to it: you look things up by a name, not by a slot.

**Ask for it by name.** A `HashMap` is not loaded into every file the way `Vec` is. It lives in Rust's **standard library**, `std`, in its `collections` part. The first line `use std::collections::HashMap;` brings the name into this file, so you can write `HashMap` instead of the full path. The `::` walks the path, as in `Vec::new`.

**Keys and values.** Each entry is a **key** you look up by and a **value** you get back. The type names both: `HashMap<String, u32>` is "from String keys to u32 values", here prices in emeralds. A key appears at most once.

```rust
use std::collections::HashMap;

let mut chest = HashMap::new();
chest.insert("coal".to_string(), 40);   // add, or replace the old value
chest.get("coal")                       // Some(&40)
chest.get("elytra")                     // None
```

**Lookups can miss.** `get` answers with an `Option<&u32>`, the same as `get` on a `Vec`: `None` when the key is not there. `.copied()` again turns it into an `Option<u32>`.

**No order.** A `HashMap` keeps no order you can rely on. Printing one twice in different runs can list the keys differently. A later quest shows the map that keeps keys sorted.

## Your turn

- In `villager_prices`, make an empty map and insert the three trades.
- In `trade_price`, look the item up and give back an `Option<u32>`.
