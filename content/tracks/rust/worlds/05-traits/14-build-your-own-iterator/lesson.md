The export button counts down 3, 2, 1 before it saves. You want that countdown to work with `for` and `collect`, like a range does. All of those run on one small trait, **`Iterator`**.

**One method.** `Iterator` asks for a single method, `next`. Each call hands back the next item as `Some(item)`, and `None` when there are no more.

**Item is its slot.** `type Item = u32;` says what each step yields. It is an associated type, the same kind of slot as `Output` two quests ago.

**Why &mut self.** `next` takes `&mut self` because the iterator remembers where it is. Every call moves that position on, here by counting down one, so the next call gives the next number.

```rust
struct Up {
    cur: u32,
    max: u32,
}

impl Iterator for Up {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.cur >= self.max {
            return None;
        }
        self.cur += 1;
        Some(self.cur)
    }
}
// Up { cur: 0, max: 3 } yields 1, 2, 3
```

**None means done.** After the first `None`, `for` and `collect` stop asking. Make sure your `next` reaches `None`, or a `for` loop over it never ends.

## Your turn

- Write `next` for `Countdown`: `None` at 0, otherwise hand back the current `n` and count down by one.
