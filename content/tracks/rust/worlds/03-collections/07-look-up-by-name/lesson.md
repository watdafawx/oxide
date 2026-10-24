The till scans "mug" and needs its price. With a `Vec` you would walk every product until the name matches. A **`HashMap`** jumps straight to it: you look things up by a name, not by a position.

**Ask for it by name.** A `HashMap` is not loaded into every file the way `Vec` is. It lives in Rust's **standard library**, `std`, in its `collections` part. The first line `use std::collections::HashMap;` brings the name into this file, so you can write `HashMap` instead of the full path. The `::` walks the path, as in `Vec::new`.

**Keys and values.** Each entry is a **key** you look up by and a **value** you get back. The type names both: `HashMap<String, u32>` is "from String keys to u32 values", here prices in cents. A key appears at most once.

```rust
use std::collections::HashMap;

let mut stock = HashMap::new();
stock.insert("pen".to_string(), 12);   // add, or replace the old value
stock.get("pen")                       // Some(&12)
stock.get("sofa")                      // None
```

**Lookups can miss.** `get` answers with an `Option<&u32>`, the same as `get` on a `Vec`: `None` when the key is not there. `.copied()` again turns it into an `Option<u32>`.

**No order.** A `HashMap` keeps no order you can rely on. Printing one twice in different runs can list the keys differently. A later quest shows the map that keeps keys sorted.

## Your turn

- In `price_list`, make an empty map and insert the three prices.
- In `price_of`, look the product up and give back an `Option<u32>`.
