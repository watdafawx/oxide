The update screen shows two columns: needs an update, and up to date. That is a grouping with only two possible keys, yes and no, and Java has a collector for exactly that.

**partitioningBy.** `collect(partitioningBy(rule))` takes a `Predicate` and hands back a `Map<Boolean, List<...>>`: the items the rule said yes to under `true`, the rest under `false`, each in stream order.

**Both keys, always.** `groupingBy(rule)` would do almost the same, but when every mod is current it simply has no `true` key, and `get(true)` is `null`. `partitioningBy` always has both keys, with an empty list where nothing matched, so the screen can show an empty column without a `null` check.

```java
Map<Boolean, List<String>> shortOrLong = ids.stream()
        .collect(partitioningBy(id -> id.length() <= 4));
shortOrLong.get(true);    // the short ids
shortOrLong.get(false);   // the rest, maybe an empty list
```

**For the curious.** Like `groupingBy`, it takes a downstream collector: `partitioningBy(rule, counting())` gives how many mods are outdated and how many are current.

## Your turn

- In `byOutdated`, split the mods: those whose version does not equal `latest.get(m.id())` under `true`, the rest under `false`.
