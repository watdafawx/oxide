# Playtest 2026-10-20b: Rust worlds 07 concurrency, 08 async, 09 systems

Method: paper playtest, not a stopwatch run. For every quest I read lesson, objective labels, starter and
tests (plain and games twin), then opened `solution/` and listed every call, keyword, macro or pattern not
shown in that lesson or in worlds 00-06 (verified by grepping the earlier lessons). **I did not run
`cargo test` on the starters** (no temp copies, no `CARGO_TARGET_DIR`): I know Rust, so my "solved" means
"I could write it, and the gap list below is what a learner would trip on". Hints were not opened.
Starter edits are comment-only; solutions and tests are untouched. `content-check` is green for 07, 08, 09
(it checks the catalog, not the build).

Verdict key: ok = nothing missing; fixed = gap found and patched in lesson/TODO/quiz (both flavours when both had it).

## World 07 concurrency

| Quest | Solved | Hints | Verdict | Missing before | Fixed |
|---|---|---|---|---|---|
| 01 spawn-and-join | yes | 0 | fixed | `move \|\|` used in TODO, taught next quest; `Result::ok()` only in hint | lesson bullets for `move ||` and `.ok()`; TODO names `.ok()` |
| 02 move-closures | yes | 0 | fixed | `split_off` named in TODO only | lesson bullet with example |
| 03 channels | yes | 0 | fixed | `rx.iter().collect()` (lesson shows only `for x in rx`) | lesson bullet |
| 04 send-and-sync (read) | yes | n/a | fixed | answers were option 1 twice and Q2's right answer was the longest | choices rewritten; answers 2 and 0; each decided by the table / "Why not Rc" |
| 05 mutex | yes | 0 | fixed | `*guard += n` on a number, `mem::take`, `unwrap_or_else(\|e\| ..)` all untaught | example + two bullets (plain), two bullets (games) |
| 06 arc-mutex-counter | yes | 0 | fixed | plain `try_unwrap` chain incomplete (`.unwrap()`s); `entry().or_insert` on a shared map (or_default is a one-word variation) | corrected chain; shared-map bullet |
| 07 rwlock | yes | 0 | fixed | `.is_ok()` / `.is_err()` on Result | bullet |
| 08 scoped-threads | yes | 0 | fixed | `split_at`, `chunks`, `iter_mut`, `.copied().max()`, `move` per chunk, `zip` of two slices | bullets (plain and games) |
| 09 rayon | yes | 0 | fixed | `filter(\|&n\| ..).count()` on a parallel range | bullet; games also got the "crate downloads on first build" note. Needs network once for the crate, tests offline after |
| 10 atomics | yes | 0 | fixed | games: `compare_exchange` argument list and its `Result` | bullet with call shape |
| 11 boss file hasher | yes | 0 | fixed | `read_dir`, `.flatten()`, `is_dir`, `let-else`, `u64::from`, `^=`, `{:016x}`, `join("\n")`, `extend`, `max(1)`, worker `recv` loop | "Small tools" block in the lesson |

## World 08 async

| Quest | Solved | Hints | Verdict | Missing before | Fixed |
|---|---|---|---|---|---|
| 01 first-async-fn | yes | 0 | fixed | `#[tokio::test]` shape only named | one sentence |
| 02 join | yes | 0 | ok | none | |
| 03 spawn-tasks | yes | 0 | fixed | `.ok()` on a JoinError | lesson + TODO |
| 04 futures-are-lazy (read) | yes | n/a | fixed | right answer longest in both questions, always option 1 | choices rewritten; answers 0 and 2 |
| 05 mpsc-channels | yes | 0 | ok | none | |
| 06 timeouts-and-select | yes | 0 | fixed | `oneshot` channel, `&mut rx` in `select!`, arms that bind (`result = fut => result`), `.ok()` on timeout | bullets |
| 06 which-future-wins (read) | yes | n/a | fixed | right answer longest, always option 1 | choices rewritten; answers 0 and 2 |
| 07 shared-state | yes | 0 | fixed | `yield_now`, tokio `Mutex::into_inner` has no `Result` (differs from std), `.cloned()` on `Option<&V>` | paragraph (plain), bullets (games) |
| 08 streams | yes | 0 | fixed | `fold` with an async closure never shown, endless `1u64..` range | paragraph (plain), bullet (games) |
| 09 cancellation | yes | 0 | fixed | plain lesson never showed `loop { select! { .. => return .. } }` | code block added (games already had it) |
| 10 http-client | yes | 0 | fixed | `.map(\|s\| s.to_string())` on `Option<&str>` (solution uses `map(str::to_string)`; the closure form is fine) | TODO text. **Offline: yes**, tests start a server on 127.0.0.1; only the first build downloads reqwest |
| 11 boss mod-downloader | yes | 0 | fixed | `client.get().send()`, `bytes()`, `tokio::fs::write/rename`, match guard, `JoinSet`, `res??`, `Ok::<_, Error>`, `Semaphore` | "New tools" block. **Offline: yes**, local test server |

Note: the world has two `06-` folders (`06-timeouts-and-select`, `06-which-future-wins`); the task counted 32 quests, the tree has 33. Not renumbered here.

## World 09 systems

| Quest | Solved | Hints | Verdict | Missing before | Fixed |
|---|---|---|---|---|---|
| 01 bytes-and-endianness | yes | 0 | fixed | `try_into().ok()?` chain (TODO said only `try_into`, which is a Result inside an Option fn) | lesson sentence + TODO (plain and games) |
| 02 bit-ops | yes | 0 | ok | none | |
| 03 binary-parsing | yes | 0 | fixed | `?` inside a struct literal; lesson `take` returns Option but the quest returns Result | bullet + note (plain), bullet (games) |
| 04 the-unsafe-contract (read) | yes | n/a | fixed | right answer longest in both questions, always option 1 | choices rewritten; answers 2 and 1 |
| 05 raw-pointers | yes | 0 | fixed | plain: `is_null()` / `ptr::null()` | bullet |
| 06 ffi | yes | 0 | fixed | `.to_string_lossy().into_owned()` (`Cow`, taught in 06-smart-pointers/08) | bullet (plain), clause (games) |
| 07 tcp-echo | yes | 0 | fixed | `set_read_timeout(Some(Duration))`, `vec![0u8; n]`/`read_exact` (games) | bullets. **Offline: yes**, loopback only |
| 08 udp | yes | 0 | fixed | `to_ascii_uppercase` on a byte slice, `buf[..n].to_vec()`, Duration for timeouts (plain) | bullets. **Offline: yes**, loopback only |
| 09 varints | yes | 0 | fixed | `value as u32` reinterpret and the decode recipe were TODO-only (plain). **Games lesson contained the full `encode_varint` loop** (solution in the lesson) | decode recipe + cast bullets; games encode loop replaced with a `groups` demo and the first-byte arithmetic |
| 10 boss minecraft-pinger | yes | 0 | fixed | `extend_from_slice`, range `.contains`, `io::Cursor` position, `Value` matching/`as_*`/recursion, `connect_timeout` | "Small tools" block |

## Left as is, deliberately

- 06 `or_default` vs taught `or_insert(0)`: variation, either compiles.
- 08/10 `.map(str::to_string)` is only in the solution; closure form works.
- Minutes were not changed. Quests with the most new vocabulary (07/11, 08/11, 09/10) already plan 50-60 min.
- No stopwatch, no compile of starters (see method). A real learner run on 08/10 and 08/11 (first-build time for reqwest/tokio) is the best follow-up.
