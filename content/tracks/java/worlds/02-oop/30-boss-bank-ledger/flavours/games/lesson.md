Your mod adds storage chests, and players *will* try to dupe items. Every kind of event must be handled, a refused one must leave no trace, and the counts must add up. This boss uses the whole world: a class with private fields, records, a sealed interface, a switch statement, and check-then-change.

**The chest.** A `Chest` has a name, a count per item id, and a log. You change it only by applying an **Event**, one of three records: `ItemPlaced(ItemStack stack)`, `ItemTaken(ItemStack stack)` and `ItemMoved(Chest to, ItemStack stack)`. They, `ItemStack` and `NotEnoughItemsException` are written.

**Seal it first.** `Event` is a plain interface in the starter. Seal it to the three records before you write any switch: the first objective turns green, and from then on the compiler counts your cases.

**The rules.** A placement adds the stack. A take removes it, but never more than the chest holds of that item: asking for more throws `NotEnoughItemsException`, with counts and log unchanged. A move is a take here plus a placement in `to`; the target logs an ordinary `ItemPlaced`, this chest logs the `ItemMoved`.

**No trace when refused.** Check before you change, as with the XP levels. The private helpers are written: `change(item, delta)` adds to an item's count (a negative delta takes away), and `record(event)` adds to the log. Call `record` after the switch, so a `throw` skips it. In a move, take from this chest first, so a refusal never reaches the target.

**The report.** One numbered line per event, oldest first, then the total number of items:

```text
1. placed 64 x cobblestone
2. took 16 x cobblestone
3. moved 8 x cobblestone to Storage B
items: 40
```

**Work in order.** The objectives go from easy to hard, and each one builds on the one before. The hints follow the same order.

## Your turn

- Seal `Event` to `ItemPlaced`, `ItemTaken` and `ItemMoved`.
- Write `Chest.apply`: a switch statement over the three kinds, then `record(event)`.
- Write `Chest.report()` with a `StringBuilder`.
