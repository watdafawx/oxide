In the last quest the compiler knew that an `Rc` must not go to another thread, while an `Arc` may. It did not guess: two traits from the standard library carry that knowledge, and every error that says "cannot be sent between threads safely" comes from them.

**Traits as labels.** `Send` and `Sync` have no methods at all. They are **marker traits**: a type either has the label or not, and the compiler checks the label wherever threads are involved. `thread::spawn` demands `Send` for everything its closure takes along.

**Send: may move.** A type is `Send` when a value of it may be moved to another thread. `String`, `Vec<T>`, numbers and `Arc<T>` are `Send`. `Rc<T>` is not, because two threads holding clones could race on its counter.

**Sync: may be shared.** A type is `Sync` when a `&` reference to it may be used from several threads at once. Plain data is `Sync`: reading the same number from two threads is harmless. `RefCell<T>` is `Send` but not `Sync`: you may hand a whole `RefCell` to another thread, but two threads must not share one, because its borrow count is a plain number too.

**You almost never write them.** The compiler adds both labels by itself: a struct is `Send` when every field is `Send`, and `Sync` when every field is `Sync`. So one `Rc` deep inside a struct makes the whole struct stay on its thread.

| type | Send | Sync | why |
|---|---|---|---|
| `i32`, `String`, `Vec<T>` | yes | yes | plain owned data |
| `Arc<T>` | yes | yes | atomic counter (needs `T: Send + Sync`) |
| `Mutex<T>` | yes | yes | a lock makes threads take turns (quest 11) |
| `Cell<T>`, `RefCell<T>` | yes | **no** | change through `&` with no lock |
| `Rc<T>` | **no** | **no** | plain counter |

**Reading the error.** When an error says a type "cannot be sent between threads safely" or "cannot be shared between threads safely", look for an `Rc`, `Cell` or `RefCell` hiding in your data. Swap it for `Arc`, an atomic, or a lock, which are the next quests.

## Your turn

Answer the questions, then start the next quest.
