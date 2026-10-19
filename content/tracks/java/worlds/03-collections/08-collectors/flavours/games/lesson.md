**Why now:** the update report. "Fabric mods here, Forge mods there", "3 Fabric, 2 Forge", "[Sodium, Iris]", "these need updating, those are fine". Each is one `Collectors` call.

`stream.collect(collector)` is the step that ends a pipeline and decides what comes out:

```java
mods.stream().collect(Collectors.groupingBy(Mod::loader));
// {fabric=[Sodium, Iris], forge=[JEI]}                Map<String, List<Mod>>
mods.stream().collect(Collectors.groupingBy(Mod::loader, Collectors.counting()));
// {fabric=2, forge=1}                                 Map<String, Long>
mods.stream().map(Mod::name).collect(Collectors.joining(", ", "[", "]"));
// [Sodium, Iris, JEI]                                 String
mods.stream().collect(Collectors.partitioningBy(m -> m.downloads() > 9000));
// {false=[Iris], true=[Sodium, JEI]}                  Map<Boolean, List<Mod>>
```

- `groupingBy(keyFn)` builds a map from key to list of items with that key.
- A second argument, the **downstream collector**, changes what each group becomes: `counting()` for a count.
- `joining(separator, prefix, suffix)` works on a stream of `String`, so `map(Mod::name)` first.
- `partitioningBy(predicate)` is `groupingBy` with exactly two groups, `true` and `false`, and both keys are always there.

## Your turn

- Open `src/main/java/oxide/collectors/Report.java`.
- `groupByLoader`, `countByLoader`: mods per loader, and how many per loader.
- `report`: `"[Sodium, Iris Shaders]"` (and `"[]"` for no mods).
- `partitionByOutdated`: `true` holds mods not on the version `latest` lists for their id.
