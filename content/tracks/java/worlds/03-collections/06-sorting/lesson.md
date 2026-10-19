**Why it matters:** a leaderboard, a search result, a file list: all of them are "the same items in a useful order". Java sorts anything once you say how to compare two items.

`List.sort(comparator)` sorts in place. A `Comparator<T>` decides which of two items comes first. You rarely write one by hand; you build it from a **key**:

```java
words.sort(Comparator.comparingInt(String::length));            // short first
words.sort(Comparator.comparingInt(String::length).reversed()); // long first
words.sort(Comparator.comparing(String::length)                  // length, then A-Z
        .thenComparing(Comparator.naturalOrder()));
```

- `comparing(keyFn)` (or `comparingInt` for int keys) sorts by whatever the key returns.
- `thenComparing(...)` is the tie-break when the first key is equal.
- `reversed()` flips the whole comparator.
- `naturalOrder()` is the default order: A-Z for strings, small-to-large for numbers.
- Sorting mutates the list. Sort a copy when the caller still needs the original.

## Your turn

- Open `src/main/java/oxide/sorting/Sorting.java`.
- `byLengthThenAlpha`: return a new list of words, shortest first, ties A-Z.
- `byScoreDescending`: return a new list of `Player` records, highest score first.
