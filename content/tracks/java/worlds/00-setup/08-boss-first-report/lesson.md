A conference prints a badge for everyone who arrives: a welcome line for the event, then a seat line for the guest. Put together everything from this world and one command prints the whole badge.

**The spec.** Four small methods, each one a tick:

- `title("DevDay")` returns `Welcome to DevDay`.
- `seatLine("Ada", 3, 40)` returns `Ada: seat 3 of 40`.
- `eventOf(args)` returns the first word typed.
- `badge(args, "Ada", 3, 40)` returns both lines, one after the other.

**Methods can call methods.** Inside one method you can call another method of your own and use what it hands back, as if it were a value, and you can hand it straight to another call: `shout(twice(word))`. Here `shout` calls `twice`:

```java
static String twice(String word) {
    return word + " " + word;
}

public static String shout(String word) {
    return twice(word) + "!";
}
```

**A new line.** Inside a text, `\n` means "start a new line". So `"one\ntwo"` is two lines. The last method needs it between the two lines of the report.

**The order to work in.** Start with the three small ones. `badge` at the end only calls the others and glues their results together, so it is the easy last step.

## Your turn

- Write `title`, `seatLine` and `eventOf`. Each tick turns green as you finish.
- In `badge`, call `eventOf`, then `title` and `seatLine`, and join the two lines with `\n`.
- Save. All four objectives turn green.
