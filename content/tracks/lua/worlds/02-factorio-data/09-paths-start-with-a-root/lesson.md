`kit-alloy` still borrows the iron plate's picture. A real mod ships its own, in a `graphics` folder inside the mod, and points at it with a path the game can find from any mod.

**A root first.** Every file path in a prototype starts with a **root**: a mod's name between double underscores, then the folders inside it, separated by forward slashes. `__base__` is vanilla's folder, so `__base__/graphics/icons/iron-plate.png` is vanilla's iron plate picture.

**Your own root.** Your mod's root is its `name` from `info.json` between double underscores. The Kit Mod is `kit-mod`, so its pictures start with `__kit-mod__/`. A path without a root means nothing to the game, because it does not say whose folder to look in.

```text
__base__/graphics/icons/iron-plate.png       vanilla's folder
__belt-extras__/graphics/icons/fast.png      the belt-extras mod's folder
```

**Shape, not picture.** Oxide checks the shape of the path. It cannot open the `.png` or see whether it looks right; only the game can tell you that. The boss borrows vanilla pictures again, because its mod folder has no `graphics` yet.

## Your turn

- Fix `kit-alloy`'s `icon` so it starts with the Kit Mod's root.
