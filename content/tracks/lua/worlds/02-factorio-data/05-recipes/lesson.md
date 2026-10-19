**Why now:** the calculator turns plates into parts. A recipe is exactly that, and in Factorio 2.0 each ingredient and result is a small table with a `type`.

Many tutorials online (before late 2024) use the old shape. In 2.0 this is the form:

```lua
{
  type = "recipe",
  name = "kit-alloy",
  energy_required = 2,                       -- seconds at crafting speed 1 (default 0.5)
  ingredients = {
    { type = "item", name = "iron-plate",   amount = 2 },
    { type = "item", name = "copper-plate", amount = 1 },
  },
  results = { { type = "item", name = "kit-alloy", amount = 1 } },
}
```

- `ingredients` and `results` are optional in the docs (an empty table means none) but a useful recipe has both.
- Each entry needs `type` (`"item"` or `"fluid"`), `name` and `amount` (a whole number from 1 to 65535).
- Optional: `enabled` (default true, so it is craftable from the start), `category` (default `"crafting"`), `main_product` (which result names and icons the recipe when there are several).
- A recipe with exactly one result borrows that item's icon and name. That is why nothing more is needed here.

## Your turn

- Open `mod/data.lua`. The item is done; the recipe still uses the old shape.
- Rewrite `ingredients` and `results`, and set the crafting time.
