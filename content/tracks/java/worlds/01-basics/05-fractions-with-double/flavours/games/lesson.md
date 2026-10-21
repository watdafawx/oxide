How full is the server? 17 players on a 20-slot server is 0.85. But ask Java for `17 / 20` and the answer is 0, because both numbers are `int`s.

**The trap again.** Two `int`s divided give an `int`, so `17 / 20` is `0`. Putting that into a `double` afterwards does not bring the fraction back: it is already gone.

**A number with a point.** A **`double`** is a number with a decimal point, like `0.85` or `2.0`. It is the type for fractions, averages and anything that is not whole.

**One side is enough.** If either side of a `/` is a `double`, Java divides as `double`. So `17 / 20.0` is `0.85`. Write `2.0` instead of `2` when you want a fraction.

**Or cast.** The cast from the `long` quest works here too. `(double)` turns a value into a `double` before the division:

```java
double ratio = (double) taken / total;   // 17 and 20 give 0.85
```

**Not exact.** A `double` is a close guess, not exact: `0.1 + 0.2` is `0.30000000000000004`. So the checker accepts answers that are very close, instead of exactly equal.

## Your turn

- In `fillRatio`, return `online` divided by `capacity` as a `double`, using the cast.
- In `averagePing`, return the average of the two pings, dividing by `2.0`.
- Save. Both objectives turn green.
