Kinetik's window needs GPUI, and GPUI lives on crates.io. A crate gets into your project by one line in `Cargo.toml`. This quest adds that line and starts the long first build.

**The word for it.** A library your project uses is a **dependency**. Each one is a line under `[dependencies]` in `Cargo.toml`: its name, its version, and sometimes extras.

**Pin the version.** A plain `"0.2.2"` lets Cargo pick any newer 0.2 release. `"=0.2.2"` means exactly 0.2.2 and nothing else. GPUI renames things between versions, and every quest in this track is written against 0.2.2, so the `=` keeps the code and the lessons in step.

**The extras.** Some crates have parts you switch on by name, called **features**. GPUI's `test-support` feature brings its test tools: they let a test open a window without a screen, which is how your quests get checked.

**One line, three parts.** Here is the same shape for a crate a later world adds, the one that reads CPU usage. Its name, an exact version, and one feature:

```toml
sysinfo = { version = "=0.39.6", features = ["system"] }
```

**Then wait once.** Saving starts the first build: about 2 minutes, while about 200 crates compile. The objective turns green when it is done, and nothing in this track takes that long again.

## Your turn

- In `Cargo.toml`, under `[dependencies]`, add `gpui` pinned to exactly version 0.2.2, with the `test-support` feature.
- Save, and let the build run.
