A program that "works" because a `HashMap` happened to print in a nice order breaks the day the data changes. Java has three maps with three different promises about order, and the same three for sets. Pick the one whose promise matches what you need.

**HashMap: no promise.** The fastest, and the one to use by default. It hands keys back in an order that depends on their hash numbers, which can change when you add one more key or run on a newer Java.

**LinkedHashMap: as you put them.** It also remembers the order you put the keys in, and loops in exactly that order. Use it when output order matters, like a menu or a report.

**TreeMap: sorted.** It keeps the keys sorted: A to Z for text, small to large for numbers. Lookups are a little slower, and you get extras like `firstKey()`. Its keys must have a natural order, or you hand it a rule for comparing them. The sets `HashSet`, `LinkedHashSet` and `TreeSet` make the same three promises: put `pear`, `apple`, `fig` in, and they loop as any order, then `pear, apple, fig`, then `apple, fig, pear`.

**How a hash map finds a key.** It turns the key into a number with `hashCode()`, jumps straight to the bucket for that number, and then uses `equals()` to find the exact key among the few in that bucket. That jump is why `get` and `contains` do not slow down as the map grows.

**The rule that follows.** So two keys that are `equals` **must** have the same `hashCode`. Break that and the map jumps to the wrong bucket, and `get` misses an entry that is "obviously" there. Strings, the number boxes and records already keep this rule; a class of your own used as a key must define `equals` and `hashCode` together.

**For the curious.** A `HashMap` grows its bucket array when it is about three quarters full, and since Java 8 a bucket that collects too many keys turns into a small sorted tree, so even bad hash codes stay reasonably fast.

## Your turn

- Answer the three questions below.
