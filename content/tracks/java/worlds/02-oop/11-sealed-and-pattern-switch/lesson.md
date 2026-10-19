**Why this matters:** you have a handful of kinds of thing (shapes, payments, messages) and a function that must do something for each kind. A chain of `if (x instanceof ...)` compiles fine even if you forget a kind. A **sealed** interface plus a `switch` makes the compiler count for you.

```java
public sealed interface Pet permits Cat, Fish, Bird {}      // these three, and nothing else
public record Cat(String name, int lives) implements Pet {}
public record Fish(String name) implements Pet {}
public record Bird(String name, boolean canFly) implements Pet {}

static String sound(Pet pet) {
    return switch (pet) {                                    // no default: add a 4th Pet and this stops compiling
        case Cat c -> "meow";
        case Fish f -> "...";
        case Bird b -> "tweet";
    };
}

static String info(Pet pet) {
    return switch (pet) {
        case Cat c when c.lives() == 0 -> "a very sad cat";  // guard: extra condition after `when`
        case Cat(String name, int lives) -> name + " has " + lives + " lives";   // record pattern: unpack it
        case Fish f -> f.name();
        case Bird b -> b.name();
    };
}
```

- `sealed ... permits` lists every allowed subtype. Records, classes and interfaces can all be subtypes.
- `case Cat c ->` tests the type **and** gives you `c` already cast. No `instanceof`, no cast.
- `when` adds a condition. Order matters: a guarded case must come **before** the same type without a guard.
- A **record pattern** `Cat(String name, int lives)` takes the record apart in the case label.

## Your turn

- Open `Shapes.java`. `Shape` and the three records are done.
- `area`: one case per shape, no `default`.
- `describe`: a Circle of radius 0 is `"a dot"` (guard), otherwise `"a circle"`, `"a rectangle"`, `"a triangle"`.
- `isSquare`: unpack `Rectangle(w, h)` and compare.
