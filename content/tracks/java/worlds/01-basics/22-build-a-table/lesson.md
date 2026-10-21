A report lists names and scores, lined up in neat columns. That is many small pieces of text glued together, and one more format trick.

**Many small appends.** A `StringBuilder` collects text piece by piece. `sb.append(...)` adds to the end, and `sb.toString()` hands back everything as a normal text. It is better than `+` in a loop, which builds a new text every round.

**Column widths.** In a slot, a number sets the width. `%-6s` pads text to the right so it fills the column, and `%4d` pads a number on the left so the digits line up. The `-` means left-aligned.

**Two rows.** Two appends, then one `toString()`:

```java
StringBuilder sb = new StringBuilder();
sb.append(String.format("%-6s|%4d\n", "Ada", 95));
sb.append(String.format("%-6s|%4d\n", "Bo", 7));
String table = sb.toString();
```

**Two arrays side by side.** `names` and `scores` line up: `names[0]` goes with `scores[0]`. Loop with a counter `i` and use it for both: `for (int i = 0; i < names.length; i++)`.

**End every row.** Finish each row with `\n` so the next one starts on a new line. Inside a `String.format` template you write it as `\n`.

## Your turn

- In `table`, make a `StringBuilder` and loop over the positions of `names`.
- For each position, append one row: the name, a `|`, the score, then `\n`. Use the column widths from the lesson.
- Return the finished text with `toString()`. Save and the objective turns green.
