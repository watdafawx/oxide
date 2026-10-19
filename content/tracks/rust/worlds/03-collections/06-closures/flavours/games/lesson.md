**Why now:** every loot rule is a little function with a setting: "add the Looting bonus", "keep stacks over 16", "sort by rarity". You already wrote `retain(|s| ...)` and `map(|d| ...)`. Those `|x| ...` things are **closures**: small unnamed functions that can use variables from around them.

```rust
let looting = 2;
let with_bonus = |count: u32| count + looting;   // captures `looting`
println!("{}", with_bonus(5));                   // 7

fn apply_twice(f: impl Fn(u32) -> u32, x: u32) -> u32 {
    f(f(x))
}
println!("{}", apply_twice(|n| n * 2, 3));       // 12

fn make_multiplier(k: u32) -> impl Fn(u32) -> u32 {
    move |x| x * k                               // `move` takes k with it
}
let fortune = make_multiplier(3);
println!("{}", fortune(7));                      // 21

// the same idea sorts a loot table: rarest first
let mut table = vec![("bone", 0), ("diamond", 3), ("iron", 1)];
table.sort_by_key(|&(_, rarity)| std::cmp::Reverse(rarity));
```

- `|args| body` is a closure. Types are usually inferred.
- It **captures** variables it uses, so no extra parameters are needed.
- **`impl Fn(A) -> B`** as a parameter type means "any closure or function that takes A and returns B".
- Returning a closure needs `impl Fn(...)` and `move`, because the captured value must survive the call.

## Your turn

- Open `src/lib.rs`.
- `make_looting`, `upgrade_twice`, `count_stacks_where`: fill in each body.
- `stacks_over`: write a closure that uses `limit` from outside.
