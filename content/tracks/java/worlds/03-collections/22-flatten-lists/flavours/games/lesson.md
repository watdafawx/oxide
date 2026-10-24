The launcher has several instances, and each one has its own mod list: a `List<List<Mod>>`. The organiser's "check every mod for updates" button wants one list of all of them. That needs one step that the streams so far did not have.

**Lists of lists.** `List<List<Mod>>` is a list whose items are lists, read from the outside in: "a list of lists of Mod". Streaming it gives one item per instance, not per mod.

**map is not enough.** `map(instance -> instance.stream())` turns each instance into a stream, so you get a stream of streams, and `toList()` would hand back a list of streams. Not what the update check wants.

**flatMap.** `.flatMap(fn)` also turns each item into a stream, then glues all those streams end to end into one. An instance of 2 mods and one of 3 give a single stream of 5 mods, in order. An empty instance adds nothing.

```java
List<List<Integer>> rounds = List.of(List.of(3, 5), List.of(), List.of(8));
List<Integer> all = rounds.stream()
        .flatMap(List::stream)
        .toList();                 // [3, 5, 8]
```

**For the curious.** `flatMap` is also the tool for "one item becomes zero, one or many": for example a mod becomes its list of dependencies. Streams of other types have their own version, like `flatMapToInt`.

## Your turn

- In `allMods`, turn the list of instances into one list of every mod, in order, with `flatMap`.
