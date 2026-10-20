**Why now:** the updater logs every download by mod id, then needs "how many downloads per id" and a mod index grouped by first letter for the report. Both are a `Map`.

A `Map<K, V>` stores a value per key. `HashMap` finds a key in near-constant time:

```java
Map<String, Integer> downloads = new HashMap<>();
downloads.put("sodium", 3);
downloads.get("sodium");                  // 3
downloads.get("iris");                    // null, not an error
downloads.getOrDefault("iris", 0);        // 0
downloads.merge("sodium", 1, Integer::sum);   // 3 + 1 = 4; absent id starts at 1
```

- `getOrDefault(id, fallback)` avoids `null` checks when reading.
- `merge(id, 1, Integer::sum)` is the counting idiom: absent means "put 1", present means `old + 1`.
- `computeIfAbsent(key, k -> new ArrayList<>())` returns the value for the key, creating it first if missing. Chain `.add(x)` to build "key to list" maps.
- `name.charAt(0)` is the first character of a string, a `char`. As a map key it is boxed to a `Character`, so the key type is `Character`.
- Iterating with `for (Map.Entry<K, V> e : map.entrySet())` gives `e.getKey()` and `e.getValue()`.

## Your turn

- Open `src/main/java/oxide/maps/Downloads.java`.
- `countDownloads`: given a log of mod ids (one per download), how many downloads did each id get?
- `indexByLetter`: map each first letter to the mod names that start with it (keep input order inside each list).
