`Vec<T>` and `Option<T>` hold any type. Your own struct can too: the ruler tool keeps a pair of sizes, the colour picker a pair of colours, and both are the same `Pair` with a different `T`.

**A type parameter on a struct.** Write `struct Pair<T> { first: T, second: T }`. `Pair<u32>` and `Pair<&str>` are two real types built from one definition, the same way `Vec<u32>` and `Vec<String>` are.

**Declare it on the impl.** Methods go in `impl<T> Pair<T>`. The first `<T>` declares the placeholder and the second uses it: "for every `T`, here are the methods of `Pair<T>`".

**Self is the type.** Inside an `impl`, **`Self`** is short for the type the block is for, here `Pair<T>`. `fn new(..) -> Self` reads "returns a Pair", and keeps working if you rename the struct.

```rust
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }

    fn into_inner(self) -> T {      // self, not &self
        self.value
    }
}
```

**self, not &self.** A method that takes plain `self` takes the value itself, not a borrow. Calling it **moves** the pair into the method, as moves did in Ownership, so the caller can't use the old one afterwards.

**Why that fits swap.** After `let p = p.swap();` nobody needs the old order. Because `swap` owns the pair, it can move both fields into the new one without copying or cloning them.

## Your turn

- Write `Pair::new`: build a `Pair` from the two values.
- Write `swap`: a new `Pair` with `first` and `second` crossed over.
