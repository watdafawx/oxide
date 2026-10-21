The status bot gets a new command, `!guess`. Players try to guess a secret five-letter codeword, and the bot answers with a row of coloured squares. You have every tool for it now: `if`, `switch`, loops, characters, text and arrays.

**The rules.** Each guess letter gets a score: **G** for the right letter in the right place, **Y** for a letter that is in the secret but elsewhere, and **.** for a letter that is not there (or already used up). `score("slime", "smile")` is `"GYGYG"`.

**The duplicate rule.** A letter turns yellow at most as many times as the secret contains it, minus the times it is already green. `score("pearl", "eerie")` is `".GY.."`: the secret has one `e`, the green one takes it, and the other `e`s get nothing.

**Two passes.** Pass 1 over the guess marks the greens, and tallies every secret letter that was **not** matched. Pass 2 goes over the guess again: a letter that is not green and still has a tally above 0 becomes yellow, and uses one up.

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

**Then the squares.** `render` loops over the score and uses a `switch` on each character (`case 'G' -> ...`), adding `GREEN`, `YELLOW` or `GRAY` to a `StringBuilder`. `reply` just calls `score` and `render` and puts the player's name in front.

**Work in order.** The objectives go from easy to hard. The first is a one-liner, and each later one builds on the one before.

## Your turn

- `score(secret, guess)`: return a five-character text of `G`, `Y` and `.`. Start with the easy case: a guess equal to the secret is `GGGGG`.
- `render(score)`: map each character to `GREEN`, `YELLOW` or `GRAY` (the constants are in the file) and join them.
- `reply(player, secret, guess)`: return the player's name, a colon and a space, then the emoji row.
