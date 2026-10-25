Your scripts in the last world ran top to bottom the moment you saved. Factorio reads a mod at fixed moments instead, and what your code may do depends on which moment it is.

**A mod is a folder.** A mod is a folder of Lua files plus `info.json`, a small ID card with the mod's name and version. Factorio looks for files with fixed names inside it:

```text
kit-mod/
  info.json     the ID card
  data.lua      describes new things; runs at start-up (this world)
  control.lua   reacts to what players do; runs during play (a later world)
```

**Blueprints, not things.** At start-up the game runs every mod's `data.lua`. Each one adds **prototypes**: a prototype is a plain table that describes one kind of thing, like `{type = "item", name = "iron-plate", stack_size = 100}`. It is a blueprint, not the thing: one iron-plate prototype, millions of plates.

**The data stage.** This start-up round is the **data stage**. It fills one big table, `data.raw`, which sorts every prototype by type and then by name: `data.raw.item["iron-plate"]` is the iron plate's prototype. When the round ends, that list is frozen until you quit the game.

**Why freeze it.** Saves and every player in a multiplayer game refer to things by name. If a mod could invent items halfway through a game, two players could disagree about what exists. So every kind of thing is decided up front, and during play only counts and positions change.

**No game yet.** The other round, the **control stage**, runs `control.lua` while you play and reacts to events like "a player built a chest". During the data stage there is no map, no player and no `game` table: only data.

**Vanilla.** The game's own content is itself a mod, called `base`. Modders call it **vanilla**: the game as it ships, without mods. Your prototypes sit next to vanilla's in `data.raw`, and you can read or change vanilla ones too.

**What Oxide fakes.** Quests here run against `mock_factorio`, a small stand-in for the data stage: the same `data.raw`, a handful of vanilla prototypes, and a checker for the fields each prototype needs. It cannot see files or pictures, so the boss ends with an optional step in the real game.

## Your turn

Answer the quiz, then look inside `data.raw` yourself.
