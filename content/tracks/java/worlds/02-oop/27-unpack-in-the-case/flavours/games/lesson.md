The admin log wants the spot of every broken block: `10 64 -3`. With `case BlockBroken e`, you would call `e.pos()`, then `.x()`, `.y()` and `.z()` on that. For a record, Java can hand you the parts directly, even the parts inside a part.

**Take it apart.** A **record pattern** lists the components in the case: `case Shop(String item, int price) ->` matches a shop sign and names its parts `item` and `price` on that line. It reads like the record's own declaration:

```java
String text = switch (sign) {
    case Shop(String item, int price) -> item + " for " + price;
    case Welcome w -> "Hello " + w.player();
    case Warning w -> w.text();
};
```

**Your names.** The names in the pattern are new variables, so pick any you like. The types and the order must match the record's components.

**Patterns nest.** A component that is itself a record can be unpacked in place: `BlockBroken(String player, String block, BlockPos(int x, int y, int z))` hands you all five parts at once.

**Mix the two kinds.** A record pattern and a plain `case PlayerJoined e` can sit in the same switch. The switch still covers every kind of the sealed interface, so there is no `default`.

## Your turn

- In `Chat.where`, switch over `event`.
- Unpack a `BlockBroken` and its `BlockPos` in one case, and hand back `"x y z"` with spaces between.
- Hand back `"nowhere"` for the other two events. Save. Both objectives turn green.
