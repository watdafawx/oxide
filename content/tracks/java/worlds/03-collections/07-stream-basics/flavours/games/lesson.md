**Why now:** the updater's job is "which of my mods have a newer version?". That is a stream: keep the outdated ones, take their names, hand back a list. No loop, no temporary list.

A stream is a **pipeline**: a source, steps that describe what to do, and one step that ends it. `Mod` is the record from before (`id`, `name`, `loader`, `downloads`, `version`).

```java
List<String> fabricNames = mods.stream()        // source
        .filter(m -> m.loader().equals("fabric"))   // keep
        .map(Mod::name)                             // change: Mod -> String
        .toList();                                  // end: the list comes out here

int downloads = mods.stream().mapToInt(Mod::downloads).sum();
long forgeCount = mods.stream().filter(m -> m.loader().equals("forge")).count();
List<Mod> firstThree = mods.stream().limit(3).toList();
```

- `filter(predicate)` keeps items where the lambda is true. `map(function)` changes every item.
- **Nothing runs until the last step** (`toList`, `count`, `sum`, ...).
- `mapToInt` gives an `IntStream`, which has `sum()`, `max()`, `average()`.
- A stream never changes `mods`. You get a new result.

## Your turn

- Open `src/main/java/oxide/stream_basics/Outdated.java`.
- `outdatedNames`: names of the mods whose version is not the one `latest` lists for their id.
- `totalDownloads`: the sum of `downloads()` over all mods.
