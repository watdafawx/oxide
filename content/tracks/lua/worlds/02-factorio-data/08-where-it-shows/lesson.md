`kit-alloy` loads, but in the game it would sit in a catch-all "Other" tab at the far end of the crafting menu. Two optional fields decide where an item shows up.

**Tabs and rows.** The crafting menu is split into tabs, which the game calls **item groups** (Logistics, Production, Intermediate products), and each tab into rows, called **subgroups**. An item's `subgroup` names the row it shows in.

**The row for parts.** Vanilla's `"intermediate-product"` row holds gears, cables and circuits, the parts other recipes use. A subgroup name must belong to a real `item-subgroup` prototype, or the item has nowhere to go.

**Order in the row.** `order` is a string. Items in a row are sorted by comparing their order strings letter by letter, like words in a dictionary, so `"a..."` comes before `"b..."`.

**The bracket habit.** Vanilla writes orders like `"c[iron-gear-wheel]"`. The letter in front does the sorting; the part in brackets only tells people what it is and breaks ties. A `"z"` in front puts an item at the end of its row.

```lua
{
  type = "item",
  name = "kit-bolt",
  icon = "__base__/graphics/icons/copper-plate.png",
  stack_size = 200,
  subgroup = "intermediate-product",
  order = "y[kit-bolt]",
}
```

## Your turn

- Put `kit-alloy` in the intermediate-product row.
- Give it an order string that sorts after the vanilla parts.
