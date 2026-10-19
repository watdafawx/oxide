A mod manager spends most of its time **waiting**: for a download, for a server, for the disk. Async lets one program wait on many things without freezing.

- `async fn` does not run when you call it. It returns a **future**: a recipe for work.
- `.await` runs the recipe and gives you the value. It can only be used inside an `async fn`.
- Something has to drive futures. **tokio** is that runtime. In a real app you write `#[tokio::main]` on `main`; in tests you write `#[tokio::test]`.

```rust
async fn double(n: u32) -> u32 {
    n * 2
}

#[tokio::main]
async fn main() {
    let x = double(21).await; // 42
    println!("{x}");
}
```

No `.await`, no work. Forget it and the compiler warns you.

## Your turn

In `src/lib.rs`:

- `greet`: return `"Hello, <name>!"`.
- `add_async`: return `a + b`.
- `greet_twice`: `.await` `greet(name)` two times and join them with a space.
