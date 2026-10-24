A crossword grid has room for words of at most 6 letters, so every longer word must go from the list. Lists can do that in one call, `removeIf`, but only if you can hand it the rule "longer than 6". A rule is code, not a value, and today you write code as a value.

**Hand over a rule.** `words.removeIf(rule)` removes every item the rule says yes to. The list does the loop; you only supply the question to ask about each word.

**The arrow.** A **lambda** is a small unnamed method written in one line: `w -> w.length() > 6`. Left of the arrow is the parameter, right of it is the result. Read it as "given a word `w`, answer whether its length is over 6".

**Its type.** A lambda that answers yes or no about a `String` is a `Predicate<String>`. You can store it in a variable, return it from a method, and ask it about one word with `test`: `rule.test("bookshelf")` is `true`.

**Why it works.** `Predicate` is an interface with a single method, `test`. A lambda is the short way to write an object that keeps that one-method promise. Java calls such an interface a **functional interface**.

```java
Predicate<String> isBlank = s -> s.trim().length() == 0;
isBlank.test("   ");        // true
List<String> names = new ArrayList<>(List.of("Ada", " ", "Linus"));
names.removeIf(isBlank);    // [Ada, Linus]
```

**Using outside values.** A lambda can read the method's parameters and variables, like `n` in `w -> w.length() > n`, as long as they never change after being set (**effectively final**). The reason: the lambda may run long after the method has returned, so Java hands it a copy of the value. If the variable could still change, that copy would silently be out of date.

## Your turn

- In `longerThan`, return a `Predicate<String>` lambda that is true when the word is longer than `n`.
- In `dropLong`, copy the list, call `removeIf` on the copy with your `longerThan(n)` rule, and return the copy.
