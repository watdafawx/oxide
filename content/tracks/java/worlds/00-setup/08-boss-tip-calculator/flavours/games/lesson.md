**Why now:** this is the finished first feature of the status bot. A server answers with raw text like `12/20`; your bot must turn it into numbers, work out how full the server is, decide how many servers a crowd needs, and post one tidy line. You already have every piece.

The spec, in order:

- `parseCounts("12/20")` turns the raw text into `{12, 20}` (online, max). It must cope with spaces: `" 3 / 20 "`.
- `fillPercent(12, 20)` returns `60`. Whole percent, rounded down. An empty server with `max` 0 is `0`.
- `serversNeeded(45, 20)` returns `3`: how many servers hold 45 players at 20 each. It rounds **up**.
- `report("survival", "12/20")` returns this exact text:

```
survival: 12/20 online (60%)
```

New tools for this boss (printing was the only one you have met):

```java
String[] parts = "12/20".split("/");            // {"12", "20"}: cut text at a separator
int online = Integer.parseInt(parts[0].trim()); // 12: text -> int; trim() drops spaces first
int[] counts = new int[] {12, 20};              // build an int[] by listing its items; `return {a, b};` is NOT valid Java
String line = String.format(Locale.ROOT, "%s: %d players", "survival", counts[0]);  // you know this from printing
int pages = (7 + 3 - 1) / 3;                    // 3: add (size - 1) before dividing to round UP
```

Integer division throws away the remainder, so `1 * 100 / 3` is `33`, which is just what you want for the percent.

## Your turn

- Open `src/main/java/oxide/status_report/StatusReport.java`.
- Fill in the four methods. Each objective turns green as you finish its method.
- If one stays red, read `expected` and `but was`.
