Chat shows "3 x diamond dropped", which is fine, until a buggy hopper reports a drop of 0 items and chat says "0 x diamond dropped". An empty drop should say "nothing dropped". One kind of event needs two answers depending on its values.

**A case with a condition.** After the pattern, **`when`** adds a test: `case ItemDropped e when e.count() == 0 -> "nothing dropped";`. The case matches only when the event is a drop and the test is true. This test is called a **guard**:

```java
String urgency = switch (sign) {
    case Warning w when w.level() > 3 -> "RUN";
    case Warning w -> "careful";
    case Welcome w -> "fine";
    case Shop s -> "fine";
};
```

**Order matters.** Java tries the cases top to bottom and takes the first match. Put the plain `case ItemDropped e` first, and it would catch every drop, empty ones included. The compiler spots that and refuses:

```text
error: this case label is dominated by a preceding case label
```

**Why not an if.** You could put an `if` inside the plain drop arm instead. The guard keeps each answer on its own line, so the switch still reads like a table: one kind, one condition, one result.

**Still counted.** A guarded case does not count as covering its kind, because its test might be false. The plain `case ItemDropped e` after it covers the rest, so the switch still needs no `default`.

## Your turn

- In `Chat.line`, switch over `event` with one case per record, the lines from last quest.
- Put a guarded case for a drop of 0 items, `"nothing dropped"`, before the plain drop case.
- Save. Both objectives turn green.
