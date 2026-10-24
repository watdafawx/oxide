The planner shows the throughput of a machine together with everything downstream of it. That is quest 02's walk again, with two differences: a machine can feed many machines instead of one "rest", and each step goes through a `RefCell`.

**Same shape, more branches.** The total is "my rate plus the total of each machine I feed". The base case is a machine that feeds nothing: the loop over `feeds` runs zero times and you are left with its own rate. No `End` arm is needed.

**Borrow to read.** `assembler.borrow()` gives a reader. Keep it in a variable, `let a = assembler.borrow();`, then use `a.rate` and loop `for target in &a.feeds`. Each target is itself an `AssemblerRef`, ready for the recursive call.

```rust
fn chain_length(machine: &AssemblerRef) -> u32 {
    let m = machine.borrow();
    let mut longest = 0;
    for target in &m.feeds {
        let len = chain_length(target);
        if len > longest {
            longest = len;
        }
    }
    longest + 1
}
```

**Readers are fine.** While `a` is alive the machine is borrowed for reading, and the recursive calls borrow *other* machines, the ones it feeds. Many readers at once are allowed, so nothing panics. A `borrow_mut()` of a machine while `a` still reads it would panic (quest 10), which is why the habit is to keep borrows short.

## Your turn

- In `total_throughput`, start from the machine's own rate and add `total_throughput` of each machine it feeds.
- `machine_count` has the same shape: each machine counts 1, plus the count of each machine it feeds.
