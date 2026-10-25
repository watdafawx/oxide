The checker has been handing you `List.of` lists for three quests, and the starter kept copying them before any change. Today the organiser gets two lists that should never change: the loaders it supports, and a snapshot of the pack taken before an update.

**A list that says no.** `List.of("fabric", "forge")` builds a list that can never change. `get` and `size` work, but `add`, `remove` or `set` stop the program with an `UnsupportedOperationException`, Java's error for "this list does not do that".

**Why you want that.** A list you hand out can be changed by whoever receives it, and the bug shows up far away, in your code, later. A read-only list turns that silent change into an immediate error at the line that tried it.

**Share or copy.** `return pack;` hands out the very same list: when a mod is installed later, the caller sees it. `List.copyOf(pack)` copies the ids into a new read-only list, a **snapshot**: later changes to `pack` do not reach it. That is what "the pack before the update" needs.

```java
List<String> live = new ArrayList<>(List.of("ash", "oak"));
List<String> frozen = List.copyOf(live);
live.add("elm");
frozen.size();      // still 2
frozen.add("elm");  // UnsupportedOperationException
```

**Same for sets and maps.** `Set.of(...)`, `Map.of(...)`, `Set.copyOf` and `Map.copyOf` work the same way. None of them accept `null` as an item.

**For the curious.** `Collections.unmodifiableList(pack)` is only a read-only *view*: you cannot change it, but it still shows every later change to `pack`. When you mean "a snapshot", `List.copyOf` is the one you want, and it skips the copy when `pack` is already read-only.

## Your turn

- In `supportedLoaders`, return `fabric`, `forge`, `neoforge` and `quilt` as a list that refuses changes.
- In `snapshot`, return a read-only copy of `pack` that stays the same when mods are installed later.
