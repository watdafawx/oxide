The popular page is sorted, most downloads first; now it needs text, not `Mod` records: every mod's name, and the "top 3" banner. A stream step can turn each item into something else.

**map.** `.map(fn)` hands every item to a Function and passes on whatever comes back. `mods.stream().map(Mod::name)` turns a stream of mods into a stream of names. Same count of items, new type.

**The type follows.** The stream's item type changes with the step: `Stream<Mod>` before `map(Mod::name)`, `Stream<String>` after, so `toList()` gives a `List<String>`. The compiler tracks it for you.

**limit.** `.limit(3)` passes on the first 3 items and stops. A shorter stream just passes everything, so a 2-mod pack gets a banner with 2 names, with no index errors to worry about.

```java
List<String> shouted = words.stream()
        .limit(2)
        .map(String::toUpperCase)
        .toList();      // ["CAT", "DOG"] from ["cat", "dog", "ox"]
```

**Order of steps.** Steps run in the order written. `limit(3).map(...)` names only the first three; `map(...).limit(3)` gives the same names here, but `filter(...).limit(3)` and `limit(3).filter(...)` are different questions.

## Your turn

- In `names`, turn every mod into its name.
- In `topThree`, return the names of the first three mods (the list is already sorted, most downloads first).
