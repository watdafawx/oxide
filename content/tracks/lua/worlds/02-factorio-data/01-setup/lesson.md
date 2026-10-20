**Why now:** your recipe calculator reads recipes from a table you typed in. Factorio keeps the real ones in a table called `data.raw`, and a mod is just Lua that adds rows to it.

A mod is a folder. Factorio loads it in **stages**, each stage running every mod before the next one starts:

```text
settings.lua  settings-updates.lua  settings-final-fixes.lua
data.lua      data-updates.lua      data-final-fixes.lua      <- this world
control.lua                                                   <- world 3, during play
```

You register a new item, recipe or technology in `data.lua`. The data stage builds `data.raw[type][name]`: every item, recipe and technology in the game. There is **no `game` yet**, no map, no player: only the tables `data`, `mods` and `settings`. That is why a prototype is plain data and never code that runs later.

```lua
data:extend({
  { type = "item", name = "kit-alloy", icon = "__base__/graphics/icons/iron-plate.png", stack_size = 100 },
})
print(data.raw.item["kit-alloy"].stack_size)   --> 100
```

**What Oxide fakes.** Your quests run against `mock_factorio`, a small stand-in for the data stage: the same `data:extend` and `data.raw`, a handful of vanilla items, and a checker for each prototype's mandatory fields. It cannot see files (so it cannot check that an icon exists), has no graphics, and no control stage yet. The real game stays the judge: the boss ends with a step to load your mod in Factorio.

`pairs` order is not insertion order in plain Lua. Never depend on it, in the game or here.

## Your turn

Answer the quiz, then write your first `info.json`.
