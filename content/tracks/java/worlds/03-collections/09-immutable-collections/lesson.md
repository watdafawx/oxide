**Why it matters:** a list you hand out can be changed by whoever receives it. Read-only collections turn "who touched my data?" bugs into an immediate exception.

`List.of`, `Set.of` and `Map.of` build collections that **cannot change**. Any `add`, `remove` or `put` throws `UnsupportedOperationException`:

```java
List<String> days = List.of("Mon", "Tue", "Wed");
days.get(0);        // "Mon"
days.add("Thu");    // UnsupportedOperationException

Map<String, Integer> limits = Map.of("small", 10, "big", 100);
```

- `List.copyOf(other)` makes a read-only **copy**. If `other` changes later, the copy does not.
- `Collections.unmodifiableList(other)` is only a read-only *view*: it still follows changes to `other`. Prefer `copyOf`.
- `List.of` and friends refuse `null` elements.
- `Map.of` takes key-value pairs up to 10 pairs; use `Map.ofEntries(...)` or build a `HashMap` for more.
- Returning `List.copyOf(internal)` from a getter protects your class's data.

## Your turn

- Open `src/main/java/oxide/immutable_collections/Immutables.java`.
- `readOnlyDays`: return the days `Mon`, `Tue`, `Wed` as a read-only list.
- `snapshot`: return a read-only copy of `source` that stays the same even if `source` changes later.
