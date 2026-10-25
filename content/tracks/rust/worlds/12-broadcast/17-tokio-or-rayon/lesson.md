This server does two very different kinds of work at once. Most of the time it waits: for the next frame from the game, for overlays, for UDP sends. Sometimes it computes hard: a folder of photos to cut. Rust has a tool for each, and mixing them up costs either speed or a frozen HUD.

**tokio is for waiting.** An async task that waits on a socket costs a few hundred bytes, not a thread. tokio runs thousands of them on a handful of threads, switching at every `.await`. Feeds, WebSockets, HTTP and timers belong here. The catch: a task that computes for a long time without an `.await` holds its thread, and every task queued on it waits.

**rayon is for computing.** rayon splits independent work, like photos or rows, across every core with work stealing, and returns when all of it is done. Change `.iter()` to `.par_iter()`. It has no idea what a socket is; it never waits, it only computes.

**Threads for long jobs.** `std::thread::spawn` gives one OS thread for something that runs on its own for a long time, like a blocking library with no async version. `std::thread::scope` borrows local data for a few threads that end before the function returns. Fine for a handful; heavy for thousands.

**Channels join them.** `std::sync::mpsc` and `crossbeam-channel` connect threads; `tokio::sync::mpsc` and `broadcast` connect tasks. Crossbeam's channels are faster and can `select!` between several, useful in thread-heavy code.

**Other runtimes.** `smol` and `async-std` (no longer maintained) are other async runtimes. tokio is what axum, tonic, reqwest and most of the ecosystem build on, so pick it unless a library needs something else.

**Mixing them.** From async code, hand heavy or blocking work to `tokio::task::spawn_blocking`: it runs on a separate pool of threads and the task `.await`s the result, so the runtime keeps serving everything else. Inside that closure, rayon is fine. The next quest does exactly this.

**In short.** Waiting on many things: tokio. The same computation over many items: rayon. One long or blocking job: a thread, or `spawn_blocking` from async code.
