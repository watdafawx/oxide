Every new shape starts with a pen, and most people never change it. **`Default`** is the standard trait for "the value you get when nobody says otherwise".

**One trait, one method.** `Default` has a single method, `fn default() -> Self`, with no `self` at all: it makes a value from nothing. You call it on the type, `Pen::default()`.

**Derive it when zero is right.** `#[derive(Default)]` fills every field with that field's own default: `0` for numbers, `""` for `String`, `false` for `bool`, an empty `Vec`. Margins of zero are exactly what a new page wants.

**Write it when zero is wrong.** A pen 0 wide draws nothing. Then you implement `Default` by hand, like any other trait, and return the value you mean.

```rust
#[derive(Debug, Default)]
struct Counter {
    clicks: u32,                 // Counter::default().clicks is 0
}

struct Zoom {
    percent: u32,
}

impl Default for Zoom {
    fn default() -> Self {
        Zoom { percent: 100 }    // 0% would show nothing
    }
}
```

**Why a trait, not just new().** Generic code can ask for it without knowing your type. `Option::unwrap_or_default` on a `None` calls `Pen::default()`, and a struct that holds a `Pen` can `derive(Default)` because `Pen` has one.

## Your turn

- Add `Default` to the derive line on `Margins`, and return `Margins::default()` from `fresh_margins`.
- In `impl Default for Pen`, return a pen 1 wide in `"black"`.
