The server loads chunks one row at a time as a player walks east. You want `for chunk in ChunkRow::new(0, 8, z)` to work, like a range does. That runs on one small trait, **`Iterator`**.

**One method.** `Iterator` asks for a single method, `next`. Each call hands back the next item as `Some(item)`, and `None` when there are no more.

**Item is its slot.** `type Item = ChunkPos;` says what each step yields. It is an associated type, the same kind of slot as `Loot` two quests ago.

**Why &mut self.** `next` takes `&mut self` because the iterator remembers where it is. Every call moves that position on, here one chunk east, so the next call gives the next chunk.

```rust
struct Ticks {
    now: u32,
    end: u32,
}

impl Iterator for Ticks {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.now >= self.end {
            return None;
        }
        self.now += 1;
        Some(self.now)
    }
}
// Ticks { now: 0, end: 3 } yields 1, 2, 3
```

**None means done.** After the first `None`, `for` and `collect` stop asking. Make sure your `next` reaches `None`, or a `for` loop over it never ends, and neither does the server tick.

## Your turn

- Write `next` for `ChunkRow`: `None` once `x` is past `last_x`, otherwise hand back the chunk at `x` and step one east.
