The organiser's loader page shows, per loader, how many mods use it and how many downloads they have together. The `getOrDefault` way from two quests ago works, but it names the key twice and reads like a puzzle. Maps have one call for "add to what is there".

**Read, add, put.** `totals.put(loader, totals.getOrDefault(loader, 0) + downloads)` is three steps in one line: read the old total, add, write it back. It is easy to get the two keys or the fallback wrong.

**merge.** `totals.merge(loader, downloads, rule)` does all three. If the key is missing, it puts `downloads`. If the key is there, it puts `rule(old, downloads)`: the old value and the new one, combined by the rule you hand it.

**The rule is a method.** The rule takes two values and gives back one. To add them, hand it `Integer::sum`, a method reference to `Integer.sum(a, b)`, which returns `a + b`. For counting, merge `1` each time; for totals, merge the downloads.

```java
Map<String, Integer> stock = new HashMap<>();
stock.merge("apple", 3, Integer::sum);   // missing: apple = 3
stock.merge("apple", 2, Integer::sum);   // there: 3 + 2 = 5
stock.merge("pear", 1, Integer::sum);    // missing: pear = 1
```

**For the curious.** Other rules work too: `Integer::max` keeps the biggest value seen per key. And when the rule returns `null`, `merge` removes the key.

## Your turn

- In `modsPerLoader`, count how many mods use each loader: one `merge` of `1` per mod.
- In `downloadsPerLoader`, add up each loader's downloads: one `merge` of the mod's downloads per mod.
