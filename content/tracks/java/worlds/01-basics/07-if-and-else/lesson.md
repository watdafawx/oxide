A number you are given can be nonsense: -1 items, or 300 on a shelf that holds 20. The program has to choose what to do depending on the number, and that is what `if` is for.

**A question and a path.** `if (condition) { ... }` runs the block only when the condition, a `boolean`, is true. When it is false, Java skips the block.

**Leave early.** A `return` inside the block ends the method on the spot. So the lines after the `if` only run when the condition was false.

**Otherwise.** `else` is the path for "the condition was false". `else if` asks a second question first. Java checks the questions top to bottom and takes the first one that is true:

**A whole chain.** Three paths, and exactly one of them runs:

```java
if (ping > 200) {
    return "laggy";
} else if (ping > 100) {
    return "slowish";
} else {
    return "fine";
}
```

**Order matters.** A ping of 250 is above 200, so it takes the first path and never reaches the second question. Put the strictest question first.

**Always braces.** Keep `{ }` around every block, even a one-line one. It stops a whole class of bugs.

## Your turn

- In `clamp`, return `lo` when `v` is below it, `hi` when it is above it, otherwise `v`.
- In `signOf`, return `-1` for a negative number, `1` for a positive one, otherwise `0`.
- Save. Both objectives turn green.
