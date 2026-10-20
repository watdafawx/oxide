**Why it matters:** "how many of each?" and "group these by something" are the two most common questions you ask a pile of data. Both are a `Map`.

A `Map<K, V>` stores a value per key. `HashMap` finds a key in near-constant time:

```java
Map<String, Integer> stock = new HashMap<>();
stock.put("apple", 3);
stock.get("apple");                  // 3
stock.get("pear");                   // null, not an error
stock.getOrDefault("pear", 0);       // 0
stock.merge("apple", 1, Integer::sum);   // 3 + 1 = 4; absent key starts at 1
```

- `getOrDefault(key, fallback)` avoids `null` checks when reading.
- `merge(key, value, combine)` is the counting idiom: absent means "put value", present means `combine(old, value)`.
- `computeIfAbsent(key, k -> new ArrayList<>())` returns the value for the key, creating it first if missing. Chain `.add(x)` on it to build "key to list" maps.
- `word.charAt(0)` is the first character of a string, a `char`. As a map key it is boxed to a `Character`, so the key type is `Character`.
- Iterating with `for (Map.Entry<K, V> e : map.entrySet())` gives `e.getKey()` and `e.getValue()`.

## Your turn

- Open `src/main/java/oxide/maps/Maps.java`.
- `countWords`: how often does each word appear?
- `groupByFirstLetter`: map each first letter to the words that start with it (keep input order inside each list).
