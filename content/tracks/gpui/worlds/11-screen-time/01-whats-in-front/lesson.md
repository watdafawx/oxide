Kinetik's screen time shows how long you spent in each program today. It starts from one question, asked every few seconds: which program is in front?

**Window to program.** `GetForegroundWindow` gives the window in front. `GetWindowThreadProcessId` gives the process that owns it, and `QueryFullProcessImageNameW` gives that process's full path, such as `C:\Games\Minecraft.Windows.exe`. `RealDesk` in `screen.rs` already does all three.

**Behind a trait.** The counter never calls Windows itself: it asks a `Desk`, the way the bar asks a `WinApi`. Tests hand it a `FakeDesk` with whatever program, idle time and clock they need.

**Names, not paths.** A full path says where you keep your programs, and nobody needs that. Kinetik keeps only the file name, lower-cased with `to_lowercase()`, because Windows ignores case in file names: `Chrome.exe` and `chrome.exe` are one program and should count together.

**From the right.** `rsplit(sep)` cuts a text into pieces and hands them out from the end, so `.next()` is the last piece. Given an array of `char`s, it cuts at any of them, which covers both `\` and `/`. Here it cuts at dots:

```rust
"minecraft.windows.exe".rsplit(['.']).next() // Some("exe")
```

**Always a piece.** `next()` gives an `Option`, because an iterator may be empty. `rsplit` always gives at least one piece, the whole text when there is nothing to cut at, so `.unwrap_or(path)` is only there to turn the `Option` into a `&str`.

## Your turn

- Write `exe_name`: the part of `path` after the last `\` or `/`.
- Lower-case it. Save.
- Press Run and switch between programs: the terminal prints the one in front every 5 seconds.
