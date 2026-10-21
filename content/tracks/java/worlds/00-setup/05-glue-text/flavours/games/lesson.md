The status bot should announce who joined, by name. A line that always says the same thing is no use, so the method has to be handed the player's name and glue it into the text.

**Hand it a name.** A method can be given values to work with. They go in the brackets: `String player` means "I need a piece of text, and inside the method I will call it `player`". The word in the brackets is a **parameter**. The caller picks the actual text, for example `announceJoin("Ada")`.

**Glue with plus.** A `+` between two pieces of text joins them into one. If `player` is `Ada`, then `player + " joined the game"` is `Ada joined the game`.

**Spaces are text too.** Nothing adds spaces for you. The space inside the quotes is part of the text, and if you forget it the words run together.

**Put it together.** The method takes the parameter and hands the glued text back:

```java
public static String announceJoin(String player) {
    return player + " joined the game";
}
```

**More than one.** A method can take several parameters. Separate them with commas: `String first, String second`. Inside, each one is its own name.

**What the checker does.** It calls your method with different names, such as `announceJoin("Ada")` and `announceJoin("Bo")`, and compares the result with the text it expects ("Ada joined the game").

## Your turn

- In `announceJoin`, return the text with the player glued in.
- In `announcePair`, do the same with both names: "Ada and Bo joined the game".
- Save. Both objectives turn green.
