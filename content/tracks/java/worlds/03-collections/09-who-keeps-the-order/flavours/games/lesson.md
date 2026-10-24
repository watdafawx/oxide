The organiser is about to print reports. A report that lists the pack in a different order on every machine is useless, and a `HashMap` makes no promise about order. Java has three maps with three different promises, and the same three for sets.

**HashMap: no promise.** The fastest, and the one to use by default. It hands keys back in an order that depends on their hash numbers, which can change when you add one more mod or run on a newer Java.

**LinkedHashMap: as you put them.** It also remembers the order you put the keys in, and loops in exactly that order. Use it when the report should follow the pack file.

**TreeMap: sorted.** It keeps the keys sorted: A to Z for mod ids, small to large for numbers. Lookups are a little slower, and you get extras like `firstKey()`. Its keys must have a natural order, or you hand it a rule for comparing them. The sets `HashSet`, `LinkedHashSet` and `TreeSet` make the same three promises: put `sodium`, `iris`, `jei` in, and they loop as any order, then `sodium, iris, jei`, then `iris, jei, sodium`.

**How a hash map finds a key.** It turns the key into a number with `hashCode()`, jumps straight to the bucket for that number, and then uses `equals()` to find the exact key among the few in that bucket. That jump is why `get` and `contains` do not slow down as the pack grows to 300 mods.

**The rule that follows.** So two keys that are `equals` **must** have the same `hashCode`. Break that and the map jumps to the wrong bucket, and `get` misses a mod that is "obviously" installed. Strings, the number boxes and records already keep this rule; a class of your own used as a key must define `equals` and `hashCode` together.

**For the curious.** A `HashMap` grows its bucket array when it is about three quarters full, and since Java 8 a bucket that collects too many keys turns into a small sorted tree, so even bad hash codes stay reasonably fast.

## Your turn

- Answer the three questions below.
