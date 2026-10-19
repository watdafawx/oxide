**Why now:** a player's tool can hold sharpness, mending, unbreaking, but sharpness twice is a bug. The updater reads enchantment lists from pack files that repeat themselves, so it needs "each id once". That is a `Set`.

A `Set<T>` holds each value at most once. Adding a value that is already there does nothing:

```java
Set<String> enchants = new HashSet<>();
enchants.add("mending");
enchants.add("sharpness");
enchants.add("mending");        // ignored
enchants.size();                // 2
enchants.contains("sharpness"); // true, and fast even with a million entries
```

- Build a set from a list in one go: `new HashSet<>(list)`. That is the shortest way to dedupe.
- A `HashSet` has **no promised order**. Never rely on the order you get back.
- `LinkedHashSet` keeps insertion order; `TreeSet` keeps sorted order.
- `line.split(" ")` cuts a string into a `String[]`. `Arrays.asList(array)` wraps it as a list.

## Your turn

- Open `src/main/java/oxide/sets/Enchantments.java`.
- `uniqueEnchantments`: lowercase the line, split on spaces, return the distinct enchantment ids as a `Set<String>`.
