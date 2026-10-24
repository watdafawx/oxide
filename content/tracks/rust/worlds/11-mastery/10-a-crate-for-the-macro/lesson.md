The next quests write proc macros: a derive and an attribute. Before any code, two questions: which kind of macro does what, and why does each quest's folder suddenly hold two crates?

**Derive: add next to it.** `#[derive(Describe)]` on a struct or enum hands the macro the type's tokens; whatever it returns is added after the type, usually an `impl` block. The type itself is never changed. `Debug`, `Clone` and serde's `Serialize` are derives.

**Attribute: replace it.** `#[counted]` above a function hands the macro the whole function, and what it returns takes the function's place. It can wrap the body, add code before it, or add new items next to it. `#[tokio::main]` and `#[test]` work this way.

**Function-like: looks like macro_rules.** `sql!(SELECT * FROM users)` is called like `vec!`, but a Rust function reads the tokens, so it can check and parse anything. This world writes the first two kinds.

**Its own crate.** A proc macro runs inside the compiler, while your crate is being built. So Cargo builds the macro crate first, as a tool, then runs it on your code. It cannot sit in the crate it changes: that crate does not exist yet. A macro crate says so in its `Cargo.toml` with `[lib]` and `proc-macro = true`, and may export only macros.

**Several crates, one workspace.** A **workspace** is a folder of crates that build together: they share one `Cargo.lock` and one `target` folder, where the build output goes. In the next quests the root crate holds the tests, and the macro crate sits in a subfolder, listed in the root's `[workspace]` table and used as a `path` dependency:

```toml
[dependencies]
describe_macro = { path = "describe_macro" }

[workspace]
members = ["describe_macro"]
```
