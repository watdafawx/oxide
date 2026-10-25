This is what real log tools do: raw text lines in, answers out. What is failing, which part of the system is noisy, and when did it go wrong? You build four small tools, and each one is a few lines made of what this world taught.

**The input.** A log line is `10:15:30 ERROR db Connection refused`: time, level, source, then a message that can contain spaces. `LogLine` is a record with those four parts, and `SourceCount` is a small record that pairs a source with its count. You do not change either.

**1. parse.** Cut the line into at most 4 parts, so the message keeps its spaces, and build a `LogLine` from them. This one is a quick win.

**2. countsPerLevel.** `{INFO=3, ERROR=4, WARN=1}`: one stream, collected with `groupingBy` on the level and `counting()`.

**3. topSources.** Count lines per source the same way. A map cannot be sorted, so turn each entry into a `SourceCount` in a list, sort the list by count, highest first, with ties A to Z by source, and keep the first `n` sources' names.

**4. errorsPerMinute.** Keep only `ERROR` lines and count them per minute. The minute is the first five characters of the time: `time.substring(0, 5)` gives `"10:15"` from `"10:15:30"`. The result must loop in time order, and a `TreeMap` keeps its keys sorted, so copy your counts into one: `new TreeMap<>(counts)`, the same copy shape as `new ArrayList<>(list)`.

```java
// the same shapes on other data: a sorted copy of a count map
Map<Integer, Long> perLength = new TreeMap<>(
        words.stream().collect(groupingBy(String::length, counting())));
```

## Your turn

- Fill in the four methods of `LogAnalyzer`, in the order above. Save after each one and watch the objectives turn green.
