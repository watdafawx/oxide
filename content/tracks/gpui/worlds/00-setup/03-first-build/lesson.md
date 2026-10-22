Kinetik's window needs GPUI, and GPUI lives on crates.io. A crate gets into your project by one line in `Cargo.toml`. This quest adds that line and starts the long first build.

**The word for it.** A library your project uses is a **dependency**. Each one is a line under `[dependencies]` in `Cargo.toml`: its name, its version, and sometimes extras.

**Pin the version.** `"=0.2.2"` means exactly 0.2.2. GPUI changes names between versions, and every quest in this track is written against 0.2.2, so the `=` keeps the code and the lessons in step.

**The extra.** `features = ["test-support"]` switches on GPUI's test tools. They let a test open a window without a screen, which is how your quests get checked:

```toml
gpui = { version = "=0.2.2", features = ["test-support"] }
```

**Then wait once.** Saving starts the first build: about 2 minutes, while about 200 crates compile. The objective turns green when it is done, and nothing in this track takes that long again.

## Your turn

- In `Cargo.toml`, add the `gpui` line on the empty line under `[dependencies]`.
- Save, and let the build run.
