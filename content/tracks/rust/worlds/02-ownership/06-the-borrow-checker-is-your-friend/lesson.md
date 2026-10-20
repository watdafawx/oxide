The borrow checker isn't punishing you. It's doing at compile time the bug-hunt you'd otherwise do at 2 a.m.

Three questions it asks about every variable:

1. **Who owns it?** Exactly one variable.
2. **Is anyone borrowing it?** Many `&` readers, or one `&mut` writer, never both.
3. **Is the owner still alive?** A borrow can't outlive what it points to.

Read this and mark each line as *moves*, *borrows* or *nothing*. The quiz below asks about it, so a quick pass is enough:

```rust
let a = String::from("hi");   // 1: a owns the String
let b = a;                    // 2: moves a -> b (a is dead now)
let c = &b;                   // 3: borrows b
let d = &b;                   // 4: borrows b again (two readers: fine)
println!("{c} {d}");          // 5: uses both borrows
let e = b;                    // 6: moves b -> e (fine: c and d are finished)
```

Line 6 is only legal because the borrows `c` and `d` ended after line 5. The compiler tracks that for you.

## Quiz

Two questions below. Look at the code above if you need to.
