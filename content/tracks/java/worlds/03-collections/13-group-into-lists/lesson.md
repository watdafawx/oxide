A word game sorts its dictionary into piles by length, so it can pick "a 5-letter word" fast. That is a map from a length to a list of words: `Map<Integer, List<String>>`. The trouble is the first word of each length, when its pile does not exist yet.

**The clumsy way.** For every word: `get` the list for its length; if that is `null`, make an empty list and `put` it; then add the word. Four lines and an `if`, every time you group.

**computeIfAbsent.** `groups.computeIfAbsent(5, rule)` hands back the value for key 5. If there is none yet, it first calls `rule` to make one, stores it, and hands back that. So the list you get is always real, and you can `add` to it straight away.

**The rule makes a list.** The rule is a lambda that takes the key and makes the new value: `k -> new ArrayList<>()`. It gets the key in case the new value depends on it; here it does not, so `k` goes unused.

```java
Map<String, List<String>> byShelf = new HashMap<>();
byShelf.computeIfAbsent("A", k -> new ArrayList<>()).add("Dune");
byShelf.computeIfAbsent("A", k -> new ArrayList<>()).add("Emma");   // same list
byShelf.computeIfAbsent("B", k -> new ArrayList<>()).add("Ulysses");
// {A=[Dune, Emma], B=[Ulysses]}
```

**Lists inside a map.** `Map<Integer, List<String>>` nests one generic type in another: the values are lists of words. Read it from the outside in, "a map from Integer to list of String".

## Your turn

- In `byLength`, for each word, get the list for its length with `computeIfAbsent` (making an empty one when missing) and add the word to it. Words keep their input order inside each list.
