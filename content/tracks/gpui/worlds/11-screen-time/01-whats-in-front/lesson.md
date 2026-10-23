Kinetik's screen time shows how long you spent in each program today. It starts with one question, asked every few seconds: which program is in front?

**Window to program.** `GetForegroundWindow` gives the window in front. `GetWindowThreadProcessId` gives the process that owns it, and `QueryFullProcessImageNameW` gives that process's full path. `RealDesk` in `screen.rs` already does all three.

**Behind a trait.** The counter never calls Windows itself: it asks a `Desk`, like the bar asks a `WinApi`. Tests hand it a `FakeDesk` with whatever program, idle time and clock they want.

**Names, not paths.** A full path says where you keep your programs, and nobody needs that. Kinetik keeps only the file name, lower-cased so `Chrome.exe` and `chrome.exe` count together:

```rust
path.rsplit(['\\', '/']).next().unwrap_or(path).to_lowercase()
```

**From the right.** `rsplit` gives the pieces from the end, so the first piece is the file name. An array of two `char`s splits on either slash.

## Your turn

- Write `exe_name` in `lib.rs`: the part after the last slash, lower-cased.
- Save. Press Run and switch between programs: the terminal prints the one in front every 5 seconds.
