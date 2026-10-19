**Why now:** the hasher hands each worker a list of jar names, and the thread must own that list because the function that built it returns first.

`thread::spawn` cannot know when your function returns, so the thread might still run after your local variables are gone. Rust refuses to let it **borrow** them.

```rust
use std::thread;

let jars = vec![String::from("sodium.jar"), String::from("iris.jar")];

// Does not compile: "closure may outlive the current function"
// let h = thread::spawn(|| println!("{}", jars.len()));

let h = thread::spawn(move || {      // move: the closure now OWNS jars
    println!("{}", jars.len());
});
h.join().unwrap();
// println!("{:?}", jars);           // error: jars was moved into the thread
```

- **`move`** makes the closure take ownership of every variable it uses.
- After the move you cannot use the value in the parent. To keep one, `.clone()` first and move the clone.
- Send **owned** data (`String`, `Vec<T>`), not references. A `&str` borrowed from a local would not live long enough.
- To get data back, return it from the closure and read it with `join()`.

## Your turn

In `src/lib.rs`:

- `greet_in_thread`: move the `String` in, return `"Hello, <name>!"` from the thread.
- `sum_in_thread`: move the `Vec<i32>` in, return its sum.
- `split_work`: split the Vec in half with `split_off`, sum each half on its own thread, return both sums.
