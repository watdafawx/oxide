You have every tool for a real game now: `if`, `switch`, loops, `char`s, strings and arrays. Wordle is the classic mix of all of them, and its one tricky rule (duplicate letters) is a famous bug in clones.

**Rules.** A 5-letter `target`, a 5-letter `guess`. Each guess letter gets a score:

- **G**: right letter, right place
- **Y**: the letter is in the target, but somewhere else
- **.**: the letter is not in the target (or you already used up all its copies)

`score("crane", "nacre")` is `"YYYYG"`. `score("crane", "cloud")` is `"G...."`.

**The duplicate rule.** A letter turns yellow **at most as many times as the target contains it, minus the times it is already green.** `score("crane", "eerie")` is `"..Y.G"`: the target has one `e`, the last `e` of the guess took it as green, so the other two `e`s get nothing.

**How to get there** (all lowercase a-z, same length):

- Pass 1 over the guess: mark greens, and tally the target letters that were *not* matched in an `int[26]` (the letter `c` lives at index `c - 'a'`).
- Pass 2: for each non-green guess letter, if the tally still has one, it is yellow and uses it up.

Then `render` turns the score into emoji squares for chat.

Work through the objectives in order; the first is a one-liner.

## Your turn

- Open `src/main/java/oxide/boss_wordle_scorer/Wordle.java`.
- `score(target, guess)`: return a 5-char string of `G`, `Y` and `.`.
- `render(score)`: map each char to `GREEN`, `YELLOW` or `GRAY` (constants are in the file) and join them.
