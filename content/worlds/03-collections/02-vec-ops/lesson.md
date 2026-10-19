Real lists need editing: remove a duplicate, sort for display, filter out junk. `Vec` has a method for each.

```rust
let mut v = vec![5, 3, 5, 1];
v.sort();                  // [1, 3, 5, 5]
v.dedup();                 // [1, 3, 5]   (only removes neighbours)
v.insert(1, 9);            // [1, 9, 3, 5]
let gone = v.remove(0);    // gone = 1, v = [9, 3, 5]
println!("{}", v.contains(&3));   // true: contains wants a reference
v.retain(|n| *n > 4);      // keep only matches: [9, 5]
```

- **`insert(i, x)`** and **`remove(i)`** shift everything after `i`. `remove` hands the item back.
- **`sort()`** works in place on `mut` vectors. **`dedup()`** needs sorted data to catch all repeats.
- **`contains(&x)`** asks "is it in there?".
- **`retain(|x| keep_it)`** keeps items where the closure says `true`. The `|x| ...` is a closure; a later quest unpacks it.

## Your turn

- Open `src/lib.rs`.
- Fill in the four functions. Each is 1-3 lines using a method above.
