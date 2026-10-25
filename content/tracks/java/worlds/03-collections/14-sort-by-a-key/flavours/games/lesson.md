The organiser's mod list needs two views: A to Z for finding a mod, and most downloaded first for "what is popular". Java can sort any list, but it cannot guess what "in order" means for a `Mod`. You tell it with a Comparator, and you rarely write one by hand.

**Who comes first.** A `Comparator<Mod>` answers one question about two mods: which one comes first. `list.sort(comparator)` asks that question as often as it needs to and rearranges the list.

**Build it from a key.** `Comparator.comparing(Mod::name)` means "compare mods by their name". You hand it a Function from a mod to the thing to compare: the method references from three quests ago. For an `int` key use `comparingInt(Mod::downloads)`, which compares the numbers without boxing them.

**The natural order.** Text sorts A to Z and numbers small to large: their **natural order**. `comparing` uses the key's natural order, so names come out A to Z and downloads low to high.

**Flip it.** `.reversed()` on a comparator turns its order around: `comparingInt(Mod::downloads).reversed()` puts the most downloaded mod first.

```java
List<String> words = new ArrayList<>(List.of("pear", "fig", "banana"));
words.sort(Comparator.comparingInt(String::length));             // fig, pear, banana
words.sort(Comparator.comparingInt(String::length).reversed());  // banana, pear, fig
```

**Sort a copy.** `sort` changes the list it is called on. The caller may still need the pack order, and `List.of` lists refuse to change anyway, so copy with `new ArrayList<>(mods)` and sort the copy.

## Your turn

- In `byName`, return a sorted copy of the mods, A to Z by name.
- In `byDownloadsDescending`, return a sorted copy with the most downloaded mod first.
