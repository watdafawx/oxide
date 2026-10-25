Your item needed exactly two extra fields, and the checker knew which. Both you and the checker get that list from the same place: the prototype docs. Every page there follows one pattern, so you can read one in a minute.

**Pin the version.** Open `lua-api.factorio.com/2.0.55/`, then **Prototypes**, then the type you are writing, such as `ItemPrototype`. Pin the number: "latest" can be newer than your game.

**Mandatory or default.** Each property shows the kind of value it takes (`string`, `double` for a decimal number, or a named type with its own page) and then one of three things. A **default** is the value you get if you leave it out. "Optional" means you may skip it. Neither means it is **mandatory**: you must write it.

**Mandatory if.** "Mandatory if `icons` is not defined" means you must give this property unless the named one is there instead. That is why an item needs `icon` or `icons`, but not both.

**Inherits from.** At the top, a page says what it **inherits from**: the page whose properties this type also has. `ToolPrototype` inherits from `ItemPrototype`, so a science pack needs everything an item needs plus its own fields. `type` and `name` live at the top of that chain, on `PrototypeBase`.

**Read the default first.** Before writing a value, check what you would get anyway: `icon_size` is 64, a recipe's `energy_required` is 0.5, `enabled` is true. A field you never write is still there, with its default.

**Trust 2.0.** When Oxide says "missing mandatory field", it mirrors these pages. Tutorials older than late 2024 describe Factorio 1.1, and some of their shapes no longer load.

## Your turn

Answer the quiz, then give `kit-alloy` a place in the crafting menu.
