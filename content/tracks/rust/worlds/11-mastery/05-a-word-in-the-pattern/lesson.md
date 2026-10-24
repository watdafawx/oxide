A stock list reads best in the shape of the data: `item!("apples" => 3)` says "apples, three of them" at a glance, where `Item { name: "apples", count: 3 }` spells out field names on every line. The `=>` in the middle is not Rust syntax; your macro's pattern simply asks for it.

**Words that must be there.** Everything in a pattern that does not start with `$` is a **literal token**: the call must contain exactly that, at that spot. The comma in `($x:expr, $y:expr)` is one; so is the `=>` in `($name:expr => $count:expr)`. A call without it does not match the arm.

```rust
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[macro_export]
macro_rules! point {
    ($x:expr ; $y:expr) => {
        Point { x: $x, y: $y }
    };
}

assert_eq!(point!(3; 4), Point { x: 3, y: 4 });
```

**Captures go anywhere.** The template is ordinary code with `$x` and `$y` pasted in. Here they land in a struct literal, the same `Point { x: .., y: .. }` you write by hand.

**Not every word fits.** After an `expr` capture only `=>`, `,` or `;` may come. `$name:expr at $count:expr` is refused, because Rust could not tell where an expression ends if any word were allowed after it. The starter's `=>` is one of the three.

## Your turn

- In `item!`, replace the stub template with an `Item` built from `$name` and `$count`.
