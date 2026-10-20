**Why now:** you are about to let commands change the player list while other code reads it. The borrow checker isn't punishing you: it does at compile time the bug-hunt you'd otherwise do while a server is crashing at 2 a.m.

Three questions it asks about every variable:

1. **Who owns it?** Exactly one variable.
2. **Is anyone borrowing it?** Many `&` readers, or one `&mut` writer, never both.
3. **Is the owner still alive?** A borrow can't outlive what it points to.

Read this and mark each line as *moves*, *borrows* or *nothing*. The quiz below asks about it, so a quick pass is enough:

```rust
let a = String::from("<Steve> hi");   // 1: a owns the chat line
let log = a;                          // 2: moves a -> log (a is dead now)
let c = &log;                         // 3: borrows log
let d = &log;                         // 4: borrows log again (two readers: fine)
println!("{c} {d}");                  // 5: uses both borrows
let e = log;                          // 6: moves log -> e (fine: c and d are finished)
```

Line 6 is only legal because the borrows `c` and `d` ended after line 5. The compiler tracks that for you.

## Quiz

Two questions below. Look at the code above if you need to.
