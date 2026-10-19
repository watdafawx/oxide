**Why it matters:** a program that "works" because a `HashMap` happened to print in a nice order will break the day the data changes. Pick the map whose *promise* matches what you need.

Same entries, three maps:

```java
// put order: pear, apple, fig
new HashMap<>()        // pear/apple/fig in ANY order: no promise
new LinkedHashMap<>()  // pear, apple, fig        (insertion order)
new TreeMap<>()        // apple, fig, pear        (sorted by key)
```

- **HashMap / HashSet**: fastest, no order. Use it by default.
- **LinkedHashMap / LinkedHashSet**: remembers insertion order. Use it when output order matters (reports, menus).
- **TreeMap / TreeSet**: always sorted by key (keys must be `Comparable`, or you pass a `Comparator`). Slightly slower, gives `firstKey()`, `headMap(...)`.

**Hashing in one paragraph.** A `HashMap` turns a key into a number with `hashCode()`, jumps to that bucket, then uses `equals()` to find the exact key. So two keys that are `equals` **must** have the same `hashCode`. Break that and `map.get(key)` can miss an entry that is "obviously" there. Strings, numbers and records already do this right. You will write your own `equals`/`hashCode` in the OOP world.

## Quiz

Three questions. The list above has all the answers.
