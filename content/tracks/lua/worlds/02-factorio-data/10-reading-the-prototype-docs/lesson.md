**Why now:** the boss builds a whole mod and you will look up fields yourself. The prototype docs follow one pattern, so learn to read a page in 30 seconds.

Open `lua-api.factorio.com/2.0.55/` (pin the version, "latest" can be newer than your game), then **Prototypes**, then the type, for example `ItemPrototype`.

A page has three parts:

- **Properties** of this type. Each shows the type of the value (`double`, `string`, a named type such as `ItemStackIndex`) and either a default, or "Mandatory if ...", or nothing. **No default and not marked optional means mandatory.** "Mandatory if `icons` is not defined" means: you must give this property unless the named one is there instead. Example: on `RecipePrototype` the fields `ingredients` and `results` are optional and `energy_required` has a default of 0.5, so only `type` and `name` (inherited from the base) are mandatory.
- **Inherits from** at the top: `ToolPrototype` inherits `ItemPrototype`, which inherits `Prototype` and `PrototypeBase`. A tool needs everything an item needs, plus its own fields. Click up the chain to find `type`, `name`, `order`, `subgroup`.
- **Types** in the sidebar: `ItemIngredientPrototype`, `IconData`, `Color` are small structs with their own page of fields.

Habits that save time:

- Search the property name on the page (Ctrl+F `stack_size`), not the whole site.
- Read the **default** before writing a value: `icon_size` is 64, `energy_required` 0.5, `enabled` true.
- When Oxide says "missing mandatory field `x`", it mirrors this list; the real game's message is similar.
- The 2.0 docs are the truth. A tutorial older than late 2024 may show the old recipe format.

## Your turn

Answer the quiz, then build the boss: a chain of two items, two recipes, two technologies and a chest.
