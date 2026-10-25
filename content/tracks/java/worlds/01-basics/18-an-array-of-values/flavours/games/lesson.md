The bot logs the player count once an hour: 7, 3, 9, and so on. Twenty-four separate variables would be a mess. Java has a type for a row of values under one name.

**A row of values.** An **array** holds a fixed number of values of one type under one name. `int[] samples = {7, 3, 9};` is an array of three ints. You met `String[]` with `args`; this is the same idea.

**Index from zero.** Each value sits at an **index**, counting from 0. `samples[0]` is the first value, `samples[1]` the second. You can also store one: `samples[1] = 5;`.

**How many.** `samples.length` is the number of values. No brackets this time, unlike `length()` on text. The last index is always `length - 1`.

**All together.** Reading, counting, changing and making one:

```java
int[] samples = {7, 3, 9};
int start = samples[0];              // 7
int size = samples.length;           // 3
samples[1] = 5;                      // now {7, 5, 9}
int[] day = new int[24];          // 24 zeros
```

**Empty ones.** `new int[24]` makes an array with room for 24 ints, all starting at 0. The number in the brackets is the size, and it cannot change later.

**Out of range.** Ask for an index that does not exist, like `values[3]` in an array of three, and the program stops with an `ArrayIndexOutOfBoundsException`.

**If this is new.** Programming 101 explains lists and indexes from zero: [A list](/quest/basics/06-collections/01-a-list).

## Your turn

- In `firstSample`, return the first value of `samples`.
- In `latestSample`, return the last value.
- In `emptyDay`, return a new `int` array with room for 24 hours. Save and all three turn green.
