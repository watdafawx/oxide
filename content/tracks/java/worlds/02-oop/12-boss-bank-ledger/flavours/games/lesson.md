**Why now:** your mod adds storage chests, and players *will* try to dupe items. Every kind of event must be handled, a rejected one must leave no trace, and the numbers must add up. This boss uses the whole world: classes and fields, encapsulation, records, sealed interfaces and pattern `switch`, on the same `Event` model you used in the last quests.

You are building a **chest with an audit log**. A `Chest` has a name and counts items by id. You change it by applying an **Event**:

```java
public sealed interface Event permits ItemPlaced, ItemTaken, ItemMoved {}
public record ItemPlaced(ItemStack stack) implements Event {}
public record ItemTaken(ItemStack stack) implements Event {}
public record ItemMoved(Chest to, ItemStack stack) implements Event {}
```

The rules:

- **ItemPlaced** adds the stack to the chest.
- **ItemTaken** removes it, but never more than the chest holds: otherwise `NotEnoughItemsException`, counts and log unchanged.
- **ItemMoved** is a take here plus a placement in the `to` chest. If this chest does not have enough, nothing moves on either side.
- `report()` prints the log, then the total number of items:

```
1. placed 64 x cobblestone
2. took 16 x cobblestone
3. moved 8 x cobblestone to Storage B
items: 40
```

The records, the exception and `ItemStack` are written. `Event` is a plain interface for now.

## Your turn

Work in the order of the objectives; the first one turns green early.

- `Chest.apply(Event)`: a `switch` over the three records.
- `Chest.report()`.
- Last: make `Event` sealed, and check your switches have no `default`.
