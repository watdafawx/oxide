The planner has exactly three shapes, and `area` must handle each one with its own sum. In world 01 a switch picked between numbers or letters. Now it has to pick by kind of object, and you want the compiler to tell you if one kind is ever missed.

**Try the obvious thing.** A switch over a `Shape` with one case per record, and no `default`, does not compile yet:

```text
error: the switch expression does not cover all possible input values
```

**Why it refuses.** Any class anywhere could write `implements Shape`, so three cases can never be all of them. The compiler is right: a fourth shape would fall through the switch.

**Close the list.** `public sealed interface Shape permits Circle, Rectangle, Triangle {}` says these three, and nothing else, may implement `Shape`. A **sealed** interface has a fixed list of kinds, like an enum has a fixed list of values. Records may implement interfaces, which is how each shape joins the list.

**Match by kind.** `case Circle c ->` matches when the shape is a `Circle`, and names it `c` with its real type, so `c.radius()` works on that line. One case per kind, and no `default`:

```java
String sound = switch (pet) {
    case Cat c -> "meow";
    case Fish f -> "...";
    case Bird b -> "tweet";
};
```

**Why it is worth it.** Add a `Square` to `permits` next month, and every switch that forgot it stops compiling. The compiler finds each spot for you, instead of a user finding the bug.

## Your turn

- Seal `Shape` so only `Circle`, `Rectangle` and `Triangle` may implement it.
- In `Shapes.area`, switch over `shape`: pi * r * r, width * height, and base * height / 2.
- Save. Both objectives turn green.
