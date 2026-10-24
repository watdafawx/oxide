The Kit Mod now has its own crafting-menu row, `kit-parts`, and a loop that moves every item whose name starts with `kit-` into it. A friend's add-on mod, `kit-extras`, adds a `kit-spring`. Save the starter: `kit-alloy` moves, `kit-spring` does not.

**Too early.** When your `data.lua` runs, `kit-extras` has not run yet, so `kit-spring` is not in `data.raw`. The loop is fine; it only looks too soon.

**Who goes first.** Factorio sorts mods before it runs them: a mod comes after every mod it depends on, and otherwise mods go by name. `kit-extras` depends on `kit-mod`, so it always comes after yours.

**Rounds.** The data stage is three rounds, and each round runs one file of every mod before the next round starts:

```text
round 1  data.lua               kit-mod, then kit-extras
round 2  data-updates.lua       kit-mod, then kit-extras
round 3  data-final-fixes.lua   kit-mod, then kit-extras
```

**Pick the round.** `data.lua` is for defining your own prototypes. By `data-updates.lua`, every mod's `data.lua` has run, so you can adjust what others defined. `data-final-fixes.lua` is the last word, for changes that must also see everyone's updates.

**Earliest that works.** Pick the earliest round that sees what you need. Other mods may want to adjust your changes in a later round, and a change pushed into final fixes leaves them no room.

## Your turn

- Move the loop from `mod/data.lua` into `mod/data-updates.lua`.
- Keep the `kit-parts` subgroup and the `kit-alloy` item in `data.lua`.
