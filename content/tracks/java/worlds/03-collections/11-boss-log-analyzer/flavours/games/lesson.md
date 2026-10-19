**Why now:** this is the last piece of the updater. It reads the pack file, and after one pass tells the player which loader has the most mods, which mods people love, and where the updates are waiting. Everything you need is already in this world: `map`, `filter`, `groupingBy`, `counting`, sorting and `limit`.

A pack file has one mod per line:

```
sodium,Sodium,fabric,9500,0.5.3
```

id, name, loader, downloads, version. You build four small tools, each a stream pipeline.

1. **`parseMod`**: one CSV line into a `Mod` record.
2. **`countsPerLoader`**: `{fabric=3, forge=2, neoforge=1}`.
3. **`topDownloaded`**: the names of the n most downloaded mods. Sort by downloads descending, ties by name, `limit(n)`.
4. **`outdatedPerLoader`**: only outdated mods (their version differs from `latest`), counted per loader, loaders in A-Z order (`TreeMap`).

Work top to bottom: the first objective is a quick win, and each next one reuses the same ideas.

```java
// sorting with a tie-break
mods.stream().sorted(Comparator.comparingInt(Mod::downloads).reversed().thenComparing(Mod::name));

// grouping into a sorted map
mods.stream().collect(groupingBy(Mod::loader, TreeMap::new, counting()));
```

## Your turn

- Open `src/main/java/oxide/update_report/UpdateReport.java`. `Mod` is a record you do not need to change.
- Fill in the four methods, in the order above. Save after each one and watch the objectives turn green.
