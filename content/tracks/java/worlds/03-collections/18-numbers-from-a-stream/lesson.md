A word game's statistics page shows two numbers: how many letters are in today's word list, and how many words are long. Some streams end in a list; these end in one number.

**No sum on objects.** `words.stream().map(String::length)` is a `Stream<Integer>`, and it has no `sum()`. A stream of any type cannot promise its items are numbers, so it cannot offer adding them up.

**mapToInt.** `.mapToInt(String::length)` is `map` for `int` results. It gives an `IntStream`, a stream of plain `int`s with no boxes, and that one has `sum()`, `max()` and `average()`. An empty stream sums to 0.

**count.** `.count()` ends any stream with how many items reached it. Put a `filter` before it to count only the matches. It hands back a `long`, because a stream may in principle have more items than an `int` can count.

```java
long shortOnes = names.stream().filter(n -> n.length() <= 3).count();
int points = players.stream().mapToInt(Player::score).sum();
```

**For the curious.** `sum` is a special case of `reduce`, the general "fold everything into one value": `IntStream.of(2, 3, 4).reduce(1, (a, b) -> a * b)` starts at 1 and multiplies each number in, giving 24.

## Your turn

- In `totalLetters`, add up the lengths of all the words with `mapToInt` and `sum`.
- In `countLonger`, count the words longer than `n` with `filter` and `count`.
