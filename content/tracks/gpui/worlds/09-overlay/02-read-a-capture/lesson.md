`fixtures/capture.csv` is a recorded game session: 1200 frames, shaped like a cut-down PresentMon capture. Testing FPS maths on a recording means the numbers are known; a live game is different every second.

**A header, then rows.** The first line names the columns: `Application,ProcessID,TimeInSeconds,MsBetweenPresents,FrameType`. Every other line is one frame. The starter reads the header and finds where each column is.

**Columns by name.** PresentMon versions add and reorder columns. Looking up `"MsBetweenPresents"` in the header keeps working when a new column appears before it.

**Each line a Frame.** Split on commas, take the three cells, parse two numbers. A line that does not parse is skipped, not a crash: `filter_map` keeps only the `Some`s, and `?` inside it gives `None` for any missing cell or bad number:

```rust
time: cells.get(time)?.parse().ok()?,
```

**Generated or not.** `FrameType` is `Application` for frames the game drew and `Generated` for frames frame generation made up.

## Your turn

- In `parse_capture`, map each remaining line to a `Frame` with `filter_map`: the three cells, two parsed numbers, and `generated` when the type is `"Generated"`.
- `collect()` the frames. Save.
