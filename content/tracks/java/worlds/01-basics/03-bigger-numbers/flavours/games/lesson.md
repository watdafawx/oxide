100,000 players online for 100,000 seconds each is 10 billion player-seconds, far more than an `int` holds. The bot needs a bigger box for totals like that.

**A bigger box.** A `long` is a whole number with twice the room of an `int`: up to about 9 quintillion. It is the type for counts that can pass 2.1 billion.

**The L.** Java reads a bare number as an `int`, so a number too big for an `int` must say it is a `long` with an `L` on the end. Underscores are allowed to make big numbers readable: `3_000_000_000L`.

**Too late.** `players * seconds` multiplies two `int`s, so the product wraps first and only then becomes a `long`. A wrong answer, widened.

**Cast early.** A **cast** like `(long)` in front of a value turns it into a `long` before the maths. Put it on one side and the whole multiplication is done as `long`:

```java
int players = 100_000;
int seconds = 100_000;
long total = (long) players * seconds;   // 10000000000
```

**Cast on the right spot.** The cast sticks to the value right after it. `(long) (players * seconds)` is too late, because the brackets multiply as `int` first.

## Your turn

- In `bytesSentLastYear`, return the number 6 billion as a `long` (it needs an `L`).
- In `playerSeconds`, make the multiplication happen as a `long`, using the cast from the lesson.
- Save. Both objectives turn green.
