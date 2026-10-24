The loader page should list, under each loader, the names of the mods that use it. That is a map from a loader to a list of names: `Map<String, List<String>>`. The trouble is the first mod of each loader, when its list does not exist yet.

**The clumsy way.** For every mod: `get` the list for its loader; if that is `null`, make an empty list and `put` it; then add the name. Four lines and an `if`, every time you group.

**computeIfAbsent.** `groups.computeIfAbsent("fabric", rule)` hands back the value for key `"fabric"`. If there is none yet, it first calls `rule` to make one, stores it, and hands back that. So the list you get is always real, and you can `add` to it straight away.

**The rule makes a list.** The rule is a lambda that takes the key and makes the new value: `k -> new ArrayList<>()`. It gets the key in case the new value depends on it; here it does not, so `k` goes unused.

```java
Map<String, List<String>> byShelf = new HashMap<>();
byShelf.computeIfAbsent("A", k -> new ArrayList<>()).add("Dune");
byShelf.computeIfAbsent("A", k -> new ArrayList<>()).add("Emma");   // same list
byShelf.computeIfAbsent("B", k -> new ArrayList<>()).add("Ulysses");
// {A=[Dune, Emma], B=[Ulysses]}
```

**Lists inside a map.** `Map<String, List<String>>` nests one generic type in another: the values are lists of names. Read it from the outside in, "a map from String to list of String".

## Your turn

- In `namesByLoader`, for each mod, get the list for its loader with `computeIfAbsent` (making an empty one when missing) and add the mod's name to it. Names keep their pack order inside each list.
