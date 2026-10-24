The belt is built; now you want to know how many items ride on it. A `for` loop does not fit, because a `Belt` is not segments side by side: it is one segment holding the rest of the belt. So the function takes the same shape as the type.

**Two cases.** Every `Belt` is either `End` or a `Segment` with items and the rest. For `End` the answer is plain: 0 items. For a `Segment` it is "my items plus the total of the rest".

**A function that calls itself.** "The total of the rest" is the very function you are writing, and calling it from inside itself is allowed. That is **recursion**: each call handles one segment and hands the shorter rest to the next call.

**Why it stops.** Every call gets a shorter belt, so the calls reach `End`, which answers without calling again. That arm is the **base case**. A recursive function without one calls itself until the stack is full and the program crashes with a stack overflow.

**The Box steps aside.** Matching on a `&Belt`, the arm `Segment(items, rest)` gives you references: `rest` is a `&Box<Belt>`, and you can pass it straight to a function that wants a `&Belt`. Rust looks through the `Box` for you; the next quest shows how. To compare a referenced number, write `*` in front of it to read the number it points at.

```rust
fn full_pipes(pipe: &Pipe) -> u32 {
    match pipe {
        Pipe::Tank => 0,                             // base case
        Pipe::Run(fluid, rest) => {
            let here = if *fluid >= 100 { 1 } else { 0 };
            here + full_pipes(rest)                  // the rest, by the same function
        }
    }
}
```

## Your turn

- In `total_items`, match on `belt`: `End` gives 0, a `Segment` gives its items plus `total_items` of the rest.
- `segment_count` has the same shape, but each `Segment` counts as 1.
