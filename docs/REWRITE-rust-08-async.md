# Rust world 08-async: rewrite map

World 08 was rewritten in the golden shape (`docs/CONTENT.md`, `content/golden/`), following the audit
`docs/audit-2026-10-24/rust-08-10.md` (score 5.5). Each quest teaches one idea and has an unlabeled intro, kicker
cards, one code block that shows a similar example (not the solution), a short Your turn, two hints with the exact
code only in the last one, and two points. Every quest has both flavours with the same objectives in the same
order. The plain flavour is now a **podcast downloader** (episodes, feeds), so it no longer uses mod/jar words.
The games flavour is the **mod downloader** (jars, modpacks, mirrors, launcher). Ledger rows for the parent:
`scratchpad/ledger/rust-08-async.md`.

## 12 quests became 25

| old | new |
|---|---|
| (new) | 01-waiting-is-not-working (read): why async exists. CPU work vs waiting on I/O, why a thread per wait is costly (2 MB stack, OS switches), what a future, a task and a runtime are, and the one-cook-many-pots picture |
| 01-first-async-fn (3 bullets, `greet`/`add_async`) | 02-a-future-is-a-ticket (`async fn` returns a future, `.await`, E0277 when you forget it, E0728 outside async, `impl Future` for the curious), 03-who-runs-the-future (**runtime** by hand with `Runtime::new` + `block_on` from a plain `#[test]`. What the `#[tokio::main]` / `#[tokio::test]` **attribute macro** writes, why Rust has no built-in runtime, runtime vs thread, don't `block_on` inside a runtime) |
| 04-futures-are-lazy (read, "poll" unexplained) | 04-futures-are-lazy (slug kept. Calling builds, awaiting runs. Poll and waker in plain words, the silent forgotten `.await` warning, the state machine for the curious) |
| 02-join (4 objectives, 8 min) | 05-two-at-once (in-turn vs `join!`, 3 objectives incl. timing; `join_all` for the curious) |
| (part of 04 read) | 06-dont-block-the-runtime (read: giving the thread back, what blocking is, async twins, `spawn_blocking` for CPU work) |
| 03-spawn-tasks (3 ideas) | 07-spawn-a-task (task, `JoinHandle`, `async move` block vs `async fn`, `'static` and `Send` explained), 08-when-a-task-panics (`JoinError`, `.ok()`, quick win) |
| (new, needed by GPUI) | 09-a-closure-that-waits (**async closures**: `async \|id\| ..`, `async move \|id\| ..`, `impl AsyncFn`, block vs closure, GPUI's `cx.spawn(async move \|this, cx\| ..)`, the older `\|x\| async move {}` spelling) |
| 05-mpsc-channels | 10-tasks-that-talk (backpressure and bounded explained, `while let` refresher, TODOs in words) |
| 06-timeouts-and-select (4 objectives, 3 ideas) | 11-give-up-in-time (`timeout`, dropping = cancelling), 12-first-one-wins (`select!`, binding the winner, losers dropped, `biased;` for the curious), 13-a-cancel-button (`oneshot`, receiver is a future, `&mut`) |
| 06-which-future-wins (read) | 14-which-future-wins (read: stops at an await, undoes nothing, `.part` files stay, spawned tasks keep running, cancel safety for the curious. Its old explicit `requires` was removed) |
| 07-shared-state (tokio Mutex + RwLock + yield_now, 12 min) | 15-one-counter-many-tasks (tokio `Mutex` + `Arc`, **why** a std guard can't cross `.await`: not `Send`, blocks the worker; when std is fine; the lost update explained), 16-many-readers-one-writer (`RwLock`, `.cloned()` out of a guard; `#[derive(Default)]` replaced by a written `new`) |
| 08-streams (3 objectives) | 17-a-stream-of-chunks (2 objectives: filter+map, take; why `ready`; `fold` moved to for-the-curious) |
| 09-cancellation | 18-stop-every-task (CancellationToken, loop around `select!`, cancel then await) |
| (new) | 19-requests-and-json (read: request/response, method, path, status codes 2xx/4xx/5xx and which to retry, JSON objects/arrays/null, (de)serialize) |
| 10-http-client (HTTP, JSON, serde untaught) | 20-ask-a-server (`reqwest::Client`, `get().send()`, `error_for_status`, `.json()` into `Value`; `.bytes()`/`.text()` named), 21-json-into-a-struct (**minimal serde bridge**: `#[derive(Deserialize)]`, `.json::<Vec<T>>()` turbofish, missing field = Err, extra fields skipped. world 10 deepens it) |
| (new, used first in the boss) | 22-at-most-n-at-once (**bridge**: `Semaphore`, `acquire_owned` and why owned, the `let _` vs `let _permit` trap), 23-a-set-of-tasks (**bridge**: `JoinSet`, `join_next` in finish order, `abort_all`, `is_cancelled`, a dropped set aborts), 24-errors-from-a-task (**bridge**: `type Error = Box<dyn Error + Send + Sync>`, E0282 for `?` in an async block, `Ok::<T, Error>(..)`, `result??`) |
| 11-boss-mod-downloader | 25-boss-mod-downloader (slug kept, renumbered to stay last, same 8 objectives and test names. Plain is now the "Episode downloader". The **games flavour is new** and has the mod downloader text. The "New tools" glossary is gone: each card names the quest that taught the tool. TODOs are in plain words, `verify` uses a `for`/`match` instead of the untaught `is_ok_and`, and `reqwest::get`, `tokio::fs`, `to_path_buf` and `std::fs::metadata` are named in the cards. 7 hints) |

All `**Why now:**` labels are gone. Every exercise is 3-8 minutes with 1-3 objectives. Quiz and point answers vary
in position.

## Ids and references

- **Boss renamed:** `rust/08-async/11-boss-mod-downloader` → `rust/08-async/25-boss-mod-downloader`. Referenced by
  `content/tracks/gpui/worlds/03-live/01-state-in-a-view/quest.toml` (`requires`) and by `docs/tracks/gpui.md`
  line 122. I did not edit either, because gpui is off-limits for this round, so the parent must update both.
  Until then `content-check --fast` reports that one `requires` as unknown.
- No other `requires` anywhere names an 08 quest. `01-waiting-is-not-working` has no `requires`, so it still chains
  to the last quest of 07.
- Every other old quest folder was removed. Only `04-futures-are-lazy` kept its slug.
- Cargo package names are new (`async_future_ticket`, … `async_boss_downloader`).

## Notes for other worlds

- This world assumes 07 teaches threads, `move` closures, `std::sync::mpsc`, `Send`, `Arc`, `Mutex` and atomics
  (`fetch_add`, `Ordering::SeqCst`).
- serde is now first taught in 08/21 (`Deserialize` only). 10/03 can call it a refresher and add `Serialize` and
  attributes.
- `spawn_blocking` is named in 08/06. 12/14 still teaches it in full.
- GPUI: async closures are taught in `08-async/09-a-closure-that-waits`, so GPUI lessons can link there.
