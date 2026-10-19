**Why now:** the status bot remembers the last 24 hourly `playerCount` samples to draw a graph and show the daily **peak**. Samples arrive oldest first, the graph wants newest first, and with several servers you get a grid (servers × hours) that the table wants the other way round. Arrays hold all of that.

```java
int[] samples = {7, 3, 9};            // length 3, indexes 0..2
samples[0] = 8;                       // arrays are mutable
int n = samples.length;               // no parentheses on arrays
int[][] grid = {{1, 2, 3}, {4, 5, 6}};   // 2 servers, 3 hours
grid[1][2];                           // 6: server 1, hour 2

java.util.Arrays.sort(samples);       // sorts in place
java.util.Arrays.toString(samples);   // "[3, 7, 8]" (printing an array directly shows junk)
```

- Index goes from `0` to `length - 1`. Anything else throws `ArrayIndexOutOfBoundsException`.
- Passing an array to a method shares it: the method can change *your* array. That is how `newestFirst` can return `void`.
- Starting "peak so far" at `0` is a bug if every sample is negative: start from the first element.

## Your turn

- Open `src/main/java/oxide/arrays/PlayerHistory.java`.
- `peak(samples)`: the highest player count.
- `newestFirst(samples)`: reverse the same array, no new array.
- `byHour(grid)`: a new grid where servers and hours are swapped.
