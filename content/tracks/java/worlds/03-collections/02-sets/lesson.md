**Why it matters:** "have I seen this before?" and "remove the repeats" come up constantly (usernames, tags, ids). A `Set` answers both and is fast at it.

A `Set<T>` holds each value at most once. Adding a value that is already there does nothing:

```java
Set<String> tags = new HashSet<>();
tags.add("java");
tags.add("code");
tags.add("java");        // ignored
tags.size();             // 2
tags.contains("code");   // true, and fast even with a million entries
```

- Build a set from a list in one go: `new HashSet<>(list)`. That is the shortest way to dedupe.
- A `HashSet` has **no promised order**. Never rely on the order you get back.
- `LinkedHashSet` keeps insertion order; `TreeSet` keeps sorted order. Same idea, different promise.
- `text.split(" ")` cuts a string into a `String[]`. `Arrays.asList(array)` wraps it as a list.

## Your turn

- Open `src/main/java/oxide/sets/Sets.java`.
- `uniqueWords`: lowercase the text, split on spaces, return the distinct words as a `Set<String>`.
