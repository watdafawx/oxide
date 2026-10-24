The downloader's first step names the file an episode is saved under. Later that step will ask a server and wait, so it is an `async fn` from the start. Today you write one, call it from another, and meet the one word that makes it work.

**Mark it async.** Putting `async` before `fn` changes what a call gives back. The body is ordinary code, `format!` works as always, but the caller does not get the `String` straight away. It gets a future: the note from the last quest that says "a `String` will be here".

**Try the obvious thing.** Use the call as if it were the text: `format!("Downloading {}", episode_file(7))`. Rust stops with `error[E0277]: impl Future<Output = String> doesn't implement std::fmt::Display`. You are holding the ticket, not what it stands for.

**Cash the ticket.** Writing `.await` after the call means "pause here until the value is ready, and let other work run meanwhile". `episode_file(7).await` is the `String`.

```rust
async fn feed_url(show: &str) -> String {
    format!("https://feeds.example.com/{show}")
}

async fn feed_header(show: &str) -> String {
    let url = feed_url(show).await; // pause until the String is ready
    format!("Feed: {url}")
}
```

**Only inside async.** Pausing only makes sense in something that can be paused, so `.await` works only inside an `async fn` (or an async block, quest 07). In a plain `fn` you get `error[E0728]: await is only allowed inside async functions and blocks`. That is why `describe` is async too: a function that awaits becomes async itself.

**The tests.** The starter's tests are `async fn` with `#[tokio::test]` above them instead of `#[test]`, so they can `.await` your functions. The next quest shows what that line does. The first build downloads and compiles tokio, which takes a minute or two; later runs take seconds.

**For the curious.** `async fn episode_file(id: u32) -> String` is shorthand for `fn episode_file(id: u32) -> impl Future<Output = String>`: a function that returns some type that is a future of a `String`.

## Your turn

- In `episode_file`, return `"episode-<id>.mp3"`.
- In `describe`, await `episode_file(id)` and put `"Downloading "` in front of it.
