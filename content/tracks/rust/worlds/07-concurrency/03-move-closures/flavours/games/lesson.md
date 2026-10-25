The hasher gives each worker thread a jar name to work on. The name lives in a variable of the function that starts the worker, and that function may return long before the worker is done. Rust has to make sure the worker never reads a variable that is already gone.

**Try the obvious thing.** The starter's `label_in_thread` uses `jar_name` inside the closure, just as you would in a loop. The compiler refuses:

```text
error[E0373]: closure may outlive the current function, but it borrows `jar_name`,
              which is owned by the current function
help: to force the closure to take ownership of `jar_name`, use the `move` keyword
```

**Why it refuses.** A closure that only reads a variable **borrows** it, like `&jar_name`. `thread::spawn` cannot know when your function returns, so the worker might still run after `jar_name` was dropped at the closing brace. That borrow could point at freed memory, which is exactly what the borrow checker exists to stop.

**The fix: move.** Write `move` in front of the bars, `move || ...`, and the closure takes **ownership** of every variable it uses instead of borrowing it. The value now belongs to the worker and lives as long as the worker needs it. The price: the function that started the worker cannot use that variable any more, because it was moved. Clone it first if you need to keep a copy.

**The word for it: 'static.** `thread::spawn` asks for a closure that is `'static`, which means it holds nothing borrowed that could die before the program ends. Owned values like `String` and `Vec` pass, and so do string literals: `"sodium.jar"` is a `&'static str`, written into the program itself, so it never goes away. A borrow of a local variable does not pass, which is what E0373 told you.

```rust
use std::thread;

let jars = vec![String::from("iris.jar"), String::from("jei.jar")];

let handle = thread::spawn(move || {   // move: the closure now OWNS jars
    jars.len()
});
println!("{}", handle.join().unwrap()); // 2
// println!("{jars:?}");                // error: jars was moved into the thread
```

## Your turn

- Save and read the E0373 error. Its `help:` line names the fix.
- In `label_in_thread`, add `move` so the worker owns the jar name. The first objective turns green.
- In `total_in_thread`, move the `Vec` of sizes into a thread, sum it there, and return the sum from `join`.
