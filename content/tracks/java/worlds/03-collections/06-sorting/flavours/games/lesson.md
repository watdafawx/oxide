**Why now:** the update report has to be readable. Mods grouped by loader, A-Z inside each group, and a "most downloaded first" list for the top picks. Java sorts anything once you say how to compare two items.

`List.sort(comparator)` sorts in place. A `Comparator<T>` decides which of two items comes first. You rarely write one by hand; you build it from a **key**:

```java
mods.sort(Comparator.comparing(Mod::name));                      // A-Z
mods.sort(Comparator.comparingInt(Mod::downloads).reversed());   // most popular first
mods.sort(Comparator.comparing(Mod::loader)                      // loader, then name
        .thenComparing(Mod::name));
```

- `comparing(keyFn)` (or `comparingInt` for int keys) sorts by whatever the key returns.
- `thenComparing(...)` is the tie-break when the first key is equal.
- `reversed()` flips the whole comparator.
- Sorting mutates the list. Sort a copy when the caller still needs the original order.

## Your turn

- Open `src/main/java/oxide/sorting/PackSort.java`.
- `byLoaderThenName`: return a new list, grouped by loader, A-Z by name inside each loader.
- `byDownloadsDescending`: return a new list, most downloaded mod first.
