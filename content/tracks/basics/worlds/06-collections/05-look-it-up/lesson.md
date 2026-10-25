The till should turn a scanned name into its price: `"milk"` into 1.2. You could keep a list and walk all of it every time, but there is a better shape: a table that you look up by name.

**Keys instead of numbers.** `local prices = {milk = 1.2, bread = 2}` stores values under names instead of slot numbers. Each name is a **key**, and what it holds is its **value**. A table used this way is a **map**, also called a dictionary or a lookup table.

**Look up.** `prices["milk"]` gives 1.2: square brackets with the key in them, like an index but with text. The key can come from a variable, so `prices[item]` looks up whatever name `item` holds right now.

```lua
local seats = {front = 20, middle = 40, back = 30}
local zone = "middle"
local free = seats[zone]       -- 40
local none = seats["roof"]     -- nil: there is no such key
```

**Missing is nil.** Looking up a key that is not there gives `nil`. That is how you ask "is this item on the price list?": compare the lookup with nil. Rust answers a lookup with an `Option` that you must check, and Java with `null`; the question is the same.

**Why a map.** A list makes you walk through the items to find one; a map jumps straight to it, however many items there are. When you look things up by a name, use a map.

## Your turn

- In `price_of`, give back the price stored under `item`.
- In `sells`, give back true when `prices` has a price for `item`, and false when it does not.
- Save. The tests also ask for an item the shop does not sell.
