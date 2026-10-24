Is this shape a square? Only a rectangle can be, and only when its width equals its height. With `case Rectangle r`, you would then call `r.width()` and `r.height()`. For a record, Java can hand you the parts directly.

**Take it apart.** A **record pattern** lists the components in the case: `case Rectangle(double w, double h) ->` matches a rectangle and names its width `w` and its height `h` on that line. It reads like the record's own declaration:

```java
String info = switch (pet) {
    case Cat(String name, int lives) -> name + " has " + lives + " lives";
    case Fish f -> f.name();
    case Bird b -> b.name();
};
```

**Your names.** The names in the pattern are new variables, so pick any you like: `w` and `h` are fine. The types and the order must match the record's components.

**Why unpack.** `w == h` reads like the rule itself, "width equals height", with no accessor calls in the way. The more parts a rule needs, the more a pattern saves.

**Mix the two kinds.** A record pattern and a plain `case Circle c` can sit in the same switch. The switch still covers every kind of the sealed interface, so there is no `default`.

## Your turn

- In `Shapes.isSquare`, switch over `shape`.
- Unpack a `Rectangle` into its width and height and hand back whether they are equal.
- Hand back `false` for a `Circle` and a `Triangle`. Save. Both objectives turn green.
