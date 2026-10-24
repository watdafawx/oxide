The quiz night is over, and the scores sit in a map from team name to points. To announce the winner you have to look at every pair, not look up one key you already know.

**A pair at a time.** `scores.entrySet()` hands you every pair in the map as a set of **entries**. Each entry is one key with its value. The enhanced `for` walks it like any other collection.

**The entry's type.** One entry of a `Map<String, Integer>` is a `Map.Entry<String, Integer>`: the same two type parameters, key first. `e.getKey()` is the name and `e.getValue()` is the score.

**Only one side.** When you need just the keys, `keySet()` is a `Set` of them; `values()` gives just the values. Reach for `entrySet()` when you need both, so you do not look every key up a second time.

```java
Map<String, Integer> stock = Map.of("apple", 3, "pear", 0, "fig", 7);
int total = 0;
for (Map.Entry<String, Integer> e : stock.entrySet()) {
    System.out.println(e.getKey() + ": " + e.getValue());
    total += e.getValue();
}
```

**The champion again.** Finding the top scorer is the champion pattern from the Basics world, with two variables this time: the best name so far and its score. Start the score at `Integer.MIN_VALUE` so the first entry always wins.

**Any order.** A `HashMap` walks its entries in no promised order, so the checker does not care in which order `passing` lists the names.

## Your turn

- In `topScorer`, return the name with the highest score. There is never a tie.
- In `passing`, return a list of every name whose score is at least `min`, in any order.
