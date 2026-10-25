rl-hud's rule for server changes: replay recorded games before and after the change, diff everything the server does, and it "must print Identical". The same check, built in as a test.

**One step function.** `Pipeline::step` takes an event and a time and gives back actions: no sockets, no clock of its own. `replay` (already written) feeds it a recorded match line by line on the recording's own clock, so the same recording gives the same lines on every run.

**Golden file.** `fixtures/match.golden` holds the lines a known-good run printed: the **golden file**, the answer everything is checked against. A change that moves a single trigger shows up as a changed line.

**Position by position.** The diff walks both lists to the end of the longer one, `0..golden.len().max(now.len())`. Past the end of the shorter list `.get(i)` gives `None` instead of panicking, so a line that only one side has still shows up.

**A pair of Options.** Matching on a tuple `(golden.get(i), now.get(i))` looks at both sides at once, and a guard can compare them. The same shape, lining up two teams' players:

```rust
for i in 0..blue.len().max(orange.len()) {
    match (blue.get(i), orange.get(i)) {
        (Some(b), Some(o)) if b == o => println!("{b} on both teams?"),
        (Some(b), Some(o)) => println!("{b} vs {o}"),
        (Some(b), None) => println!("{b} vs nobody"),
        (None, Some(o)) => println!("nobody vs {o}"),
        (None, None) => {}
    }
}
```

**Changes on purpose.** When a change is meant (a new trigger), the diff shows exactly what moved: `- old` for the golden line, `+ new` for this run's. Check it, then save the new run as the golden file.

## Your turn

- In `diff_lines`, walk both lists to the longer one's length and match on the pair of lines at each position.
- Where they are equal, push nothing; otherwise push `- old` and `+ new` for whichever exists. Save.
