**Why it matters:** arrays have a fixed size, real data does not. Every program that collects things (names, scores, files) starts with a `List`.

An `ArrayList<T>` is a resizable array. You pick the element type in `<>` and it grows as you `add`:

```java
List<String> names = new ArrayList<>();   // starts empty, no size to choose
names.add("Ada");
names.add("Grace");
names.get(0);       // "Ada"
names.size();       // 2
names.remove(0);    // removes the element AT index 0
names.remove("Grace");   // removes the first element EQUAL to "Grace"
```

- Declare the variable as `List<...>`, create an `ArrayList`. Code that only needs a `List` keeps working if you swap the implementation.
- **The trap:** with `List<Integer>`, `remove(3)` means "index 3". To remove the *value* 3, write `remove(Integer.valueOf(3))`.
- `List.of(1, 2, 3)` is read-only, so copy it with `new ArrayList<>(other)` before changing it.
- The number in `new ArrayList<>(2)` is only a starting capacity. The list grows on its own past it.

## Your turn

- Open `src/main/java/oxide/lists/Lists.java`.
- `removeIndexAndValue`: return a copy of the list without the element at `index` and without the first element equal to `value`.
- `labels`: build `item0`, `item1`, ... in a loop and return the list.
