**Why now:** your modpack updater needs to keep a list of installed mod ids and drop the ones you remove. Mod lists grow as you install, and removing one means either "the mod in slot 3" or "the mod with this project id".

An `ArrayList<T>` is a resizable array. You pick the element type in `<>` and it grows as you `add`:

```java
List<Integer> ids = new ArrayList<>();
ids.add(394468);
ids.add(306612);
ids.get(0);      // 394468
ids.size();      // 2
ids.remove(0);   // removes the mod IN SLOT 0
ids.remove(Integer.valueOf(306612));   // removes the id EQUAL to 306612
```

- Declare `List<...>`, create an `ArrayList`. The rest of the updater only ever needs a `List`.
- **The trap:** with `List<Integer>`, `ids.remove(3)` means slot 3. To remove project id 3, write `remove(Integer.valueOf(3))`.
- `List.of(...)` is read-only, so copy with `new ArrayList<>(other)` before editing.
- `new ArrayList<>(2)` is only a starting capacity. Installing the 100th mod just works.

## Your turn

- Open `src/main/java/oxide/lists/ModList.java`.
- `dropMods`: return a copy of the ids without the mod in `slot` and without the mod with `projectId`.
- `placeholderNames`: build `mod0`, `mod1`, ... in a loop for a pack that is about to get `count` mods.
