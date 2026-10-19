**Why now:** your status bot's `!players` command shows the online list in pages, and the status line shows how full the server is. Both come down to division on `playerCount`, and `int` division behaves in a way that surprises everyone once.

```java
int pages = 17 / 5;           // 3   (fraction thrown away, not rounded)
int left  = 17 % 5;           // 2   (the remainder)
double r  = 17 / 20;          // 0.0 (int division, THEN widened)
double ok = (double) 17 / 20; // 0.85
```

- Int division **truncates toward zero**: it never rounds.
- `%` is the remainder: "how many players are left for the last page?".
- To keep the fraction, make one side a `double` **before** dividing.

## Your turn

- Open `src/main/java/oxide/integer_division/Pages.java`.
- `pagesAndLeftover(players, perPage)`: return `{full pages, leftover players}` as an `int[]`.
- `fillRatio(players, capacity)`: return how full the server is, `0.85` for 17 of 20. The starter has the classic bug.
