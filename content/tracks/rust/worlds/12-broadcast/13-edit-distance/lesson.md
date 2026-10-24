A photo called `Mos.png` and a player called Moss are one letter apart. To spot that, the asset checker needs a number for how far apart two names are, and a way to work it out that does not try every possible spelling. This quest does it on paper; the next one turns it into code.

**Edits, counted.** The **edit distance** (also called the Levenshtein distance) is the fewest one-letter edits that turn one word into another. An edit is an insert, a delete, or a swap of one letter for another. `moss` to `mos` is 1 (delete an `s`), `moss` to `mass` is 1 (swap `o` for `a`), `fen` to `fan` is 1, and `ash` to `fen` is 3.

**Too many ways.** You could try every sequence of edits, but the number of sequences explodes as words get longer. The better route answers small questions first and builds the big answer out of them. That idea is called **dynamic programming**: a table of small answers, filled in an order where every cell needs only cells that are already filled.

**The table.** Write `fen` down the side and `fan` across the top, each after an empty start `""`. The cell in a row and a column holds the distance between the start of `fen` up to that row and the start of `fan` up to that column. The top row is 0, 1, 2, 3: turning nothing into `f`, `fa`, `fan` takes that many inserts. The left column is 0, 1, 2, 3 too, with deletes.

**One cell, three neighbours.** Every other cell looks at three cells next to it that are already filled. The one to the left, plus 1: insert this column's letter. The one above, plus 1: delete this row's letter. The one up and to the left, plus 0 if the row's and the column's letters are the same, plus 1 if they differ: keep it or swap it. The cell gets the smallest of the three.

**Filled in.** Row by row, left to right, every cell from its three neighbours:

```text
        ""   f   a   n
   ""    0   1   2   3
   f     1   0   1   2
   e     2   1   1   2
   n     3   2   2   1
```

**Read one cell.** Take row `e`, column `a`. Left is 1, plus 1 makes 2. Above is 1, plus 1 makes 2. Up-left is 0, and `e` is not `a`, so plus 1 makes 1. The smallest is 1: turning `fe` into `fa` is one swap.

**Why it works.** Any way of editing one start into the other ends with a last move: an insert, a delete, or a keep or swap of the last letters. Each neighbour already holds the best way to do everything *before* that last move. So the smallest neighbour plus its move is the best way overall, and the bottom-right cell is the answer for the whole words: `fen` to `fan` is 1.

**Two rows are enough.** A cell needs only the row above it and the cell just to its left. So code keeps two rows, the `previous` one and the `row` being built, and when a row is done it becomes `previous`. The memory stays one row long, however long the names get.

**For the curious.** The table takes one step per cell, the length of one word times the other: nothing for names. `git diff` uses the same idea on lines instead of letters, with tricks that skip most of the table.
