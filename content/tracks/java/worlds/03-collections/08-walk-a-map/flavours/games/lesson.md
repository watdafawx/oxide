The organiser now keeps a map from mod id to download count. "Which mod is the most popular?" means looking at every pair, not looking up one id you already know.

**A pair at a time.** `downloads.entrySet()` hands you every pair in the map as a set of **entries**. Each entry is one key with its value. The enhanced `for` walks it like any other collection.

**The entry's type.** One entry of a `Map<String, Integer>` is a `Map.Entry<String, Integer>`: the same two type parameters, key first. `e.getKey()` is the mod id and `e.getValue()` is its download count.

**Only one side.** When you need just the keys, `keySet()` is a `Set` of them; `values()` gives just the values. Reach for `entrySet()` when you need both, so you do not look every key up a second time.

```java
Map<String, Integer> stock = Map.of("apple", 3, "pear", 0, "fig", 7);
int total = 0;
for (Map.Entry<String, Integer> e : stock.entrySet()) {
    System.out.println(e.getKey() + ": " + e.getValue());
    total += e.getValue();
}
```

**The champion again.** Finding the most downloaded mod is the champion pattern from the Basics world, with two variables this time: the best id so far and its count. Start the count at `Integer.MIN_VALUE` so the first entry always wins.

**Any order.** A `HashMap` walks its entries in no promised order, so the checker does not care in which order `popular` lists the ids.

## Your turn

- In `mostDownloaded`, return the mod id with the most downloads. There is never a tie.
- In `popular`, return a list of every mod id with at least `min` downloads, in any order.
