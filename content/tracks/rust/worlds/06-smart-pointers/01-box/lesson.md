A plan is a list of tasks, and the shortest way to say that is "one task, then the rest of the plan". That sentence mentions a plan inside a plan. Rust needs to know how many bytes every value takes before the program runs, and this one has no end.

**Where values live.** A quick refresher from the ownership world. Each function call gets the **stack**, a fixed-size area for its local values that is thrown away when the function returns. Values that can grow, like the text of a `String`, live on the **heap**, a big shared area, and the function keeps only their address. When a value's owner reaches its closing brace, the value is **dropped**, which means its memory is freed.

**Try the obvious thing.** Write the plan as "minutes for one task, then the rest": `enum Plan { Task(u32, Plan), Done }`. The starter has exactly that, and the compiler refuses it:

```text
error[E0072]: recursive type `Plan` has infinite size
```

**Why infinite.** To make room for a `Plan` on the stack, the compiler adds up its parts: 4 bytes for the `u32`, plus a whole `Plan`, which is 4 bytes plus a whole `Plan`, forever. It needs that number **at compile time**, while `cargo` builds the program, because that is when it lays out each function's stack space.

**A pointer is an address.** A **pointer** is a number that says where in memory a value lives. Its size never changes, whatever it points at: 8 bytes on a 64-bit computer. Keep the rest of the plan somewhere else and store only its address, and the size stops growing.

**Box: an owning pointer.** `Box::new(value)` moves the value to the heap and gives you a `Box<T>`, a pointer that **owns** it. When the `Box` is dropped, the value on the heap is freed with it. So `Task(u32, Box<Plan>)` is "4 bytes for the minutes, 8 for the address", a size the compiler can count.

```rust
enum Song {
    Note(u8, Box<Song>),   // a pitch, then the rest of the song
    End,
}

let tune = Song::Note(60, Box::new(Song::Note(64, Box::new(Song::End))));
```

## Your turn

- Save and read the E0072 error. Its `help:` line names the fix.
- In `Plan`, put the second field of `Task` in a `Box`. The first objective turns green.
- In `add_first`, return a new `Task` with `minutes` in front and the old plan in a `Box`.
