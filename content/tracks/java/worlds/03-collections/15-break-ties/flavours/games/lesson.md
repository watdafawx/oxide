The pack page groups mods by loader: all fabric mods, then forge, then neoforge. Sorted by loader alone, the mods inside each loader come out in whatever order the pack file had, and the page reshuffles after every install. A second key settles it: inside a loader, A to Z by name.

**A tie-break.** `first.thenComparing(second)` builds a comparator that uses `first`, and only when `first` calls two items equal does it ask `second`. You can chain as many as you need.

**Where reversed goes.** `.reversed()` flips the whole comparator built so far. `comparingInt(Mod::downloads).thenComparing(Mod::name).reversed()` flips both keys: most downloads first, but names Z to A. Put `reversed()` right after the key it belongs to, then add the tie-break.

```java
List<String> words = new ArrayList<>(List.of("pear", "fig", "kiwi", "date"));
words.sort(Comparator.comparingInt(String::length)
        .thenComparing(Comparator.naturalOrder()));    // fig, date, kiwi, pear
```

**Natural order as a key.** In the example the words are the key themselves, so the tie-break is `Comparator.naturalOrder()`: A to Z for text.

**For the curious.** `list.sort` is **stable**: items that compare equal keep their input order. That is why a missing tie-break gives "whatever order the pack file had", not random shuffling.

## Your turn

- In `byLoaderThenName`, return a sorted copy: loaders A to Z, and mods of the same loader A to Z by name.
