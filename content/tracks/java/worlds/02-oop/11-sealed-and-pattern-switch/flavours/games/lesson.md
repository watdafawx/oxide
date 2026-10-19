**Why now:** a mod is a stream of events: a player joins, a block breaks, an item drops. Each one needs its own chat line or log entry. A chain of `if (event instanceof ...)` compiles even when you forget a kind. A **sealed** `Event` plus a `switch` makes the compiler count for you: add a fourth event later and every switch that misses it stops compiling.

```java
public sealed interface Sign permits Welcome, Warning, Shop {}     // these three, and nothing else
public record Welcome(String player) implements Sign {}
public record Warning(String text, int level) implements Sign {}
public record Shop(String item, int price) implements Sign {}

static String text(Sign sign) {
    return switch (sign) {                                           // no default: add a 4th Sign and this stops compiling
        case Welcome w -> "Hello " + w.player();
        case Warning w -> w.text();
        case Shop s -> s.item() + " for " + s.price();
    };
}

static String urgency(Sign sign) {
    return switch (sign) {
        case Warning w when w.level() > 3 -> "RUN";                 // guard: extra condition after `when`
        case Warning(String text, int level) -> "careful: " + text; // record pattern: unpack it
        default -> "fine";
    };
}
```

- `sealed ... permits` lists every allowed subtype. Records are perfect for events.
- `case Welcome w ->` tests the type **and** gives you `w` already cast. No `instanceof`, no cast.
- `when` adds a condition. Order matters: a guarded case must come **before** the same type without a guard.
- A **record pattern** `Warning(String text, int level)` takes the record apart in the case label. They nest: `BlockBroken(String p, String b, BlockPos(int x, int y, int z))`.

## Your turn

- Open `Chat.java`. `Event`, the three event records and `BlockPos` are done.
- `line`: one case per event, no `default`. Formats: `"Alex joined the game"`, `"Alex broke stone"`, `"3 x diamond dropped"`.
- Add the guard: an `ItemDropped` with count 0 is `"nothing dropped"`.
- `where`: for a `BlockBroken`, return `"x y z"` using a nested record pattern; anything else is `"nowhere"`.
