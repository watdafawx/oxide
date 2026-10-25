The planner labels shapes for the screen reader: "a circle", "a rectangle", "a triangle". A circle of radius 0 is not really a circle, though; it should say "a dot". One kind needs two answers depending on its values.

**A case with a condition.** After the pattern, **`when`** adds a test: `case Circle c when c.radius() == 0 -> "a dot";`. The case matches only when the shape is a circle and the test is true. This test is called a **guard**:

```java
String mood = switch (pet) {
    case Cat c when c.lives() == 0 -> "a very sad cat";
    case Cat c -> "a cat";
    case Fish f -> "a fish";
    case Bird b -> "a bird";
};
```

**Order matters.** Java tries the cases top to bottom and takes the first match. Put the plain `case Circle c` first, and it would catch every circle, radius 0 included. The compiler spots that and refuses:

```text
error: this case label is dominated by a preceding case label
```

**Why not an if.** You could put an `if` inside the plain circle arm instead. The guard keeps each answer on its own line, so the switch still reads like a table: one kind, one condition, one result.

**Still counted.** A guarded case does not count as covering its kind, because its test might be false. The plain `case Circle c` after it covers the rest, so the switch still needs no `default`.

## Your turn

- In `Shapes.describe`, switch over `shape` with one case per record.
- Put a guarded case for a circle of radius 0, `"a dot"`, before the plain circle case.
- Save. Both objectives turn green.
