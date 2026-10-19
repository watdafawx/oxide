**Why now:** the hasher shares a progress counter and a results list between workers. `Send` and `Sync` are why the compiler accepts `Arc<Mutex<..>>` there and rejects `Rc`.

Rust stops data races at **compile time**. It does it with two traits you almost never write by hand: `Send` and `Sync`.

- **`Send`**: ownership of a `T` can be moved to another thread. `thread::spawn` requires it for everything the worker's closure captures.
- **`Sync`**: a `&T` can be shared between threads. (`T` is `Sync` exactly when `&T` is `Send`.)

The compiler adds them automatically when every field has them.

| Type | Send | Sync | Why |
|---|---|---|---|
| `u64`, `String`, `Vec<ModJar>` | yes | yes | plain owned data |
| `Arc<T>` | yes | yes | atomic counter (needs `T: Send + Sync`) |
| `Mutex<T>` | yes | yes | lock guards access |
| `Cell<T>`, `RefCell<T>` | yes | **no** | mutation through `&` without a lock |
| `Rc<T>` | **no** | **no** | non-atomic counter |

```rust
use std::rc::Rc;
use std::thread;

let progress = Rc::new(0);
// error[E0277]: `Rc<i32>` cannot be sent between threads safely
// thread::spawn(move || println!("{progress}"));
```

**Why not `Rc`?** Cloning bumps a plain integer. Two workers doing it at the same time can lose an update, the count ends too low, and the value is freed while still in use. `Arc` does the same job with an atomic counter, a little slower, so you pay only when you need threads.

Next quests use `Mutex`, `Arc` and friends that are `Send + Sync` for you. When an error mentions "cannot be sent between threads", look for an `Rc` or `RefCell` hiding in your data.
