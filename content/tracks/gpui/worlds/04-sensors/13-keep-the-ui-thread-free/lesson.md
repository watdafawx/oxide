Run the bar on a slow network and it stutters: every second, ping waits up to a second for an answer, and nothing can draw or answer the mouse meanwhile. The reads have to happen somewhere else, and only their results come back to the bar. This quest is the map; the next two build it.

**One thread draws.** A GPUI app has one **UI thread**. It runs the event loop: it handles the mouse and keyboard, calls `render`, and paints. Every `update`, every `cx.spawn` task and every timer callback you have written so far ran on it. While your code runs there, the app can do nothing else, so a one-second ping is a one-second freeze.

**Waiting is not blocking.** `.await` on a timer does not freeze the UI thread: the task steps aside, and the event loop runs other work until the timer is due. A ping is different. `connect_timeout` is plain code that sits on the thread until it returns, and `.await` cannot make it step aside.

**Two executors.** GPUI has two places to run a task. `cx.spawn(..)` runs on the **foreground executor**: the UI thread, where the task may touch entities and `cx`. `cx.background_executor().spawn(async move { .. })` runs on the **background executor**: a pool of worker threads, where slow code freezes nothing, but where entities and `cx` cannot go, because they live on the UI thread only.

**The hop.** So the work hops out and back, inside the foreground task. First copy what the work needs out of the bar with `this.read_with(..)`. Then hand that to the background executor. Its `spawn` gives back a `Task<T>`, a future of the result, and `.await` waits for it without blocking. Last, `this.update(..)` stores the result on the UI thread and redraws. The same hop, counting a log file's lines:

```rust
let Ok(path) = this.read_with(cx, |page, _| page.log_path.clone()) else { break };
let lines = cx.background_executor().spawn(async move { count_lines(&path) }).await;
if this.update(cx, |page, cx| page.show_count(lines, cx)).is_err() { break; }
```

**Let, or else.** `let Ok(path) = .. else { break };` is **let-else**: if the value matches the pattern, its parts get names and the code goes on; if not, the `else` block runs, and it must leave (`break`, `return`). `read_with` fails once the bar is gone, so this stops the loop the way `is_err()` did in the Live world.

**Shared across threads.** The sensors themselves must travel to the worker and stay in the bar. The Rust track's concurrency world built that: `Arc` lets two threads own one value, and `Mutex` lets one of them use it at a time. So the bar keeps `Arc<Mutex<Vec<Box<dyn Sensor>>>>`, and each tick hands the worker a clone of the `Arc`, which is a second handle to the same sensors, not a copy of them. Everything that enters the background `spawn` must also be **Send**, which that world defined as safe to move to another thread; the next quest meets that check.

**A poisoned lock.** `lock()` fails if another thread panicked while holding the lock: the mutex is **poisoned**, in case the data was half-changed. `unwrap_or_else(|poisoned| poisoned.into_inner())`, in `read_sensors`, takes the data anyway. One sensor's panic should not stop the bar forever.
