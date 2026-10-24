In the last three quests you wrote `equals`, `hashCode` and `toString` by hand for `BlockPos`, on top of fields and a constructor. That is thirty lines for three ints, and every one of them can be got wrong. For a class that is just data, Java has a one-line version, and mod code uses it everywhere.

**List the fields.** A **record** is declared with its fields, called **components**, in brackets: `public record ChunkPos(int x, int z) {}`. That single line is the whole class.

**What Java writes.** From that line Java writes a private final field for each component, a constructor taking them in order, an accessor for each, and `equals`, `hashCode` and `toString` that use all the components. Everything you did by hand, done the same way every time.

**Accessors, no get.** The accessor has the component's name: `chunk.x()`, `chunk.z()`. That is why your classes in this world used `count()`, not `getCount()`.

**It cannot change.** The fields are final and there are no setters, so a record is **immutable**: once built, it holds its values forever. Its `toString` reads like this:

```java
ChunkPos home = new ChunkPos(3, -7);
home.toString();                      // ChunkPos[x=3, z=-7]
home.equals(new ChunkPos(3, -7));     // true
```

**When to use one.** A record fits values: positions, ids, item stacks, settings. A thing that changes over time, like a chest with items going in and out, stays a class.

## Your turn

- Replace the whole `BlockPos` class with a record that has the components `int x`, `int y` and `int z`, in that order.
- Save. Both objectives turn green.
