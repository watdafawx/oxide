The results page announces the best player. But on the first evening nobody has played yet, and "the best of no players" has no answer. Java makes that case impossible to forget.

**max needs a rule.** `stream.max(comparator)` finds the largest item, using a Comparator to decide what "larger" means: `max(Comparator.comparingInt(Player::score))`. For numbers or text on their own, `Comparator.naturalOrder()` is the rule.

**No answer is an answer.** For an empty stream there is no largest item. Instead of `null`, which you could forget to check, `max` hands back an `Optional<Player>`: a box that holds one player, or is empty. The type itself says "this may be missing".

**Look inside.** `isPresent()` asks whether something is inside, and `get()` takes it out, but that is the `null` check again. The better tools say what to do in both cases at once.

**map and orElse.** `opt.map(Player::name)` turns a box with a player into a box with their name, and leaves an empty box empty. `.orElse("nobody")` then takes the value out, or hands back the fallback when the box is empty. Think of an `Optional` as a stream of zero or one items.

```java
Optional<String> longest = words.stream()
        .max(Comparator.comparingInt(String::length));
int letters = longest.map(String::length).orElse(0);   // 0 for no words
```

**For the curious.** `Optional` is meant for return values like this one, not for fields or parameters. `IntStream` has its own `OptionalInt`, which is why `mapToInt(...).max()` ends in `getAsInt()` or `orElse`.

## Your turn

- In `bestPlayer`, return the player with the highest score as an `Optional<Player>`, empty when there are no players.
- In `winnerName`, use `bestPlayer`, then `map` to the name, with `"nobody"` as the fallback.
