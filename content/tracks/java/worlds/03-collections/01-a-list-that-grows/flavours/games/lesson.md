Your modpack has 3 mods today and 40 next month. An array from the Basics world gets its size the moment you make it, so it cannot keep up. Java's list that grows is the `ArrayList`, and it is the first piece of the modpack organiser.

**Fixed size hurts.** `new String[3]` holds exactly three mod ids. For a fourth you would make a bigger array and copy every id across, every time a mod is added.

**A list that grows.** An `ArrayList` does that copying for you, out of sight. It starts empty, and each `add` puts a mod at the end and makes room when needed. `size()` says how many it holds, and `get(0)` reads the first one, counting from 0 like an array.

**Say what goes in.** `ArrayList<String>` reads "an ArrayList of Strings". The type in angle brackets is a **type parameter**: it tells Java what the list holds. This feature is called **generics**.

**Why say it at all.** With the type written once, the compiler checks every use: `mods.add(9500)` is a compile error, and `mods.get(0)` comes back as a `String`, ready for `toUpperCase()`. Without it, the list would take anything and hand back values of unknown type, and a download count among your mod ids would only show up as a crash while the organiser runs.

**The empty brackets.** On the right, `new ArrayList<>()` leaves the brackets empty. That `<>` is the **diamond**: Java copies the type from the left side, so you write `String` only once.

```java
ArrayList<String> loaders = new ArrayList<>();
loaders.add("fabric");
loaders.add("forge");
loaders.size();     // 2
loaders.get(1);     // "forge"
```

**For the curious.** Inside, an `ArrayList` is an array. When it is full, `add` makes one about 1.5 times bigger and copies across, so adding stays fast on average. The number in `new ArrayList<>(100)` only picks the starting array; the list still grows past it.

## Your turn

- The starter already has `import java.util.ArrayList;`, which tells Java where `ArrayList` lives.
- In `starterPack`, make an empty list, add `sodium`, `iris` and `lithium` in that order, and return it.
- In `placeholderNames`, use a counted loop to add `"mod" + i` for every `i` from 0 up to (not including) `count`, then return the list.
