You have a row of numbers and need the first and the last, or an empty row of a given size. Twenty separate variables would be a mess. Java has a type for a row of values under one name.

**A row of values.** An **array** holds a fixed number of values of one type under one name. `int[] values = {7, 3, 9};` is an array of three ints. You met `String[]` with `args`; this is the same idea.

**Index from zero.** Each value sits at an **index**, counting from 0. `values[0]` is the first value, `values[1]` the second. You can also store one: `values[1] = 5;`.

**How many.** `values.length` is the number of values. No brackets this time, unlike `length()` on text. The last index is always `length - 1`.

**All together.** Reading, counting, changing and making one:

```java
int[] values = {7, 3, 9};
int start = values[0];              // 7
int size = values.length;           // 3
values[1] = 5;                      // now {7, 5, 9}
int[] day = new int[24];          // 24 zeros
```

**Empty ones.** `new int[24]` makes an array with room for 24 ints, all starting at 0. The number in the brackets is the size, and it cannot change later.

**Out of range.** Ask for an index that does not exist, like `values[3]` in an array of three, and the program stops with an `ArrayIndexOutOfBoundsException`.

**If this is new.** Programming 101 explains lists and indexes from zero: [A list](/quest/basics/06-collections/01-a-list).

## Your turn

- In `first`, return the first value of `values`.
- In `last`, return the last value.
- In `zeros`, return a new `int` array with room for `count` numbers. Save and all three turn green.
