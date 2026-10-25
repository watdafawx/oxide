A block at y = 400 is above the build limit, and one at x = 40,000,000 is beyond the world border. A record writes its constructor for you, which is great until you need it to check something. You want to add rules to that constructor without writing it all out again.

**A compact constructor.** Inside the record, `public BlockPos { ... }` with no brackets after the name is a **compact constructor**. It runs first, with the components as parameters, and Java stores them in the fields after it finishes. So it is the place for rules only.

**Two limits.** The world runs from y = -64 to 319, and the border sits 30,000,000 blocks from the centre in every direction. Each rule is one `if` with `||` between its two ends.

**Refuse with throw.** For a value that cannot be fixed, stop. **`throw`** ends the constructor at once with an error, an **exception**, instead of a finished object. Nobody ever holds a position outside the world:

```java
public ChunkPos {
    if (x < -1_875_000 || x > 1_875_000) {
        throw new IllegalArgumentException("outside the world: " + x);
    }
}
```

**Read the line.** `new IllegalArgumentException("...")` builds the error object, the way `new` builds any object, with a message for the red text. `IllegalArgumentException` is Java's ready-made exception for "you passed me a bad value". It is also what the starter's TODO lines have done all along, with `UnsupportedOperationException`, "not written yet". World 04 teaches catching them.

## Your turn

- Add a compact constructor to `BlockPos`.
- In it, throw an `IllegalArgumentException` when `y` is below -64 or above 319.
- Throw one too when `x` or `z` is beyond 30,000,000 either way. Save. Both objectives turn green.
