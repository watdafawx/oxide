**Why now:** the updater runs in steps. Once it has decided what to update, nobody should be able to change the list underneath it, and the list of supported loaders should never change at all. Read-only collections make both a hard guarantee.

`List.of`, `Set.of` and `Map.of` build collections that **cannot change**. Any `add`, `remove` or `put` throws `UnsupportedOperationException`:

```java
List<String> loaders = List.of("fabric", "forge");
loaders.get(0);          // "fabric"
loaders.add("quilt");    // UnsupportedOperationException

Map<String, String> latest = Map.of("sodium", "0.5.8", "iris", "1.7.0");
```

- `List.copyOf(live)` makes a read-only **snapshot**. If `live` changes later, the snapshot does not.
- `Collections.unmodifiableList(live)` is only a read-only *view*: it still follows changes. Prefer `copyOf`.
- `List.of` and friends refuse `null` elements.
- `Map.of` takes key-value pairs up to 10 pairs; use `Map.ofEntries(...)` or build a `HashMap` for more.

## Your turn

- Open `src/main/java/oxide/immutable_collections/Locking.java`.
- `supportedLoaders`: return `fabric`, `forge`, `neoforge`, `quilt` as a read-only list.
- `snapshot`: return a read-only copy of the live mod list that stays the same even if the live list changes later.
