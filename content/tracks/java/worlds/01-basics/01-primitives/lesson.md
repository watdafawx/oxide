A program that counts things (users, bytes, orders) fails in an ugly way when the number gets too big: Java does **not** crash, it wraps around and carries on with a wrong answer.

Primitive number types have fixed sizes:

- `int`: 32 bits, about ±2.1 billion (`Integer.MAX_VALUE` = 2147483647)
- `long`: 64 bits, about ±9.2 quintillion (write literals as `5L`)
- `double`: 64-bit decimal, approximate (`0.1 + 0.2` is not exactly `0.3`)

```java
int big = Integer.MAX_VALUE;
int next = big + 1;           // -2147483648: wrapped, no error
long safe = (long) big + 1;   // 2147483648: widened BEFORE the add
```

The cast `(long)` binds tighter than `+`, so it widens `big` first. Writing `(long) (big + 1)` would still wrap, because the int add already happened.

## Your turn

- Open `src/main/java/oxide/primitives/Primitives.java`.
- `addInts`: return the sum of two ints (let it wrap).
- `addAsLong`: return the correct sum as a `long`. The starter already compiles but adds as `int`.
- Save. Both objectives turn green.
