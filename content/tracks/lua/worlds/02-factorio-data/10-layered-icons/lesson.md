Many modded items are a vanilla picture with a colour wash on top: it looks related to the original but you can tell them apart at a glance. Factorio builds that from layers instead of a new drawing.

**A stack of pictures.** `icons` (with an s) is a list of **layers**, each a table with its own `icon` path. The game draws them in list order, so the first is at the bottom and the last on top.

**One or the other.** When `icons` is there, the game ignores `icon`, so the old line only misleads whoever reads the file next: delete it. An empty `icons` list is an error.

**A tint.** A layer may carry a `tint`, a colour multiplied over its picture: `{r = 0.9, g = 0.5, b = 0.2, a = 0.8}`. Each channel (red, green, blue, and `a` for how see-through) runs from 0 to 1. Plain white, all ones, changes nothing.

**Big numbers.** If any channel is above 1, the game reads all of them as 0 to 255 instead. `{r = 255, g = 128, b = 0}` is the same orange as `{r = 1, g = 0.5, b = 0}`.

```lua
icons = {
  { icon = "__belt-extras__/graphics/icons/fast-belt.png" },
  { icon = "__base__/graphics/icons/transport-belt.png", tint = { r = 0.4, g = 0.8, b = 1, a = 0.6 } },
},
```

## Your turn

- Replace `kit-alloy`'s `icon` with an `icons` list.
- Layer 1: the Kit Mod's own picture. Layer 2: vanilla's iron plate with a tint that is not white.
