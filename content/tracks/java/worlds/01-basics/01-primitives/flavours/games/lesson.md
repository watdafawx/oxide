**Why now:** your server status bot keeps a `playerCount` and a running total of how long players have been online. A count is a small `int`. A total of player-seconds across a whole month is not: it blows past 2.1 billion, and Java will not warn you.

Primitive number types have fixed sizes:

- `int`: 32 bits, about ±2.1 billion (`Integer.MAX_VALUE` = 2147483647). Good for `playerCount`.
- `long`: 64 bits, about ±9.2 quintillion (write literals as `5L`). Good for totals and timestamps.
- `double`: 64-bit decimal, approximate. Good for TPS like `19.87`.

```java
int playerCount = Integer.MAX_VALUE;
int next = playerCount + 1;           // -2147483648: wrapped, no error
long total = (long) playerCount + 1;  // 2147483648: widened BEFORE the add
```

The cast `(long)` binds tighter than `+`, so it widens first. `(long) (a + b)` would still wrap, because the int add already happened.

## Your turn

- Open `src/main/java/oxide/primitives/StatusStats.java`.
- `addPlayers`: return the sum of two player counts (let it wrap).
- `playerSeconds`: return `players * seconds` as a correct `long`. The starter compiles but multiplies as `int`.
- Save. Both objectives turn green.
