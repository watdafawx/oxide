A `Predicate` answers yes or no. Sorting, counting and grouping the pack, the next quests, need a different kind of rule: "turn each mod into its name", so they know what to compare or group by. That rule is a `Function`, and it often has a shorter spelling.

**Turn one into another.** A `Function<Mod, String>` takes a `Mod` and hands back a `String`: input type first, result type second. Like `Predicate`, it is a functional interface; its one method is `apply`, so `name.apply(sodium)` is `"Sodium"`.

**Name the method.** When a lambda only calls one method on its parameter, like `mod -> mod.name()`, you can name that method instead: `Mod::name`. This is a **method reference**: "the `name` method of `Mod`, called on whatever comes in". Same Function, less noise.

**On a class too.** A reference can also name a `static` method that takes the value as its input: `s -> Integer.parseInt(s)`, which turns text like `"42"` into the number 42, is `Integer::parseInt`. You will use `Integer::sum`, which adds two numbers, in the next quest.

```java
Function<String, String> tidy = s -> s.trim();
Function<String, String> sameTidy = String::trim;
tidy.apply("  sodium ");        // "sodium"
sameTidy.apply("  sodium ");    // "sodium"
```

**When to keep the lambda.** As soon as the lambda does more than one call, like `mod -> mod.name().toUpperCase()`, there is no single method to name, so keep the arrow. Pick whichever reads more clearly.

## Your turn

- In `nameByLambda`, return a `Function<Mod, String>` written as a lambda that gives the mod's name.
- In `nameByReference`, return the same Function written as a method reference.
