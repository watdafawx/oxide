You started this track with a calculator that knew a few recipes you typed in. Now the game's own tables hold yours. This boss ships the whole Kit Mod, and `src/main.lua` runs your recipes back through a calculator to print what one kit-crate costs in plates.

**The chain.** Two items, two recipes, a chest and two technologies, each pointing at the next by name:

```text
iron-plate + copper-plate --recipe--> kit-alloy
4 kit-alloy + iron-gear-wheel --recipe--> kit-crate (item) --place_result--> kit-crate (container)
automation (vanilla) --> kit-metallurgy (unlocks kit-alloy) --> kit-storage (unlocks kit-crate)
```

**Already there.** Everything lives in `mod/data.lua`. The alloy item and its recipe are written, so the first objective is green from the start, and `kit-metallurgy` has its icon.

**One shape per quest.** Every piece is a shape you have written once in this world: the item and stack size, the recipe's `{type, name, amount}` entries, the item and container that point at each other, the technology's `effects` and `prerequisites`, and the research bill with its short `{name, amount}` pairs. Copy from those quests rather than from memory.

**Easy to forget.** Both recipes start locked with `enabled = false`. The new technology `kit-storage` needs its own `icon` and `icon_size`, just like the first one; copy them across.

**See it run.** Pressing Run on `src/main.lua` loads your mod through the mock and calls `print`, which writes a line of text to the output: your prototypes, then the raw cost of a kit-crate.

## Your turn

- Add the kit-crate item and its recipe, then the container.
- Add the `kit-storage` technology after `kit-metallurgy`.
- Lock both recipes and finish `kit-metallurgy`, until all eight objectives are green.

## Then load it in Factorio (optional, 5 minutes)

**Why bother.** The tests cannot see pictures, balance or the real game's loader. The real game is the final judge.

**Copy the folder.** Copy the `mod/` folder into your Factorio `mods` folder and rename it `kit-mod_0.1.0`. A real `info.json` is already inside.

**Play it.** Start Factorio, enable Kit Mod in the Mods menu and start a new map. Research Automation, then Kit Metallurgy, then Kit Storage; craft a kit-alloy and a kit-crate, place the crate and open it. The mod has no pictures of its own, so it borrows vanilla's.
