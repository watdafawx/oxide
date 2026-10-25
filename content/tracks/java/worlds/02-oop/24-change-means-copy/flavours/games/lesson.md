A torch goes one block above the chest. A record cannot change: its fields are final, there is no setter. So the answer is not to move this position, but to make a new one that differs in its numbers.

**Move means copy.** A method that "changes" a record builds a new record with values swapped and hands it back. Minecraft names this one `offset`: `pos.offset(0, 1, 0)` is the block above. The original stays exactly as it was:

```java
public ChunkPos east() {
    return new ChunkPos(x + 1, z);      // one chunk east, same z
}
```

**Read the fields directly.** Inside the record, the components are plain names: `x`, `y`, `z`. No accessor call needed.

**The rules come along.** The copy is built with `new`, so it passes through the compact constructor you wrote last quest. A move above the build limit is refused for the copy too, without a single extra line.

**Why bother.** Code that holds a position can trust it never shifts under its feet. A chest and a hopper can share one `BlockPos` safely, because neither can move it on the other.

## Your turn

- In `offset`, build and hand back a new `BlockPos` at `x + dx`, `y + dy`, `z + dz`.
- Save. Both objectives turn green.
