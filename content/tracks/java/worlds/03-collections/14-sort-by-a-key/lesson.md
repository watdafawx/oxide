A leaderboard is the same players in a useful order. Java can sort any list, but it cannot guess what "in order" means for a `Player`: by name? by score? You tell it with a Comparator, and you rarely write one by hand.

**Who comes first.** A `Comparator<Player>` answers one question about two players: which one comes first. `list.sort(comparator)` asks that question as often as it needs to and rearranges the list.

**Build it from a key.** `Comparator.comparing(Player::name)` means "compare players by their name". You hand it a Function from a player to the thing to compare: the method references from three quests ago. For an `int` key use `comparingInt(Player::score)`, which compares the numbers without boxing them.

**The natural order.** Text sorts A to Z and numbers small to large: their **natural order**. `comparing` uses the key's natural order, so names come out A to Z and scores low to high.

**Flip it.** `.reversed()` on a comparator turns its order around: `comparingInt(Player::score).reversed()` puts the highest score first.

```java
List<String> words = new ArrayList<>(List.of("pear", "fig", "banana"));
words.sort(Comparator.comparingInt(String::length));             // fig, pear, banana
words.sort(Comparator.comparingInt(String::length).reversed());  // banana, pear, fig
```

**Sort a copy.** `sort` changes the list it is called on. The caller may still need their order, and `List.of` lists refuse to change anyway, so copy with `new ArrayList<>(players)` and sort the copy.

## Your turn

- In `byName`, return a sorted copy of the players, A to Z by name.
- In `byScoreDescending`, return a sorted copy with the highest score first.
