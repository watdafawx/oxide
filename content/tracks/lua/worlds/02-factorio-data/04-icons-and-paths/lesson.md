**Why now:** `kit-alloy` still borrows the iron plate icon. A real mod ships its own pictures and tints vanilla ones, and both start with a path.

A file path in a prototype always begins with a **root** in double underscores, then forward slashes:

```text
__base__/graphics/icons/iron-plate.png       the base game
__kit-mod__/graphics/icons/kit-alloy.png     your own mod's folder
```

A backslash or a missing root is an error. For several pictures in one icon use `icons`, a list of layers drawn bottom to top; each layer needs `icon`, may set `icon_size` (default 64) and a `tint`:

```lua
icons = {
  { icon = "__kit-mod__/graphics/icons/kit-bolt.png" },
  { icon = "__base__/graphics/icons/copper-plate.png",
    tint = { r = 0.4, g = 0.8, b = 0.4, a = 0.7 } },
}
```

- In the starter, `icon = "graphics/icons/kit-alloy.png"` is broken because it has no root. Your own mod's root is its name between double underscores: this mod is `kit-mod`, so `__kit-mod__`.

- Use either `icon` or `icons` on one prototype, never an empty `icons`.
- A colour is `{r =, g =, b =, a =}` or `{r, g, b, a}`; values are 0 to 1. If any value is above 1 all of them are read as 0 to 255.
- Oxide checks the **shape** of the path, not whether the `.png` exists or looks good: only the game can tell you that.

## Your turn

- Open `mod/data.lua` and replace the broken `icon` with an `icons` list of two layers.
