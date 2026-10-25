The update checker downloads a list of the newest version of every mod, and the organiser asks it "what is the latest `sodium`?". That is a question about a **key**, the mod id, not about a position in a list. Java's collection for it is the map.

**Key to value.** A `Map` stores pairs: each **key** points to one **value**. `Map<String, String>` has two type parameters, the key type first: mod id to latest version. `HashMap` is the class you usually pick, and it finds a key as fast as a `HashSet` finds a value.

**Put and get.** `put("sodium", "0.5.8")` stores a pair, and putting the same key again replaces its value: one key, one value. `get("sodium")` reads it back.

**Missing is null.** `get("quark")` for a key that is not there hands back `null`, not an error. A `null` version would print as "null" in the report, and a `null` count unboxed into an `int` is the crash from three quests ago.

**A fallback.** `getOrDefault("quark", "unknown")` hands back the value, or `"unknown"` when the key is missing. The same call makes counting easy: read the old count with fallback 0, add one, put it back.

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

- In `latestVersion`, return the newest version for `id`, or `"unknown"` when the list does not have it.
- In `countPerLoader`, build a `Map<String, Integer>` from each loader to how many mods use it.
