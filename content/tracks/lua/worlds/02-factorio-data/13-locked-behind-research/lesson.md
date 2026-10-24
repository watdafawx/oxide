Right now anyone can craft `kit-alloy` in the first minute of a new game. Progress in Factorio comes from research, so the recipe gets locked and a technology called `kit-metallurgy` unlocks it.

**Open by default.** Every recipe is craftable from the start unless it says `enabled = false`. Without that line, research would unlock something the player already has.

**What research does.** A technology's `effects` lists what finishing it changes. `{type = "unlock-recipe", recipe = "kit-alloy"}` switches one locked recipe on.

**What comes first.** `prerequisites` is a list of technology names that must be researched before this one. Vanilla's `automation`, the first research most players finish, is a good place to hang a new technology in the tree.

**Linked by name.** These links are plain strings. Lua cannot tell that `"kit-aloy"` is a typo, so the game only reports it when it loads, and Oxide's reference check names the broken link.

```lua
{
  type = "technology",
  name = "kit-welding",
  icon = "__base__/graphics/technology/steel-processing.png",
  icon_size = 256,
  prerequisites = { "steel-processing" },
  effects = { { type = "unlock-recipe", recipe = "kit-rod" } },
  unit = { count = 50, time = 20, ingredients = { { "automation-science-pack", 2 } } },
}
```

**Bigger pictures.** Technology pictures are 256 pixels square, while item icons default to 64, so a technology says `icon_size = 256`. The `unit` line is the research bill, already written here; the next quest is about it.

## Your turn

- Lock the `kit-alloy` recipe.
- Make `kit-metallurgy` need `automation` first.
- Make it unlock the `kit-alloy` recipe.
