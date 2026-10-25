Which mods need an update? With a loop you make an empty list, walk the pack, test each mod, add it, and return the list: five lines where the idea "keep the outdated ones" hides in the middle. A stream says only the idea, and the rest of the organiser's reports are built from streams.

**The loop way.** `List<Mod> out = new ArrayList<>(); for (Mod m : mods) { if (rule.test(m)) { out.add(m); } } return out;`. Every line is about *how*: the temporary list, the loop, the `add`. Only the `if` is about *what*.

**A pipeline.** A **stream** is the same work as a pipeline: a source, steps, and an end. `mods.stream()` is the source, `.filter(rule)` is a step that keeps the items the rule says yes to, and `.toList()` ends it and hands back a new list. The rule is a lambda, a `Predicate` like the one you wrote for `removeIf`.

**Nothing changes.** A stream never changes its source. The pack is the same list afterwards; you get a new, read-only list from `toList()`.

```java
List<String> shortIds = ids.stream()
        .filter(id -> id.length() <= 4)
        .toList();
```

**Only a plan, until the end.** Building the steps does no work. Nothing runs until the last step (`toList()`, and others you will meet), which pulls the items through one at a time. Until then a stream is only a plan, and a stream can be used once.

**For the curious.** Because items flow through one at a time, a step like "stop after the first 3" never even looks at the rest of a 300-mod pack. Streams can also run on several cores with `parallelStream()`, which only pays off for big, slow work.

## Your turn

- In `outdated`, keep the mods whose version does not equal `latest.get(m.id())`, as one stream pipeline.
- In `onLoader`, keep the mods whose loader equals `loader`, the same way.
