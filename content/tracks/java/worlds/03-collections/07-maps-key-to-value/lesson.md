A shop looks up a price by the item's name, not by its position in a list. "What does `bread` cost?" is a question about a **key**, and Java's collection for it is the map.

**Key to value.** A `Map` stores pairs: each **key** points to one **value**. `Map<String, Integer>` has two type parameters, the key type first: item name to price. `HashMap` is the class you usually pick, and it finds a key as fast as a `HashSet` finds a value.

**Put and get.** `put("bread", 3)` stores a pair, and putting the same key again replaces its value: one key, one value. `get("bread")` reads it back.

**Missing is null.** `get("pear")` for a key that is not there hands back `null`, not an error. Unboxing that `null` into an `int` is the crash from three quests ago, so reading a key that may be missing needs care.

**A fallback.** `getOrDefault("pear", 0)` hands back the value, or `0` when the key is missing. That one call also makes counting easy: read the old count with fallback 0, add one, put it back.

```java
Map<String, Integer> stock = new HashMap<>();
stock.put("apple", 3);
stock.get("apple");                 // 3
stock.get("pear");                  // null
stock.getOrDefault("pear", 0);      // 0
stock.put("apple", stock.getOrDefault("apple", 0) + 1);   // now 4
```

**For the curious.** `containsKey(k)` asks whether a key is there at all, which matters when `null` itself can be a stored value. Map keys are a set: no key appears twice, for the same reason a `HashSet` holds each value once.

## Your turn

- In `priceOf`, return the item's price, or `0` when the shop does not list it.
- In `countWords`, build a `Map<String, Integer>` from each word to how often it appears.
