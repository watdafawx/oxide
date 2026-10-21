Which hour was the busiest? And the quietest? With a row of hourly samples, the answer comes from looking at every one and remembering the best so far.

**Keep a champion.** Make a variable for the best value seen so far. Visit every value, and whenever one beats the champion, it takes its place.

**Visit every one.** The enhanced `for` works on any array, not only on characters: `for (int v : samples)` gives you each value as `v`.

**The pattern.** Here the champion is the longest word so far:

```java
int longest = 0;
for (String w : words) {
    if (w.length() > longest) {
        longest = w.length();
    }
}
```

**Where to start.** For lengths, 0 is a safe start. For numbers that can be negative it is not: an array of only -5 and -2 would wrongly keep 0 as its "largest". Start from the first element, `samples[0]`, so the champion is always a real value.

**Challenger wins.** The test inside the `if` decides who wins. Flip the comparison and the same loop finds the smallest instead of the largest.

## Your turn

- In `peak`, return the largest value. Start from the first element, not from 0.
- In `low`, return the smallest value the same way. Save and both objectives turn green.
