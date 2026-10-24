This world builds a todo list that lives in a file. People type that file by hand, so some lines will be broken: a priority of `high` instead of `2`, a missing `|`, an empty line. Before writing any code, here is how Rust thinks about things going wrong.

**Exceptions jump.** In many languages, a failing function **throws an exception**, which means it stops and jumps straight to some `catch` block further up, maybe in a different file. Nothing in the function's signature warns you, so it is easy to forget the catch and crash.

**Rust returns instead.** A Rust function that can fail hands back a value like any other: a `Result`, either `Ok(value)` or `Err(reason)`. You met it in the foundations world. The failure travels the normal way, through `return`, and nothing jumps anywhere.

**The type is the warning.** `fn parse_priority(text: &str) -> Result<u32, String>` says "a number, or a reason why not" right in the signature. You cannot add the result to another number without first dealing with the `Err` case, so the compiler reminds you every time.

```rust
let priority: Result<u32, String> = Err("'high' is not a number".to_string());
let note = match priority {
    Ok(n) => format!("priority {n}"),
    Err(reason) => format!("skipped a line: {reason}"),
};
```

**An error is just data.** Because an `Err` is a normal value, you can store it, match on it, add a line number to it or turn it into a message. The quests ahead do exactly that: first an error type with one variant per kind of failure, then the tools that pass errors up with little typing.

**One real crash.** Rust also has **panic**, which stops the program on the spot. It is for bugs in your own code, never for a badly typed line. A later quest in this world says exactly where the line is.

## Your turn

- Answer the two questions. No code this time.
