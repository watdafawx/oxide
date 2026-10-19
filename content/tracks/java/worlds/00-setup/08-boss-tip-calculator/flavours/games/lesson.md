**Why now:** this is the finished first feature of the status bot. A server answers with raw text like `12/20`; your bot must turn it into numbers, work out how full the server is, decide how many servers a crowd needs, and post one tidy line. You already have every piece.

The spec, in order:

- `parseCounts("12/20")` turns the raw text into `{12, 20}` (online, max). It must cope with spaces: `" 3 / 20 "`.
- `fillPercent(12, 20)` returns `60`. Whole percent, rounded down. An empty server with `max` 0 is `0`.
- `serversNeeded(45, 20)` returns `3`: how many servers hold 45 players at 20 each. It rounds **up**.
- `report("survival", "12/20")` returns this exact text:

```
survival: 12/20 online (60%)
```

Reminders from this world: `Integer.parseInt`, `String.split`, `String.format(Locale.ROOT, ...)`. Integer division throws away the remainder, so `1 * 100 / 3` is `33`, which is just what you want here.

## Your turn

- Open `src/main/java/oxide/status_report/StatusReport.java`.
- Fill in the four methods. Each objective turns green as you finish its method.
- If one stays red, read `expected` and `but was`.
