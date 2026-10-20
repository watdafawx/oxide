**Why now:** your machine graph starts with belts. A belt is a run of segments, and each segment carries some items and then *continues into the next belt*. A belt that contains a belt that contains a belt: Rust needs a **`Box<T>`** to know how big that is.

A `Box` is a pointer to a value on the heap. It has a fixed size (one pointer) and it **owns** what it points to: when the Box goes out of scope, the value is freed.

```rust
enum Belt {
    Segment(u32, Box<Belt>),  // items on this tile, then the rest of the belt
    End,
}
use Belt::{End, Segment};

let belt = Segment(8, Box::new(Segment(4, Box::new(End))));
```

Without the Box, `Belt` would be infinitely big and the compiler refuses it.

Matching works as usual, and `&Box<Belt>` auto-derefs to `&Belt` when you pass it on.

To walk it, match on it: the `End` arm returns a plain number and stops, the other arm calls the *same function* on the rest of the chain and adds its own part. A function that calls itself like this is **recursion**.

## Your turn

In `src/lib.rs` fill in:

- `extend`: wrap the old belt in a Box and put a new segment with `items` in front.
- `total_items`: recursion over `Segment` / `End`.
- `segment_count`: same shape, counting segments.
