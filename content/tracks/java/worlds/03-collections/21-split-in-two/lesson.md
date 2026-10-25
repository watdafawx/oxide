The exam results page shows two columns: passed and failed. That is a grouping with only two possible keys, yes and no, and Java has a collector for exactly that.

**partitioningBy.** `collect(partitioningBy(rule))` takes a `Predicate` and hands back a `Map<Boolean, List<...>>`: the items the rule said yes to under `true`, the rest under `false`, each in stream order.

**Both keys, always.** `groupingBy(s -> s >= 50)` would do almost the same, but when nobody failed it simply has no `false` key, and `get(false)` is `null`. `partitioningBy` always has both keys, with an empty list where nothing matched, so the page can show an empty column without a `null` check.

```java
Map<Boolean, List<String>> shortOrLong = words.stream()
        .collect(partitioningBy(w -> w.length() <= 4));
shortOrLong.get(true);    // the short words
shortOrLong.get(false);   // the rest, maybe an empty list
```

**For the curious.** Like `groupingBy`, it takes a downstream collector: `partitioningBy(s -> s >= 50, counting())` gives how many passed and failed.

## Your turn

- In `passFail`, split the scores: 50 or more under `true`, the rest under `false`.
