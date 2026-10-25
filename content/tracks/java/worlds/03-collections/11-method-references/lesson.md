A `Predicate` answers yes or no. Sorting, counting and grouping, the next quests, need a different kind of rule: "turn each word into its length", so they know what to compare or group by. That rule is a `Function`, and it often has a shorter spelling.

**Turn one into another.** A `Function<String, Integer>` takes a `String` and hands back an `Integer`: input type first, result type second. Like `Predicate`, it is a functional interface; its one method is `apply`, so `len.apply("cat")` is `3`.

**Name the method.** When a lambda only calls one method on its parameter, like `w -> w.length()`, you can name that method instead: `String::length`. This is a **method reference**: "the `length` method of `String`, called on whatever comes in". Same Function, less noise.

**On a class too.** A reference can also name a `static` method that takes the value as its input: `w -> Integer.parseInt(w)`, which turns text like `"42"` into the number 42, is `Integer::parseInt`. You will use `Integer::sum`, which adds two numbers, in the next quest.

```java
Function<String, String> tidy = s -> s.trim();
Function<String, String> sameTidy = String::trim;
tidy.apply("  hi ");        // "hi"
sameTidy.apply("  hi ");    // "hi"
```

**When to keep the lambda.** As soon as the lambda does more than one call, like `w -> w.length() * 2`, there is no method to name, so keep the arrow. Pick whichever reads more clearly.

## Your turn

- In `lengthByLambda`, return a `Function<String, Integer>` written as a lambda that gives the word's length.
- In `lengthByReference`, return the same Function written as a method reference.
