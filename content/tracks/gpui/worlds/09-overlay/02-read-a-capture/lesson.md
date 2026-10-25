`fixtures/capture.csv` is a recorded game session: 1200 frames, shaped like a cut-down PresentMon capture. Testing FPS maths on a recording means the right numbers are known in advance; a live game is different every second.

**A header, then rows.** The first line names the columns: `Application,ProcessID,TimeInSeconds,MsBetweenPresents,FrameType`. Every other line is one frame. The starter already reads the header and finds the position of each column it needs: `time`, `ms` and `kind`.

**Columns by name.** PresentMon versions add and reorder columns. Looking up `"MsBetweenPresents"` by name keeps working when a new column appears in front of it; a fixed position would quietly read the wrong numbers.

**Skip, don't crash.** A cut-off last line or a stray text cell should drop that one line, not stop the whole capture. `filter_map` keeps only the `Some`s its closure gives back. Inside the closure, `?` on an `Option` means "if this is `None`, give back `None` for this line right now": a missing cell from `cells.get(..)?`, or a number that will not parse, from `.parse().ok()?`.

**parse knows the type.** `parse()` can turn text into many types; it picks one from where the result goes. Written straight into a field declared `f64`, it parses an `f64`, with no `::<f64>` needed.

**One star for text.** `cells` is a `Vec<&str>`, so `cells.get(i)` gives an `Option<&&str>`: a reference to one of the `&str`s in the list. After `?` it is a `&&str`, and Rust will not compare that with a plain `"Lost"`. One `*` steps through the outer reference to the `&str`, which compares fine. More in [Follow the arrow](/quest/rust/02-ownership/09-follow-the-arrow).

**The same steps, on a ping log.** Each line of a ping log becomes a `Ping` with a parsed number and a yes-or-no from a text cell:

```rust
lines
    .filter_map(|line| {
        let cells: Vec<&str> = line.split(',').collect();
        Some(Ping {
            ms: cells.get(ms)?.parse().ok()?,
            lost: *cells.get(status)? == "Lost",
        })
    })
    .collect()
```

**Generated or not.** `FrameType` is `Application` for frames the game drew and `Generated` for frames that frame generation made up.

## Your turn

- In `parse_capture`, turn each remaining line into a `Frame` with `filter_map`: `time` and `ms` parsed from their cells, and `generated` when the kind cell is `"Generated"`.
- `collect()` the frames and give them back. Save.
