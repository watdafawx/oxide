rl-hud's rule for server changes: replay recorded games before and after, diff everything the server does, and it "must print Identical". The same check, built in.

**One step function.** `Pipeline::step` takes an event and a time and gives back actions: no sockets, no clock of its own. `replay` feeds it a recorded match line by line on the recording's own clock.

**Golden file.** `fixtures/match.golden` holds the lines a known-good run printed. A change that moves a single trigger shows up as a changed line.

**Line by line.** The diff walks both lists position by position, to the longer one's end, and shows only what differs:

```rust
(Some(old), Some(new)) if old == new => {}
```

**Changes on purpose.** When a change is meant (a new trigger), the diff shows exactly what moved; then save the new run as the golden file.

## Your turn

- Write `diff_lines`: walk both lists to the longer one's length.
- Push `- old` and `+ new` where they differ. Save.
