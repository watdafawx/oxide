**Why it matters:** this is what real log tools do. Raw text lines in, answers out: what is failing, which part is noisy, when did it go wrong. Everything you need is already in this world: `map`, `filter`, `groupingBy`, `counting`, sorting and `limit`.

A log line looks like this:

```
10:15:30 ERROR db Connection refused
```

time, level, source, then the message (which can contain spaces). `line.split(" ", 4)` cuts into **at most 4** parts, so the 4th part keeps its spaces: that is how the message survives. You build four small tools, each a stream pipeline.

1. **`parse`**: one line of text into a `LogLine` record.
2. **`countsPerLevel`**: `{INFO=3, ERROR=4, WARN=1}`.
3. **`topSources`**: the n sources with the most lines. Count with `groupingBy`, then sort the map's entries by count descending, ties by name.
4. **`errorsPerMinute`**: only `ERROR` lines, bucketed by the `HH:mm` part of the time, in time order (`TreeMap`).

Work top to bottom: the first objective is a quick win, and each next one reuses the same ideas.

```java
// sorting map entries: highest count first
Comparator<Map.Entry<String, Long>> byCount = Map.Entry.comparingByValue();
counts.entrySet().stream().sorted(byCount.reversed().thenComparing(Map.Entry::getKey));

// grouping into a sorted map
lines.stream().collect(groupingBy(l -> l.time().substring(0, 5), TreeMap::new, counting()));
```

## Your turn

- Open `src/main/java/oxide/log_analyzer/LogAnalyzer.java`. `LogLine` is a record you do not need to change.
- Fill in the four methods, in the order above. Save after each one and watch the objectives turn green.
