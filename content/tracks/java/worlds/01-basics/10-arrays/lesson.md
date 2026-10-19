Arrays are Java's most basic container: a fixed number of values of one type, side by side, found by index. Lists (soon) are friendlier, but arrays are under everything, and 2D grids are arrays of arrays.

```java
int[] scores = {7, 3, 9};             // length 3, indexes 0..2
scores[0] = 8;                        // arrays are mutable
int n = scores.length;                // no parentheses on arrays
int[] zeros = new int[5];             // five 0s
int[][] grid = {{1, 2, 3}, {4, 5, 6}};   // 2 rows, 3 columns
grid[1][2];                           // 6: row 1, column 2

java.util.Arrays.sort(scores);        // sorts in place
java.util.Arrays.toString(scores);    // "[3, 7, 8]" (printing an array directly shows junk)
```

- Index goes from `0` to `length - 1`. Anything else throws `ArrayIndexOutOfBoundsException`.
- Passing an array to a method shares it: the method can change *your* array. That is how `reverseInPlace` can return `void`.
- Starting a "largest so far" at `0` is a bug when all values are negative: start from the first element.

## Your turn

- Open `src/main/java/oxide/arrays/ArrayWork.java`.
- `max(values)`: the largest value.
- `reverseInPlace(values)`: reverse the same array, no new array.
- `transpose(grid)`: a new grid where rows and columns are swapped.
