**Why it matters:** most data code is "take these items, keep some, change some, give me the result". A stream says that as one readable pipeline instead of a loop with a temporary list.

A stream is a **pipeline**: a source, steps that describe what to do, and one step that ends it.

```java
List<String> longUpper = words.stream()        // source
        .filter(w -> w.length() > 3)           // keep
        .map(String::toUpperCase)              // change
        .toList();                             // end: the list comes out here

int letters = words.stream().mapToInt(String::length).sum();
long howMany = words.stream().filter(w -> w.startsWith("a")).count();
List<String> firstTwo = words.stream().limit(2).toList();
```

- `filter(predicate)` keeps items where the lambda is true. `map(function)` changes every item.
- **Nothing runs until the last step** (`toList`, `count`, `sum`, ...). Until then it is only a plan.
- `mapToInt` gives an `IntStream`, which has `sum()`, `max()`, `average()`.
- A stream never changes its source list. You get a new result.
- A stream can be used once.

## Your turn

- Open `src/main/java/oxide/stream_basics/Streams.java`.
- `evenSquares`: keep the even numbers, square them, return the list.
- `totalLength`: the total number of characters in all words.
