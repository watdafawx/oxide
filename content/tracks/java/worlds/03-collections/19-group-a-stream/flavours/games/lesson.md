Six quests ago you grouped mod names by loader with `computeIfAbsent` and a loop. Streams have the same grouping as one step, and one more word turns each group into a count: the "how many of each" question every report asks.

**collect.** `.collect(collector)` ends a stream, and the **collector** decides what comes out. `toList()` was a shortcut for the simplest one. `Collectors` holds the rest.

**groupingBy.** `collect(groupingBy(Mod::loader))` gives a `Map<String, List<Mod>>`: each key the Function returned, with the list of items that gave it, in stream order. That is your `computeIfAbsent` loop in one line.

**Count instead of list.** A second argument, a **downstream** collector, decides what each group becomes. `groupingBy(Mod::loader, counting())` makes each group a count instead of a list: a `Map<String, Long>`, since `count` is a `long`.

```java
Map<Boolean, List<Integer>> evenOdd = nums.stream()
        .collect(groupingBy(n -> n % 2 == 0));
Map<Integer, Long> perLength = ids.stream()
        .collect(groupingBy(String::length, counting()));
```

**The short names.** `groupingBy` and `counting` really live in `Collectors`, so their full names are `Collectors.groupingBy(...)`. The starter has `import static java.util.stream.Collectors.groupingBy;` (and the same for `counting`): a **static import**, which lets you write the short name.

## Your turn

- In `groupByLoader`, collect the mods into a map from loader to the mods that use it.
- In `countByLoader`, collect them into a map from loader to how many mods use it.
