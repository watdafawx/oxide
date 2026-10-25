You have now met every rule the borrow checker uses. Here they are in one place, then a six-line trace to read. The checker isn't punishing you: it does before the program runs the bug hunt you would otherwise do at 2 a.m.

**Who owns it?** Every value has exactly one owner, and it is dropped when the owner's block ends. Assigning it or passing it by value moves it to a new owner.

**Who is borrowing it?** Any number of `&` readers, or exactly one `&mut` writer, never both at the same moment.

**Is the owner still alive?** A borrow may not outlive the value it points at. A reference to something already dropped would point at freed memory.

**Read the trace.** Mark each line as *moves*, *borrows* or *nothing*. The quiz asks about it, so one careful pass is enough:

```rust
let a = String::from("hi");      // 1: a owns the String
let b = a;                       // 2: moves a -> b (a is dead now)
let c = &b;                      // 3: borrows b
let d = &b;                      // 4: borrows b again (two readers: fine)
let both = format!("{c} {d}");   // 5: last use of both borrows
let e = b;                       // 6: moves b -> e (fine: c and d are finished)
```

**Borrows end early.** Line 6 is legal only because `c` and `d` are not used after line 5. A borrow lasts until its last use, not until the `}`. The compiler tracks that for you.

**Read which rule it names.** When the checker says no, the message points at three places: the owner, the borrow, and the line that conflicts. Fix the one that is out of order, as you did in "One writer at a time".

## Your turn

- Answer the three questions. Look back at the trace if you need to.
