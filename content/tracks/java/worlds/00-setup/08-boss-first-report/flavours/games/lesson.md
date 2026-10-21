The status bot has to answer the real command now: `java StatusCommand play.example.net`. Put together everything from this world and it prints a two-line report.

**The small ones.** `announce("play.example.net")` returns `Status bot online: play.example.net`. `occupancy("play.example.net", 3, 20)` returns `play.example.net has 3 of 20 players`. `serverOf(args)` returns the first word typed.

**The report.** `report(args, 3, 20)` returns both lines, one after the other.

**Methods can call methods.** Inside one method you can call another method of your own and use what it hands back, as if it were a value, and you can hand it straight to another call: `shout(twice(word))`. Here `shout` calls `twice`:

```java
public static String twice(String word) {
    return word + " " + word;
}

public static String shout(String word) {
    return twice(word) + "!";
}
```

**A new line.** Inside a text, `\n` means "start a new line". So `"one\ntwo"` is two lines. The last method needs it between the two lines of the report.

**The order to work in.** Start with the three small ones. `report` at the end only calls the others and glues their results together, so it is the easy last step.

## Your turn

- Write `announce`, `occupancy` and `serverOf`. Each tick turns green as you finish.
- In `report`, call `serverOf`, then `announce` and `occupancy`, and join the two lines with `\n`.
- Save. All four objectives turn green.
