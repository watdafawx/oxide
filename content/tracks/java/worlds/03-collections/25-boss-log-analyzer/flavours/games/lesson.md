This is the last piece of the modpack organiser. It reads the raw pack file and, in one pass, tells the player which loader has the most mods, which mods people love, and where the updates are waiting. You build four small tools, and each one is a few lines made of what this world taught.

**The input.** A pack file has one mod per line: `sodium,Sodium,fabric,9500,0.5.3`, which is id, name, loader, downloads and version. `Mod` is the record you have used all world. You do not change it.

**1. parseMod.** Cut the line at its commas and build a `Mod`. Downloads arrive as text, `"9500"`, and `Integer.parseInt(parts[3])` turns that into the number. This one is a quick win.

**2. countsPerLoader.** `{fabric=3, forge=2, neoforge=1}`: one stream, collected with `groupingBy` on the loader and `counting()`.

**3. topDownloaded.** The names of the `n` most downloaded mods. Sort a copy of the list by downloads, most first, with ties A to Z by name, then keep the first `n` names.

**4. outdatedPerLoader.** Keep only outdated mods (their version differs from `latest.get(m.id())`) and count them per loader. The report must list loaders A to Z, and a `TreeMap` keeps its keys sorted, so copy your counts into one: `new TreeMap<>(counts)`, the same copy shape as `new ArrayList<>(list)`.

```java
// the same shapes on other data: a sorted copy of a count map
Map<Integer, Long> perLength = new TreeMap<>(
        ids.stream().collect(groupingBy(String::length, counting())));
```

## Your turn

- Fill in the four methods of `UpdateReport`, in the order above. Save after each one and watch the objectives turn green.
