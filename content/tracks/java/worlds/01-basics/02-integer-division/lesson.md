Pagination, change-making, progress bars: real code constantly asks "how many full groups, and what is left?". Integer division answers exactly that.

```java
int groups = 17 / 5;          // 3   (fraction thrown away, not rounded)
int left   = 17 % 5;          // 2   (the remainder)
double r   = 17 / 5;          // 3.0 (still int division, THEN widened)
double ok  = (double) 17 / 5; // 3.4
double ok2 = 17 / 5.0;        // 3.4 (one side is already a double)
```

- Int division **truncates toward zero**: `-7 / 2` is `-3`, and `-7 % 2` is `-1`.
- Dividing by zero throws on ints, but gives `Infinity` on doubles.

## Your turn

- Open `src/main/java/oxide/integer_division/Division.java`.
- `divmod(a, b)`: return `{quotient, remainder}` as an `int[]`.
- `ratio(a, b)`: return the exact fraction as a `double`. The starter has the classic bug.
