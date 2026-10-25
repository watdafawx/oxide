`kit-alloy` exists, but nobody can make it. A recipe is the missing piece: it says what goes in, what comes out and how long it takes, just like the recipes in your calculator.

**In and out.** A recipe prototype has `ingredients`, the items that go in, and `results`, the items that come out. Both are lists, because most recipes take several things.

**One entry.** Each entry in either list is a small table: `{type = "item", name = "iron-plate", amount = 2}`. `type` is `"item"` or `"fluid"` (a liquid such as water), `name` names an existing prototype, and `amount` is a whole number from 1 to 65535.

**How long.** `energy_required` is the crafting time in seconds, by hand or in a machine at speed 1. Leave it out and you get the default, 0.5 seconds.

**Same name, no clash.** The recipe can be called `kit-alloy` like the item, because they live in different tables: `data.raw.recipe` and `data.raw.item`. A recipe with exactly one result borrows that item's icon, so it needs none of its own.

```lua
{
  type = "recipe",
  name = "kit-bolt",
  energy_required = 0.75,
  ingredients = {
    { type = "item", name = "iron-plate", amount = 3 },
  },
  results = { { type = "item", name = "kit-bolt", amount = 4 } },
}
```

**Old tutorials.** Guides written before Factorio 2.0 (October 2024) write ingredients as short pairs, `{"iron-plate", 2}`, and the product as `result = "..."`. Recipes in 2.0 reject both, and Oxide's checker says so if you try.

## Your turn

- Fill `ingredients` with 2 iron plates and 1 copper plate.
- Fill `results` with 1 `kit-alloy`.
- Make crafting take 2 seconds.
