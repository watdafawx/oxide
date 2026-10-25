Your machine graph starts with belts. A belt is a run of segments: the items on this tile, then the rest of the belt. That sentence mentions a belt inside a belt. Rust needs to know how many bytes every value takes before the program runs, and this one has no end.

**Where values live.** A quick refresher from the ownership world. Each function call gets the **stack**, a fixed-size area for its local values that is thrown away when the function returns. Values that can grow, like the text of a `String`, live on the **heap**, a big shared area, and the function keeps only their address. When a value's owner reaches its closing brace, the value is **dropped**, which means its memory is freed.

**Try the obvious thing.** Write the belt as "items on this segment, then the rest": `enum Belt { Segment(u32, Belt), End }`. The starter has exactly that, and the compiler refuses it:

```text
error[E0072]: recursive type `Belt` has infinite size
```

**Why infinite.** To make room for a `Belt` on the stack, the compiler adds up its parts: 4 bytes for the `u32`, plus a whole `Belt`, which is 4 bytes plus a whole `Belt`, forever. It needs that number **at compile time**, while `cargo` builds the program, because that is when it lays out each function's stack space.

**A pointer is an address.** A **pointer** is a number that says where in memory a value lives. Its size never changes, whatever it points at: 8 bytes on a 64-bit computer. Keep the rest of the belt somewhere else and store only its address, and the size stops growing.

**Box: an owning pointer.** `Box::new(value)` moves the value to the heap and gives you a `Box<T>`, a pointer that **owns** it. When the `Box` is dropped, the value on the heap is freed with it. So `Segment(u32, Box<Belt>)` is "4 bytes for the items, 8 for the address", a size the compiler can count.

```rust
enum Pipe {
    Run(u32, Box<Pipe>),   // fluid in this pipe, then the rest of the line
    Tank,
}

let line = Pipe::Run(100, Box::new(Pipe::Run(40, Box::new(Pipe::Tank))));
```

## Your turn

- Save and read the E0072 error. Its `help:` line names the fix.
- In `Belt`, put the second field of `Segment` in a `Box`. The first objective turns green.
- In `extend`, return a new `Segment` with `items` in front and the old belt in a `Box`.
