**Why now:** right now anyone can craft `kit-alloy` at minute zero. Progression in Factorio means research, so the recipe gets locked and a **technology** unlocks it.

A technology is a prototype with a research cost (`unit`), things it grants (`effects`) and the techs you need first (`prerequisites`):

```lua
{
  type = "technology",
  name = "kit-metallurgy",
  icon = "__base__/graphics/technology/steel-processing.png", icon_size = 256,
  prerequisites = { "automation" },
  effects = { { type = "unlock-recipe", recipe = "kit-alloy" } },
  unit = {
    count = 30,                                   -- how many units (or count_formula, never both)
    time = 15,                                    -- seconds per unit at lab speed 1
    ingredients = { { "automation-science-pack", 1 } },   -- {name, amount} pairs
  },
}
```

- `unit` needs `time`, `ingredients` and exactly one of `count` / `count_formula`. A tech may use `research_trigger` instead of `unit`, but not in this quest.
- Research ingredients must be `tool` items, the science packs. Careful: unlike recipe ingredients (`{type, name, amount}` tables), `unit.ingredients` are plain `{name, amount}` pairs.
- The recipe must start locked: `enabled = false`. Otherwise research unlocks something you already own.
- Names in `prerequisites` and `recipe` are looked up by name: a typo is an error in the game, and Oxide's reference check names it.

## Your turn

- Open `mod/data.lua`. The item and the recipe are there; finish the technology and lock the recipe.
