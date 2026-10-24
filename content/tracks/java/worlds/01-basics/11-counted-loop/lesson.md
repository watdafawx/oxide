You want 1 + 2 + ... + N for any N, and a row of N stars. Doing that by hand every time is silly. Repeating something is a job for a loop.

**Do it again.** A **loop** repeats a block of code. The counted `for` loop is the one to use when you know how many rounds you want.

**Three parts.** `for (int i = 1; i <= n; i++)` has three parts between the brackets, separated by `;`. First where to start, `int i = 1`. Then how long to keep going, `i <= n`. Last what to do after every round, `i++`.

**Plus plus.** `i++` adds 1 to `i`. And `total += 10` is shorthand for `total = total + 10`: add to what is already there. The same `+=` works on text, to add to the end of it.

**A running total.** Make the variable before the loop, change it inside:

```java
int total = 0;
for (int i = 1; i <= 3; i++) {
    total += 10;   // adds 10 three times
}
```

**Off by one.** `i <= n` includes `n` itself, `i < n` stops one early. Counting one too many or one too few is the most common loop bug, so always check the first and last round.

**If this is new.** Programming 101 explains counting loops from zero: [Count with for](/quest/basics/04-repetition/02-count-with-for).

## Your turn

- In `sumTo`, add the numbers 1 up to and including `n` with a counted `for` loop.
- In `stars`, build a text of `n` stars (`*`), one per round, with a loop.
- Save. Both objectives turn green.
