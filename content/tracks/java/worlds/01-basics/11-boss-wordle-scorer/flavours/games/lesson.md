**Why now:** the status bot gets a mini-game. Players type `!guess <word>` in the server chat; the bot has a 5-letter secret **codeword** of the day and answers with a row of coloured squares. This is the whole world in one feature: a `String` in, `char`s walked with loops and `switch`, an `int[]` tally, and a reply built by calling smaller methods, just like the status line.

**Rules.** A 5-letter `secret`, a 5-letter `guess`. Each guess letter gets a score:

- **G**: right letter, right place
- **Y**: the letter is in the secret, but somewhere else
- **.**: the letter is not in the secret (or you already used up all its copies)

`score("slime", "smile")` is `"GYGYG"`. `score("sword", "words")` is `"YYYYY"`.

**The duplicate rule.** A letter turns yellow **at most as many times as the secret contains it, minus the times it is already green.** `score("pearl", "eerie")` is `".GY.."`: the secret has one `e`, and the second `e` of the guess took it as green, so the other `e`s get nothing.

**How to get there** (all lowercase a-z, same length):

- Pass 1 over the guess: mark greens, and tally the secret letters that were *not* matched in an `int[26]` (the letter `c` lives at index `c - 'a'`).
- Pass 2: for each non-green guess letter, if the tally still has one, it is yellow and uses it up.

`render` turns the score into emoji squares, and `reply("Ada", secret, guess)` gives the chat line `Ada: <squares>`.

New tools for this boss:

```java
import java.util.Arrays;                  // at the top of the file (already there): lets you write Arrays
char[] row = new char[5];                 // a char[] you can change one slot at a time
Arrays.fill(row, '.');                    // every slot becomes '.'
row[2] = 'G';                             // set one slot
String text = new String(row);            // char[] -> String: "..G.."
int slot = 'd' - 'a';                     // 3: subtracting chars gives the distance from 'a'
```

Work through the objectives in order; the first is a one-liner.

## Your turn

- Open `src/main/java/oxide/boss_wordle_scorer/Codeword.java`.
- `score(secret, guess)`: return a 5-char string of `G`, `Y` and `.`.
- `render(score)`: map each char to `GREEN`, `YELLOW` or `GRAY` (constants are in the file) and join them.
- `reply(player, secret, guess)`: the chat line, built from `score` and `render`.
