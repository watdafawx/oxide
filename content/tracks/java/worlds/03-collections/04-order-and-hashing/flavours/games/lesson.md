**Why now:** the updater is about to print reports. A report that lists mods in a different order on every machine is useless, so you need to know which map gives you which order.

Same entries, three maps:

```java
// installed: sodium, iris, lithium
new HashMap<>()        // sodium/iris/lithium in ANY order: no promise
new LinkedHashMap<>()  // sodium, iris, lithium    (insertion order)
new TreeMap<>()        // iris, lithium, sodium    (sorted by id)
```

- **HashMap / HashSet**: fastest, no order. Use it by default (lookups by mod id).
- **LinkedHashMap / LinkedHashSet**: remembers insertion order. Use it for anything printed in pack-file order.
- **TreeMap / TreeSet**: always sorted by key, so the report is alphabetical without extra work.

**Hashing in one paragraph.** A `HashMap` turns a mod id into a number with `hashCode()`, jumps to that bucket, then uses `equals()` to find the exact key. So two keys that are `equals` **must** have the same `hashCode`. Break that and `downloads.get(id)` can miss an entry that is "obviously" there. Strings, numbers and records already do this right.

## Quiz

Three questions. The list above has all the answers.
