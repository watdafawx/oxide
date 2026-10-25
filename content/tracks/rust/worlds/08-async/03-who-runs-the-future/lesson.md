The downloader's settings screen is plain code, not async, and it wants to show the status line that `describe` builds. Plain code can't `.await`, and a future can't run itself. Something has to drive it: that is the runtime from quest 01, and today you start one by hand.

**A future needs a driver.** A future only moves when someone asks it to go on, and comes back to it after each wait. That someone is the runtime. `tokio::runtime::Runtime::new()` starts one, with its worker threads, and `rt.block_on(fut)` runs one future to the end and hands back its value. The plain code that called `block_on` stops until then.

```rust
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();           // start tokio
    let header = rt.block_on(feed_header("daily")); // run one future to the end
    println!("{header}");
}
```

**The attribute writes that for you.** Most programs put `#[tokio::main]` above `async fn main` instead. It is an **attribute**, like `#[derive]` from world 02: an instruction about the item below it. This one is a macro, code that writes code (world 04): it turns your async `main` into a plain `main` that starts a runtime and calls `block_on`, the two lines above. `#[tokio::test]` does the same for one test.

**Why Rust makes you choose.** JavaScript and Python come with one built-in event loop. Rust keeps the language small and ships none, so you pick the runtime that fits: tokio for apps and servers, tiny ones for chips without an operating system. The price is that one line on `main`.

**A runtime is not a thread.** By default `#[tokio::main]` starts one worker thread per CPU core and spreads every task over them. Ten thousand tasks may share eight threads; the runtime decides which task runs where.

**Only at the edge.** `block_on` stops the thread that calls it, so call it only where plain code meets async code. Inside code that already runs on a runtime it panics with `Cannot start a runtime from within a runtime`; there you `.await` instead.

## Your turn

- In `describe_now`, start a tokio runtime.
- Use it to run `describe(id)` to the end and return the text.
