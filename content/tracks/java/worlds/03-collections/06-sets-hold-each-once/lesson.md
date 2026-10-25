A blog post was tagged by three people, and the tag list reads `java, code, java, tips, code`. A list keeps every repeat, because order and count are its job. For "which tags, once each", Java has another collection.

**One of each.** A `Set` holds each value at most once. Adding a value that is already there changes nothing. `Set` is an interface, like `List`, and `HashSet` is the class you usually pick.

**Add answers back.** `add` hands back a `boolean`: `true` when the value was new, `false` when the set already had it. That answers "have I seen this before?" in the same step as remembering it.

**Fast to ask.** `contains("tips")` on a `HashSet` is about as fast with a million tags as with ten, because the set jumps straight to where that value would be instead of checking every item. A `List` has to walk through all of them. How the jump works is a short read three quests from now.

**From a list in one go.** `new HashSet<>(tags)` builds a set from any list and drops the repeats on the way, the same copy shape as `new ArrayList<>(other)`.

```java
Set<String> rooms = new HashSet<>();
rooms.add("hall");          // true: new
rooms.add("kitchen");       // true
rooms.add("hall");          // false: already there
rooms.size();               // 2
rooms.contains("kitchen");  // true
```

**No order.** A `HashSet` promises nothing about the order it hands values back in. Never rely on it; the checker compares sets without looking at order.

## Your turn

- In `distinct`, return every tag once, as a `Set<String>`.
- In `firstRepeat`, walk the tags in order, remember each one in a set, and return the first tag that `add` says was already there. Return `null` when no tag repeats.
