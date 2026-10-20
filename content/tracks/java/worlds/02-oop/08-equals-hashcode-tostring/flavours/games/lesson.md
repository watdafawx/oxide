**Why now:** your mod keeps a `Set<BlockPos>` of lit torches and a `Map<BlockPos, ...>` of chests. By default Java compares objects by **identity**: the position (10, 64, -3) you built a tick ago and the one you build now are "different" because they are two objects. The set says "not found", you place a second chest on top of the first.

Three methods from `Object` fix that, and they go together:

```java
public class ChunkPos {
    private final int x, z;
    public ChunkPos(int x, int z) { this.x = x; this.z = z; }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof ChunkPos other)) return false;   // also false for null
        return x == other.x && z == other.z;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, z);                           // same fields as equals
    }

    @Override
    public String toString() {
        return "chunk " + x + "," + z;                       // shown in chat, logs, errors
    }
}
```

- **The contract:** if `a.equals(b)` then `a.hashCode() == b.hashCode()`. `HashSet` and `HashMap` jump to a bucket by hash first, then use `equals`. Break the contract and equal positions land in different buckets: "not found".
- `instanceof ChunkPos other` checks the type and gives you the cast variable in one go.
- `Objects` lives in `java.util`, so a file that uses it starts with `import java.util.Objects;` (one `import` line per outside class). Your starter already has it.
- `toString()` is what chat messages, logs and AssertJ failure messages show.

## Your turn

- Open `src/main/java/oxide/equals_hashcode_tostring/BlockPos.java`.
- Write `equals` (same x, y, z), `hashCode` (same fields) and `toString` (`(10, 64, -3)`).
