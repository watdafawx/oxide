**Why it matters:** real data is often nested (a list of orders, each with a list of items) and often needs boiling down to one value (a total, the biggest). `flatMap` and `reduce` are those two moves.

```java
List<List<Integer>> nested = List.of(List.of(1, 2), List.of(3), List.of());

nested.stream().flatMap(List::stream).toList();    // [1, 2, 3]   one stream per inner list, glued

IntStream.rangeClosed(1, 4).reduce(1, (a, b) -> a * b);   // 24   1*1*2*3*4

List.of(3, 9, 4).stream().max(Comparator.naturalOrder());   // Optional[9]
new ArrayList<Integer>().stream().max(Comparator.naturalOrder());   // Optional.empty
```

- `map` turns one item into one item. `flatMap` turns one item into a **stream** of items and flattens them all into one stream.
- `IntStream.rangeClosed(1, n)` is the numbers 1 to n, **inclusive**. `range(1, n)` stops before n.
- `reduce(identity, combine)` folds a stream into one value: it starts at `identity`, then `combine(sofar, next)` for each item.
- `max` can't answer for an empty stream, so it returns an `Optional`. `.isPresent()`, `.isEmpty()`, `.get()` (only after checking), `.orElse(fallback)`.

## Your turn

- Open `src/main/java/oxide/flatmap_reduce/Flat.java`.
- `flatten`: one list out of a list of lists.
- `product`: `n!` using `rangeClosed` and `reduce`.
- `largest`: the biggest number as an `Optional<Integer>`.
