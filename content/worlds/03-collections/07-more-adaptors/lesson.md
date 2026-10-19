A handful of small adaptors cover most of what you used to hand-write with index math.

```rust
let names = ["Ada", "Linus", "Grace"];

for (i, name) in names.iter().enumerate() {   // (0, "Ada"), (1, "Linus"), ...
    println!("{}: {name}", i + 1);
}

let scores = [10, 20, 30];
let pairs: Vec<_> = names.iter().zip(scores).collect();   // [("Ada",10), ...]

let first_two: Vec<_> = names.iter().take(2).collect();   // Ada, Linus
let after_one: Vec<_> = names.iter().skip(1).collect();   // Linus, Grace
let backwards: String = "abc".chars().rev().collect();    // "cba"

let temps = [3, 5, 4, 8];
let pairs: Vec<&[i32]> = temps.windows(2).collect();      // [3,5] [5,4] [4,8]
```

- **`enumerate()`**: gives `(index, item)`. **`zip(other)`**: pairs two iterators, stops at the shorter.
- **`take(n)` / `skip(n)`**: first n / all but the first n. Together they make pages.
- **`rev()`**: reverse. **`chars()`**: a string's characters; collect them back into a `String`.
- **`windows(n)`**: slides a window of n neighbours over a slice. Perfect for "compare each with the next".

## Your turn

- Open `src/lib.rs`.
- Four functions, one chain each: `numbered`, `dot_product`, `page`, `count_rises`.
