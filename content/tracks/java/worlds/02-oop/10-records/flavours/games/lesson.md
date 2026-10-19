**Why now:** last quest you wrote `equals`, `hashCode` and `toString` by hand for `BlockPos`: 30 lines for three ints. A **record** is Java's one-line version: you list the fields, Java writes everything else. Modders use records for positions, ids and settings all the time.

```java
// the long way: fields, constructor, accessors, equals, hashCode, toString...
// the record way:
public record ChunkPos(int x, int z) {

    public ChunkPos {                               // compact constructor: validation only
        if (Math.abs(x) > 1_875_000) {
            throw new IllegalArgumentException("outside the world: " + x);
        }
    }

    public ChunkPos east() {                        // "move" = build a copy
        return new ChunkPos(x + 1, z);
    }
}

ChunkPos a = new ChunkPos(3, -7);
a.x();                                      // accessor named after the field, no "get"
a.equals(new ChunkPos(3, -7));              // true: compared by value
a.toString();                               // ChunkPos[x=3, z=-7]
a.east();                                   // new ChunkPos(4, -7); `a` is unchanged
```

- Fields are `private final`: a record is **immutable**.
- The **compact constructor** has no parentheses. It runs first, so it is the place to validate.
- Records can have extra methods, static methods and implement interfaces. They cannot extend a class.

## Your turn

- Open `src/main/java/oxide/records/BlockPos.java`. It is last quest's class.
- Turn it into `record BlockPos(int x, int y, int z)` and delete everything the record writes for you.
- Add the compact constructor (y must be in -64..319) and `offset`.
