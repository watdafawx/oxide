The ID card reads as JSON now, but Factorio would still refuse it: it does not say which version of the mod this is, which game it is made for, or what it needs.

**Four must-haves.** Factorio refuses a mod without `name`, `version`, `title` and `author`. `name` is the mod's internal ID (letters, digits, `-` and `_`, 4 to 49 characters for the portal); `title` is what players see.

**Three numbers.** `version` is three numbers with dots, like `"1.2.0"`, each at most 65535. You raise it with every release so the game and the portal can tell the new one from the old.

**Which game.** `factorio_version` names the game version the mod is made for, with **two** numbers: `"2.0"`. Leave it out and Factorio assumes the ancient `"0.12"` and refuses the mod; a third number is rejected by the portal.

**What it needs.** `dependencies` is a list of mods this one needs. Read `"base >= 2.0.0"` as "the mod `base`, version 2.0.0 or newer". A `?` in front makes one optional, and a `!` means "never together with this mod".

**Why name base.** Your recipes will use vanilla iron plates, so the Kit Mod needs `base`. A dependency also sets the loading order: Factorio loads a mod after the mods it depends on, so vanilla's prototypes already exist when your `data.lua` runs.

```json
{
  "name": "belt-extras",
  "version": "1.2.0",
  "factorio_version": "2.0",
  "dependencies": ["base >= 2.0.0", "? some-optional-mod"]
}
```

## Your turn

- Give `version` three numbers.
- Set `factorio_version` to the game this world targets.
- Make `dependencies` list `base` at version 2.0.0 or newer.
