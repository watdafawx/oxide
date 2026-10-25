Writing `Display`, `Error` and `From` by hand for every error type is a lot of typing, and Rust programmers solved it once and shared the result. The next quests use two shared libraries for errors: thiserror and anyhow. Adding one to your project works a lot like adding a library mod to a modpack.

**A crate is a package.** A **crate** is one Rust project: a library or a program, with its own `Cargo.toml`. Every quest you have solved was a crate. Thousands more are published on crates.io, and **Cargo**, Rust's build tool, can download any of them for you.

**Cargo.toml is the list.** `Cargo.toml` is your crate's mod list: it names your crate and every crate it needs. The `[package]` section says who your crate is. `[dependencies]` lists crates your code needs when it runs. `[dev-dependencies]` lists crates only the tests need, and they never ship with the program.

```toml
[package]
name = "modpack_loader"
version = "0.1.0"
edition = "2024"

[dependencies]             # needed when the program runs
thiserror = "2"
anyhow = "1.0"             # means ^1.0: 1.0.0 up to, not including, 2.0.0

[dev-dependencies]         # only for tests
tempfile = "3"
```

**A version is a range.** `anyhow = "1.0"` does not mean exactly 1.0.0. Crates follow **semver**: a new middle or last number only adds or fixes things, while a new first number, like 2.0, may break your code. So Cargo may pick any `1.x` that is at least `1.0`, and never `2.0`. It is the same reason a mod's 2.0 release may not load your old worlds.

**The lock file pins it.** The first build writes **`Cargo.lock`** with the exact versions Cargo chose. Later builds reuse them, so the program builds the same way tomorrow and on someone else's machine.

**cargo add.** `cargo add anyhow` finds the newest version and writes the line for you. `cargo add --dev tempfile` puts it under `[dev-dependencies]`. In this app the quests come with their `Cargo.toml` ready, so you only read it.

## Your turn

- Answer the two questions. No code this time.
