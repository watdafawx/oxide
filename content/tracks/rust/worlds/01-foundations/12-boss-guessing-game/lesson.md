Boss time. Everything from this world goes into one small program: a **struct**, an **enum**, **match**, **Result**, a **loop**.

The game logic is separate from typing, so tests can drive it with a script. No terminal needed.

```rust
let mut game = Game::new(7);
game.guess(3);                       // Outcome::TooLow
game.guess(7);                       // Outcome::Correct
game.attempts();                     // 2

parse_guess(" 12 ");                 // Ok(12)
parse_guess("pizza");                // Err("'pizza' is not a number")

play(7, &["3", "pizza", "7"]);
// ["Too low", "'pizza' is not a number", "Correct! 2 attempts"]
```

- Bad input prints its error but is **not** an attempt.
- `play` stops at the first correct guess; later inputs are ignored.
- "1 attempt", but "2 attempts".

## Toolbox for `play`

New here: a **`Vec`** is a growable list. You will build the output with one.

```rust
let mut messages: Vec<String> = Vec::new();   // empty list
messages.push("Too low".to_string());         // add to the end
for m in &messages { println!("{m}"); }       // `&` = look at each, don't take the list
```

- `for input in inputs` works the same on the `&[&str]` slice you are given.
- `break` leaves the loop early.

## Your turn

- Open `src/lib.rs`. `Outcome` and `Game` are given.
- Do the objectives top to bottom: `new`, `guess`, `attempts`, `parse_guess`, then `play`.
- The first objective is one line. Take the win, then keep going.

Press Run to play it.
