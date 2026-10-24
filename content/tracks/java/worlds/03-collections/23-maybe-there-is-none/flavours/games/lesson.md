The organiser's header names the pack's most downloaded mod. But a brand-new instance has no mods, and "the most downloaded of no mods" has no answer. Java makes that case impossible to forget.

**max needs a rule.** `stream.max(comparator)` finds the largest item, using a Comparator to decide what "larger" means: `max(Comparator.comparingInt(Mod::downloads))`. For numbers or text on their own, `Comparator.naturalOrder()` is the rule.

**No answer is an answer.** For an empty stream there is no largest item. Instead of `null`, which you could forget to check, `max` hands back an `Optional<Mod>`: a box that holds one mod, or is empty. The type itself says "this may be missing".

**Look inside.** `isPresent()` asks whether something is inside, and `get()` takes it out, but that is the `null` check again. The better tools say what to do in both cases at once.

**map and orElse.** `opt.map(Mod::name)` turns a box with a mod into a box with its name, and leaves an empty box empty. `.orElse("none")` then takes the value out, or hands back the fallback when the box is empty. Think of an `Optional` as a stream of zero or one items.

```java
Optional<String> longest = ids.stream()
        .max(Comparator.comparingInt(String::length));
int width = longest.map(String::length).orElse(0);   // 0 for no ids
```

**For the curious.** `Optional` is meant for return values like this one, not for fields or parameters. `IntStream` has its own `OptionalInt`, which is why `mapToInt(...).max()` ends in `getAsInt()` or `orElse`.

## Your turn

- In `mostDownloaded`, return the mod with the most downloads as an `Optional<Mod>`, empty when there are no mods.
- In `headline`, use `mostDownloaded`, then `map` to the name, with `"none"` as the fallback.
