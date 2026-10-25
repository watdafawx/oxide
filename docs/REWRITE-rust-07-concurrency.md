# Rust world 07: rewrite map

World 07 was rewritten in the golden shape (`docs/CONTENT.md`, `content/golden/`): one idea per quest, unlabeled intro, kicker cards, one code block that shows a similar example (not the solution), a short Your turn, two hints with exact code only in the last, and points that ask about what the card just showed. Every quest has a plain flavour (a file hasher: files, sizes, checksums) and a games flavour (the Minecraft mod-folder hasher: jars, `mods/`, the launcher), with the same objectives in the same order. Audit: `docs/audit-2026-10-24/rust-05-07.md` (World 07). Ledger rows: the scratchpad `ledger/rust-07-concurrency.md`, which the parent merges.

The audit's main gaps are now taught where they are first needed:

- **Thread, process, core, and why output order changes between runs**: new read quest 01.
- **Data race** (what `+=` from two threads corrupts) and **data race vs race condition / deadlock**: new read quest 08, before `Arc`, `Send`/`Sync` and `Mutex` rely on it.
- **`'static`**: named in 03-move-closures, where the constraint first appears, and reused in 15.
- **Atomic**: used in 09-arc ("atomically reference counted") with a one-line gloss, then taught in 18.
- **Recursion over folders** (old boss): its own quest, 20, which points back to rust 06's walk-the-chain.
- **Hex literals, XOR, `wrapping_mul`, `{:016x}`** (old boss glossary): quests 21 and 22.
- **`Arc<Mutex<Receiver>>` pool protocol** (old boss hints): quest 23, which also shows how to read the type inside out.

## 11 quests became 24

| old | new |
|---|---|
| — | **01-two-things-at-once** (read, new: thread, process, main thread, core, nondeterministic order, when threads pay off) |
| 01-spawn-and-join (JoinHandle, join, move preview, `.ok()`, panic isolation and ordering in one 4-min quest; `\|&n\|` used untaught) | 02-spawn-and-join (spawn, JoinHandle, join), 04-start-them-all (start every thread before joining any; order kept), 05-a-crash-stays-put (panic ends only its thread, `join` → `Err`, `.ok()`). No `\|&n\|` |
| 02-move-closures (the audit's model) | 03-move-closures (kept the shape: E0373 → why → `move` → consequence; adds `'static` by name) |
| 03-channels | 06-channels (one sender: send, recv, `for x in rx`, when the channel closes), 07-many-senders (`tx.clone()`, `drop(tx)`, mpsc; bounded `sync_channel` / backpressure as a for-the-curious card) |
| — | **08-data-races** (read, new: `+=` is three steps, lost update, data race, Rust prevents it at compile time; race condition and deadlock are still possible) |
| 04-send-and-sync (read; "data race" and "atomic" untaught) | 09-arc (Rc refused across threads with E0277, Arc as the thread-safe Rc; moved up from old 06), 10-send-and-sync (read: marker traits as labels, the Send/Sync table, reading E0277) |
| 05-mutex (guards, `mem::take(&mut *guard)`, deadlock, poisoning, `unwrap_or_else`, `into_inner`) | 11-mutex (lock, guard, drop unlocks, self-deadlock), 12-poisoned-lock (poisoning, `unwrap_or_else(\|e\| e.into_inner())`). `mem::take(&mut *guard)` is gone |
| 06-arc-mutex-counter (10 min) | 13-arc-mutex (stack them, one clone per thread, join then read) |
| 07-rwlock (4 objectives) | 14-rwlock (3 objectives: read, write, `try_write` refused while reading) |
| 08-scoped-threads (`'static` untaught, split_at/chunks/split_at_mut/chunks_mut in two bullets, 10 min) | 15-scoped-threads (E0521, `thread::scope`, borrowing halves), 16-split-at-mut (E0499 on two `&mut` halves, `split_at_mut`; `starter_fails_to_compile = true` on purpose). `chunks`/`chunks_mut` cut |
| 09-rayon | 17-rayon (par_iter, pool, order kept, closures run on every thread so need Send/Sync) |
| 10-atomics | 18-atomics (fetch_add, load, Relaxed vs SeqCst, when atomics stop being enough; Acquire/Release as for-the-curious), 19-claim-once (race condition with load-then-store, `compare_exchange`) |
| — | **20-walk-a-folder**, **21-hash-the-bytes**, **22-print-in-hex**, **23-a-worker-pool** (bridge quests for the boss: `read_dir` + recursion, FNV-1a with hex literals/XOR/`wrapping_mul`, `{:016x}`, the `Arc<Mutex<Receiver>>` pool) |
| 11-boss-parallel-file-hasher (no games flavour, glossary lesson, hints gave the FNV constants and the full pool protocol, untaught `#![allow(..)]`, literal-newline rendering bug) | 24-boss-parallel-file-hasher (slug kept, renumbered to stay last). **New games flavour** (mod-folder hasher, 8 themed objectives in the same order). The lesson points each piece back to its quest instead of a glossary. FNV constants live in the `fnv1a` doc comment (spec, not skill). `#![allow]` removed. `"\n"` is escaped |

Cut: `mem::take(&mut *guard)`, `chunks` / `chunks_mut`, `#![allow(..)]` in the boss starter, and the "Why now:" labels (plain and games).

## Id changes and references

- `rust/07-concurrency/11-boss-parallel-file-hasher` → `24-boss-parallel-file-hasher`. **Referenced by** `content/tracks/gpui/worlds/04-sensors/01-what-needs-admin/quest.toml` (`requires`). Not edited here (other agents own gpui); the parent updates it. Until then, `content-check --fast` reports `requires unknown quest` for that line.
- All other old ids (01-10) were renamed or split. A grep of `content/` found no other `requires` naming a 07 quest, inside or outside the rust track.
- 01-two-things-at-once has no `requires`, as the old 01 had none, so it still chains to the last quest of world 06.

## Notes

- Quiz/point answers were rebalanced across positions 0-3 (the app shuffles anyway).
- Tests that use channels or pools run the learner's code on a helper thread with a time limit, so a forgotten `Sender` fails instead of hanging. The lessons say so.
- 16-split-at-mut is a compile-error quest: E0499 is the lesson.
