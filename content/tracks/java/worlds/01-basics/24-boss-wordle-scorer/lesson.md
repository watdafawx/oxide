Wordle is the classic mix of everything in this world: `if`, `switch`, loops, characters, text and arrays. Its one tricky rule, duplicate letters, is a famous bug in clones.

**The rules.** Each guess letter gets a score: **G** for the right letter in the right place, **Y** for a letter that is in the target but elsewhere, and **.** for a letter that is not there (or already used up). `score("crane", "nacre")` is `"YYYYG"`.

**The duplicate rule.** A letter turns yellow at most as many times as the target contains it, minus the times it is already green. `score("crane", "eerie")` is `"..Y.G"`: the target has one `e`, the green one takes it, and the other `e`s get nothing.

**Two passes.** Pass 1 over the guess marks the greens, and tallies every target letter that was **not** matched. Pass 2 goes over the guess again: a letter that is not green and still has a tally above 0 becomes yellow, and uses one up.

**The tally.** A tally is an `int[26]`: one counter per letter. The letter `c` lives at index `c - 'a'`, because characters are numbers underneath, and `'d' - 'a'` is 3.

**New tools.** Everything this boss needs that you have not met yet:

```java
import java.util.Arrays;               // at the top of the file (already there)
char[] row = new char[5];                // a char array you can change one slot at a time
Arrays.fill(row, '.');                   // every slot becomes '.'
row[2] = 'G';                            // set one slot
String text = new String(row);           // char array -> String: "..G.."
int slot = 'd' - 'a';                    // 3: subtracting chars gives the distance from 'a'
int[] tally = new int[26];               // one counter per letter
tally[slot]++;                           // add one to that letter's counter
```

**Then the squares.** `render` loops over the score and uses a `switch` on each character (`case 'G' -> ...`), adding `GREEN`, `YELLOW` or `GRAY` to a `StringBuilder`.

**Work in order.** The objectives go from easy to hard. The first is a one-liner, and each later one builds on the one before.

## Your turn

- `score(target, guess)`: return a five-character text of `G`, `Y` and `.`. Start with the easy case: a guess equal to the target is `GGGGG`.
- `render(score)`: map each character to `GREEN`, `YELLOW` or `GRAY` (the constants are in the file) and join them.
