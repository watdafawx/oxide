Ada and Grace both scored 82. Sorted by score alone, which comes first is down to luck of the input order, and the leaderboard flickers between runs. A second key settles it: equal scores go A to Z by name.

**A tie-break.** `first.thenComparing(second)` builds a comparator that uses `first`, and only when `first` calls two items equal does it ask `second`. You can chain as many as you need.

**Where reversed goes.** `.reversed()` flips the whole comparator built so far. `comparingInt(Player::score).thenComparing(Player::name).reversed()` flips both keys: high scores first, but names Z to A. Put `reversed()` right after the key it belongs to, then add the tie-break.

```java
List<String> words = new ArrayList<>(List.of("pear", "fig", "kiwi", "date"));
words.sort(Comparator.comparingInt(String::length)
        .thenComparing(Comparator.naturalOrder()));    // fig, date, kiwi, pear
```

**Natural order as a key.** In the example the words are the key themselves, so the tie-break is `Comparator.naturalOrder()`: A to Z for text.

**For the curious.** `list.sort` is **stable**: items that compare equal keep their input order. That is why a missing tie-break gives "luck of the input order", not random shuffling.

## Your turn

- In `byScoreThenName`, return a sorted copy: highest score first, and players with equal scores A to Z by name.
