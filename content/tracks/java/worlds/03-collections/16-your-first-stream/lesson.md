Which players passed? With a loop you make an empty list, walk the players, test each one, add it, and return the list: five lines where the idea "keep the ones with 50 or more" hides in the middle. A stream says only the idea.

**The loop way.** `List<Player> out = new ArrayList<>(); for (Player p : players) { if (p.score() >= 50) { out.add(p); } } return out;`. Every line is about *how*: the temporary list, the loop, the `add`. Only the `if` is about *what*.

**A pipeline.** A **stream** is the same work as a pipeline: a source, steps, and an end. `players.stream()` is the source, `.filter(rule)` is a step that keeps the items the rule says yes to, and `.toList()` ends it and hands back a new list. The rule is a lambda, a `Predicate` like the one you wrote for `removeIf`.

**Nothing changes.** A stream never changes its source. `players` is the same list afterwards; you get a new, read-only list from `toList()`.

```java
List<String> shortNames = names.stream()
        .filter(n -> n.length() <= 4)
        .toList();
```

**Only a plan, until the end.** Building the steps does no work. Nothing runs until the last step (`toList()`, and others you will meet), which pulls the items through one at a time. Until then a stream is only a plan, and a stream can be used once.

**For the curious.** Because items flow through one at a time, a step like "stop after the first 3" never even looks at the rest of a million-item list. Streams can also run on several cores with `parallelStream()`, which only pays off for big, slow work.

## Your turn

- In `passing`, keep the players with a score of at least `min`, as one stream pipeline.
- In `longWords`, keep the words longer than `n`, the same way.
