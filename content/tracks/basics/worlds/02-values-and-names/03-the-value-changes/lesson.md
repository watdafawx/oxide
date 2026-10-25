Every ticket sold takes a seat away. A name that keeps its first value forever cannot count anything, so the machine needs names whose value changes as the evening goes on.

**Store again.** Writing `seats = 98` for a name that already exists, without `local`, puts a new value into the same box. The old value is gone. `local` is only for the first time, when the box is made.

**Not maths.** `seats = seats - 1` looks like a false equation, but `=` in code does not mean "is equal to". It means **assign**: work out the right side, then store the result in the name on the left. With 100 seats, the right side is 99, so `seats` becomes 99.

```lua
local sold = 0
sold = sold + 1     -- 1
sold = sold + 1     -- 2
sold = sold + 5     -- 7: a group of five
```

**Moment by moment.** A variable holds one value at a time, and which one depends on how far the program has got. To read code by hand, keep a small table: the name, then its value after each line. This is called **tracing**, and it is how programmers find out what code really does.

**Other languages.** Rust makes you say up front that a value will change (`let mut seats = 100;`), and Rust and Java both have the shorthand `seats -= 1`. Lua has no shorthand, so you write it out: `seats = seats - 1`.

## Your turn

- In `free_seats`, take away the couple's 2 seats, then the family's 4, one line each.
- In `till_total`, add each sale (9, 18, 36) to `till`, one line each.
- Save and check both objectives.
