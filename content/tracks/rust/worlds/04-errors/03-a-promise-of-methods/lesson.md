The todo list shows a short line for everything it knows: a task's status, and from now on also the error from a broken line. Each type could get its own method with its own name, but then code that wants "anything that can describe itself" has nothing to ask for.

**Same question, many types.** A `Status` describes itself as `open` or `done`. A `PriorityError` should describe itself as `no priority` or `'high' is not a number`. Same question, different types, and they need to agree on one method name.

**A promise of methods.** A **trait** is a named list of methods with no bodies, only their signatures: `trait Label { fn label(&self) -> String; }`. It is a promise a type can make: "I have these methods, with exactly these signatures".

**Keep the promise.** `impl Label for Status { ... }` is a type keeping it. It is the `impl` block from the foundations world with `Label for` added, and it must hold a body for every method the trait lists. Leave one out and the build stops.

```rust
impl Label for Status {
    fn label(&self) -> String {
        match self {
            Status::Open => "open".to_string(),
            Status::Done => "done".to_string(),
        }
    }
}
```

**Why a promise.** Because the promise has a name, other code can ask for it: "give me anything that is `Label`", and the compiler checks every call before the program runs. The standard library is full of such promises, and the next quests keep three of them for your error: one prints it, one converts other errors into it, one marks it as an error.

**You have seen two.** `#[derive(Debug, PartialEq)]` above your enums asks the compiler to write the impls of two standard traits for you: `Debug`, printing with `{:?}` for programmers, and `PartialEq`, comparing with `==`. Derive only works when the body is obvious. A message for humans is not obvious, so you write that one.

**The compiler notices.** The tests call `.label()` on `PriorityError`, which never made the promise, so the starter does not build yet. Read the note at the bottom: it names the trait you need to implement.

```text
error[E0599]: no method named `label` found for enum `PriorityError` in the current scope
note: `Label` defines an item `label`, perhaps you need to implement it
```

## Your turn

- Below the `PriorityError` enum, write `impl Label for PriorityError { }`.
- Inside, write `fn label(&self) -> String`, copying the signature from the trait.
- In the body, `match self` and build the text for each variant with `format!` or `.to_string()`.
