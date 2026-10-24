A mod is a stream of events: a player joins, a block breaks, an item drops. Each kind needs its own chat line. In world 01 a switch picked between numbers or letters. Now it has to pick by kind of object, and you want the compiler to tell you if one kind is ever missed.

**Try the obvious thing.** A switch over an `Event` with one case per record, and no `default`, does not compile yet:

```text
error: the switch expression does not cover all possible input values
```

**Why it refuses.** Any class anywhere could write `implements Event`, so three cases can never be all of them. The compiler is right: a fourth event would fall through the switch.

**Close the list.** `public sealed interface Event permits PlayerJoined, BlockBroken, ItemDropped {}` says these three, and nothing else, may implement `Event`. A **sealed** interface has a fixed list of kinds, like an enum has a fixed list of values. Records may implement interfaces, which is how each event joins the list.

**Match by kind.** `case PlayerJoined e ->` matches when the event is a `PlayerJoined`, and names it `e` with its real type, so `e.player()` works on that line. One case per kind, and no `default`:

```java
String text = switch (sign) {
    case Welcome w -> "Hello " + w.player();
    case Warning w -> w.text();
    case Shop s -> s.item() + " for " + s.price();
};
```

**Why it is worth it.** Add a `PlayerLeft` to `permits` next update, and every switch that forgot it stops compiling. The compiler finds each spot for you, instead of a player finding the bug.

## Your turn

- Seal `Event` so only `PlayerJoined`, `BlockBroken` and `ItemDropped` may implement it.
- In `Chat.line`, switch over `event`: `"Alex joined the game"`, `"Alex broke stone"`, `"3 x diamond dropped"`.
- Save. Both objectives turn green.
