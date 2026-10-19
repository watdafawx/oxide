**Why it matters:** "group these", "how many of each", "make one string out of them" are the questions a report answers. `Collectors` do each in one line, where a loop would take eight.

`stream.collect(collector)` is the step that ends a pipeline and decides what comes out:

```java
words.stream().collect(Collectors.groupingBy(String::length));
// {3=[cat, dog], 5=[horse]}                      Map<Integer, List<String>>
words.stream().collect(Collectors.groupingBy(String::length, Collectors.counting()));
// {3=2, 5=1}                                     Map<Integer, Long>
words.stream().collect(Collectors.joining(", ", "[", "]"));
// [cat, dog, horse]                              String
words.stream().collect(Collectors.partitioningBy(w -> w.length() > 3));
// {false=[cat, dog], true=[horse]}               Map<Boolean, List<String>>
```

- `groupingBy(keyFn)` builds a map from key to list of items with that key.
- A second argument, the **downstream collector**, changes what each group becomes: `counting()` for a count.
- `joining(separator, prefix, suffix)` works on a stream of `String`. Prefix and suffix are optional.
- `partitioningBy(predicate)` is `groupingBy` with exactly two groups, `true` and `false`, and both keys are always there.
- `import static java.util.stream.Collectors.*;` lets you write `groupingBy(...)` directly.

## Your turn

- Open `src/main/java/oxide/collectors/Collecting.java`.
- `groupByLength`, `countByLength`: buckets of words per length, and the size of each bucket.
- `joinWithCommas`: `"[a, b, c]"` (and `"[]"` for no words).
- `partitionPassFail`: scores of 50 or more under `true`, the rest under `false`.
