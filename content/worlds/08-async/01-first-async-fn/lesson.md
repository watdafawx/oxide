**Why now:** the mod downloader spends nearly all its time waiting on servers, so every download function is an `async fn` that tokio drives.

A downloader spends most of its time **waiting**: for a server, for the disk. Async lets one program wait on many things without freezing.

- `async fn` does not run when you call it. It returns a **future**: a recipe for work.
- `.await` runs the recipe and gives you the value. It can only be used inside an `async fn`.
- Something has to drive futures. **tokio** is that runtime. In a real app you write `#[tokio::main]` on `main`; in tests you write `#[tokio::test]`.

```rust
async fn jar_name(id: u32) -> String {
    format!("mod-{id}.jar")
}

#[tokio::main]
async fn main() {
    let name = jar_name(21).await; // "mod-21.jar"
    println!("{name}");
}
```

No `.await`, no work. Forget it and the compiler warns you.

## Your turn

In `src/lib.rs`:

- `greet`: return `"Hello, <name>!"`.
- `add_async`: return `a + b`.
- `greet_twice`: `.await` `greet(name)` two times and join them with a space.
