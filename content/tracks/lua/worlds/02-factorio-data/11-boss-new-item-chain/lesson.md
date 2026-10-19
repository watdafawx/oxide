**Why now:** you started with a calculator that knew a few recipes. Now the game's own tables hold yours: this boss puts every piece together, and `src/main.lua` runs your recipes back through a calculator to print what a kit-crate costs in plates.

The Kit Mod is one chain:

```text
iron-plate + copper-plate --recipe--> kit-alloy --4x + gear--recipe--> kit-crate (item) --place_result--> kit-crate (container)
automation (vanilla tech) --> kit-metallurgy (unlocks kit-alloy) --> kit-storage (unlocks kit-crate)
```

Everything lives in `mod/data.lua`. The alloy item and its recipe are written for you (objective 1 passes at the start). Add, in this order:

- the **kit-crate item** (stack 50, subgroup `storage`) and its **recipe** (4 kit-alloy + 1 iron-gear-wheel);
- the **container** with 48 slots, the item that places it, and `minable` back to the item;
- the second **technology**, `kit-storage`, after the first one;
- `enabled = false` on both recipes so research is the only way in.

Reminders: ingredients are `{type, name, amount}`; research `unit.ingredients` are `{name, amount}` pairs; a placeable container needs an icon and the `placeable-neutral` flag; copy shapes from earlier quests rather than from memory.

## Your turn

- Edit `mod/data.lua` until all eight objectives are green.
- Press Run on `src/main.lua` to see your prototypes and the recipe cost.

## Then load it in Factorio (optional, 5 minutes)

The tests cannot see graphics, balance or the real game's loader. To check with the real thing:

1. Copy the `mod/` folder into your Factorio `mods` folder and rename it `kit-mod_0.1.0`. A real `info.json` is already inside.
2. Start Factorio, enable Kit Mod in the Mods menu, load a new map.
3. Research Automation, then Kit Metallurgy, then Kit Storage. Craft a kit-alloy and a kit-crate, place the crate and open it.

The mod has no icon files of its own, so it borrows the base game's icons. The capstone quizzes you on what you saw.
