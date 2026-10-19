**Why now:** the mod downloader must wait on the network without freezing the launcher. Everything async starts with one idea: an **`async fn`** does not run when you call it, it returns a **future**, and `.await` is what runs it.

```rust
async fn announce_download(jar: &str) -> String {
    format!("Downloading {jar}")
}

#[tokio::main]
async fn main() {
    let line = announce_download("sodium.jar").await;   // runs now
    println!("{line}");
}
```

- `async fn f() -> T` really returns `impl Future<Output = T>`.
- `.await` only works inside another `async fn` (or async block).
- An async runtime drives futures. This course uses **tokio**: `#[tokio::main]` for programs, `#[tokio::test]` for tests.
- Your function body looks like normal code. The magic is in what `.await` does later: it lets other tasks run while this one waits.

## Your turn

In `src/lib.rs`:

- `announce_download`: return `"Downloading <jar>"`.
- `total_size`: return the sum of two file sizes.
