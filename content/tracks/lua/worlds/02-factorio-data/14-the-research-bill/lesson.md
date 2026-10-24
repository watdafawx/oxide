A technology without a cost is not research, and the checker refuses `kit-metallurgy` until it has one. The bill says how many science packs a lab eats, and how long it takes.

**Paid in packs.** Labs research by using up science packs. In the prototypes a science pack has type `"tool"`: an item that wears out as it is used. Vanilla's red one is `automation-science-pack`.

**Units.** `unit` is the bill. Research happens in `count` units; each unit takes `time` seconds in a lab and uses up the listed packs once. 30 units at 1 pack each is 30 red packs.

**The short pair, here only.** `unit.ingredients` lists `{name, amount}` pairs with no `type`: `{"automation-science-pack", 1}`. Research can only eat science packs, so there is no item-or-fluid question to answer. This is the one place the short pair is still right; recipes reject it.

```lua
unit = {
  count = 50,
  time = 20,
  ingredients = { { "automation-science-pack", 2 } },
},
```

**For the curious.** A technology can instead be unlocked by doing something, such as crafting an item, with `research_trigger` in place of `unit`. And `count_formula` in place of `count` makes research that gets dearer each level.

## Your turn

- Give `kit-metallurgy` a `unit`: 30 units of 15 seconds.
- Each unit uses 1 automation science pack.
