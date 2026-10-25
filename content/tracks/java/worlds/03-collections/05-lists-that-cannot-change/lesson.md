The checker has been handing you `List.of` lists for three quests, and the starter kept copying them before any change. Today you find out why, and when a list that refuses changes is exactly what you want.

**A list that says no.** `List.of("Mon", "Tue")` builds a list that can never change. `get` and `size` work, but `add`, `remove` or `set` stop the program with an `UnsupportedOperationException`, Java's error for "this list does not do that".

**Why you want that.** A list you hand out can be changed by whoever receives it, and the bug shows up far away, in your code, later. A read-only list turns that silent change into an immediate error at the line that tried it.

**Share or copy.** `return source;` hands out the very same list: when the owner changes it later, the caller sees the change. `List.copyOf(source)` copies the items into a new read-only list, a **snapshot**: later changes to `source` do not reach it.

```java
List<String> live = new ArrayList<>(List.of("ash", "oak"));
List<String> frozen = List.copyOf(live);
live.add("elm");
frozen.size();      // still 2
frozen.add("elm");  // UnsupportedOperationException
```

**Same for sets and maps.** `Set.of(...)`, `Map.of(...)`, `Set.copyOf` and `Map.copyOf` work the same way. None of them accept `null` as an item.

**For the curious.** `Collections.unmodifiableList(source)` is only a read-only *view*: you cannot change it, but it still shows every later change to `source`. When you mean "a snapshot", `List.copyOf` is the one you want, and it skips the copy when `source` is already read-only.

## Your turn

- In `openingDays`, return `Mon`, `Tue` and `Wed` as a list that refuses changes.
- In `snapshot`, return a read-only copy of `source` that stays the same when `source` changes later.
