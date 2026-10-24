The plan is built; now you want its total time. A `for` loop does not fit, because a `Plan` is not tasks side by side: it is one task holding the rest of the plan. So the function takes the same shape as the type.

**Two cases.** Every `Plan` is either `Done` or a `Task` with minutes and the rest. For `Done` the answer is plain: 0 minutes. For a `Task` it is "my minutes plus the total of the rest".

**A function that calls itself.** "The total of the rest" is the very function you are writing, and calling it from inside itself is allowed. That is **recursion**: each call handles one task and hands the shorter rest to the next call.

**Why it stops.** Every call gets a shorter plan, so the calls reach `Done`, which answers without calling again. That arm is the **base case**. A recursive function without one calls itself until the stack is full and the program crashes with a stack overflow.

**The Box steps aside.** Matching on a `&Plan`, the arm `Task(minutes, rest)` gives you references: `rest` is a `&Box<Plan>`, and you can pass it straight to a function that wants a `&Plan`. Rust looks through the `Box` for you; the next quest shows how. To compare a referenced number, write `*` in front of it to read the number it points at.

```rust
fn high_notes(song: &Song) -> u32 {
    match song {
        Song::End => 0,                              // base case
        Song::Note(pitch, rest) => {
            let here = if *pitch > 60 { 1 } else { 0 };
            here + high_notes(rest)                  // the rest, by the same function
        }
    }
}
```

## Your turn

- In `total_minutes`, match on `plan`: `Done` gives 0, a `Task` gives its minutes plus `total_minutes` of the rest.
- `task_count` has the same shape, but each `Task` counts as 1.
