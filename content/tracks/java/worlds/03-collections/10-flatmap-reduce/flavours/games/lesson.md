**Why now:** a player runs several game instances, each with its own mod folder. The updater needs "every mod across all instances", "total downloads", and "the most popular one", where the last one might not exist if there are no mods at all.

```java
List<List<Mod>> instances = List.of(survival, creative, List.of());

instances.stream().flatMap(List::stream).toList();    // all mods in one list

mods.stream().map(Mod::downloads).reduce(0, Integer::sum);   // 0 + 9500 + 8200 ...

mods.stream().max(Comparator.comparingInt(Mod::downloads));   // Optional[Mod[...]] or Optional.empty
```

- `map` turns one item into one item. `flatMap` turns one item into a **stream** of items and flattens them all into one stream.
- `reduce(identity, combine)` folds a stream into one value: it starts at `identity`, then `combine(sofar, next)` for each item.
- `max` can't answer for an empty stream, so it returns an `Optional`: a box that holds the answer, or is empty. Return it as it is.

## Your turn

- Open `src/main/java/oxide/flatmap_reduce/Instances.java`.
- `allMods`: one list out of a list of per-instance mod lists.
- `totalDownloads`: the sum of all downloads, using `reduce`.
- `mostDownloaded`: the top mod as an `Optional<Mod>`.
