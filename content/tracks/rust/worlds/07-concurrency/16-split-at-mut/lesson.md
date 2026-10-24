The hasher reads file sizes in kibibytes (1 KiB is 1024 bytes) and wants them in bytes, changed in place. Two threads could each take half of the slice. Reading halves worked in the last quest; writing them runs into the one-writer rule from the ownership world.

**Try the obvious thing.** The starter cuts the slice into `&mut sizes[..mid]` and `&mut sizes[mid..]` and gives each half to a scoped thread. The compiler refuses:

```text
error[E0499]: cannot borrow `*sizes` as mutable more than once at a time
help: use `.split_at_mut(position)` to obtain two mutable non-overlapping sub-slices
```

**Why it refuses.** Each `&mut sizes[..]` is a `&mut` borrow of the *whole* `sizes`, as far as the borrow checker can tell. It does not compare `..mid` with `mid..` to prove the two ranges never overlap, so it sees two writers of one value, which is the data race it exists to stop.

**One call, two writers.** `sizes.split_at_mut(mid)` gives back a pair, `(left, right)`: a `&mut` to everything before `mid` and a `&mut` to everything from `mid` on. The standard library promises that the two halves never overlap, so the compiler accepts both writers at once. Each thread then changes its half with `for size in left.iter_mut() { *size *= 1024; }`.

## Your turn

- Save and read the E0499 error.
- Replace the two `let` lines that cut the slice with a single `split_at_mut(mid)`. The objective turns green.
