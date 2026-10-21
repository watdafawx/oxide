Status lines are mostly fixed text with a few values dropped in: `play.example.net: 3/20 online`. Gluing with `+` works, but a template with slots is easier to read and to get right.

**A template.** `String.format` takes a text with **slots** in it, then the values to put in them. It fills the slots in order, left to right, and hands back the finished text.

**Two kinds of slot.** `%s` is a slot for text. `%d` is a slot for a whole number. So `String.format("%s has %d items", "Ada", 3)` gives `Ada has 3 items`.

**Whole numbers.** A whole number has the type `int`, short for integer. You write it in the brackets like any parameter: `String server, int online, int max`.

**Order is everything.** The first value goes in the first slot, the second in the second. Everything outside the slots, such as `:` and `/`, is copied exactly as written.

**Wrong slot, red.** If a `%d` meets text, Java stops with an error while running, and the checker shows it. Match each slot to its value.

Here is a whole method:

```java
public static String statusLine(String server, int online, int max) {
    return String.format("%s: %d/%d online", server, online, max);
}
```

## Your turn

- In `statusLine`, build "play.example.net: 3/20 online" with `String.format`.
- In `onlineLine`, build "survival: 12 online" the same way.
- Save. Both objectives turn green.
