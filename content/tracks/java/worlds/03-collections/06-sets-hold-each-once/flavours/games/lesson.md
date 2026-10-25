Three config files each add enchantments to the pack, and together they read `sharpness, mending, sharpness, unbreaking, mending`. A list keeps every repeat, because order and count are its job. For "which enchantments, once each", Java has another collection.

**One of each.** A `Set` holds each value at most once. Adding a value that is already there changes nothing. `Set` is an interface, like `List`, and `HashSet` is the class you usually pick.

**Add answers back.** `add` hands back a `boolean`: `true` when the value was new, `false` when the set already had it. That answers "have I seen this before?" in the same step as remembering it, which is how the organiser can warn about a config that registers an id twice.

**Fast to ask.** `contains("mending")` on a `HashSet` is about as fast with a million ids as with ten, because the set jumps straight to where that value would be instead of checking every item. A `List` has to walk through all of them. How the jump works is a short read three quests from now.

**From a list in one go.** `new HashSet<>(ids)` builds a set from any list and drops the repeats on the way, the same copy shape as `new ArrayList<>(other)`.

```java
Set<String> biomes = new HashSet<>();
biomes.add("desert");       // true: new
biomes.add("taiga");        // true
biomes.add("desert");       // false: already there
biomes.size();              // 2
biomes.contains("taiga");   // true
```

**No order.** A `HashSet` promises nothing about the order it hands values back in. Never rely on it; the checker compares sets without looking at order.

## Your turn

- In `distinct`, return every enchantment id once, as a `Set<String>`.
- In `firstRepeat`, walk the ids in order, remember each one in a set, and return the first id that `add` says was already there. Return `null` when no id repeats.
