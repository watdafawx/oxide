The organiser's header shows two numbers: the pack's total downloads, and how many mods run on a given loader. Some streams end in a list; these end in one number.

**No sum on objects.** `mods.stream().map(Mod::downloads)` is a `Stream<Integer>`, and it has no `sum()`. A stream of any type cannot promise its items are numbers, so it cannot offer adding them up.

**mapToInt.** `.mapToInt(Mod::downloads)` is `map` for `int` results. It gives an `IntStream`, a stream of plain `int`s with no boxes, and that one has `sum()`, `max()` and `average()`. An empty stream sums to 0.

**count.** `.count()` ends any stream with how many items reached it. Put a `filter` before it to count only the matches. It hands back a `long`, because a stream may in principle have more items than an `int` can count.

```java
long shortIds = ids.stream().filter(id -> id.length() <= 3).count();
int letters = ids.stream().mapToInt(String::length).sum();
```

**For the curious.** `sum` is a special case of `reduce`, the general "fold everything into one value": `IntStream.of(2, 3, 4).reduce(1, (a, b) -> a * b)` starts at 1 and multiplies each number in, giving 24.

## Your turn

- In `totalDownloads`, add up every mod's downloads with `mapToInt` and `sum`.
- In `countOn`, count the mods whose loader equals `loader` with `filter` and `count`.
