A battery at 15 % should turn red, and one at 80 % should not. Your `is_warning` only knows "too high is bad", so right now 80 % is the alarming one. Battery and free disk space are the other way round.

**A direction.** Each limit now says which side is bad. `WarnWhen` has two variants, `Above` and `Below`, and a `Limit` holds the number `at` and the direction `when`. Open `limits.rs` to see them; the battery's limit is 20, `Below`.

**Ask the limit.** In the arm where both are there, the answer now depends on `limit.when`. An arm's answer can itself be a `match`, with its own arms, and a comma after its closing brace. Writing a reading by its unit could look like this:

```rust
Some(v) => match self.kind.unit() {
    Unit::Celsius => format!("{v:.0}°C"),
    _ => format_percent(v),
},
```

**Equal still warns.** A battery at exactly 20 % with a limit of 20 is already red, the same rule as 85 °C for the CPU: `<=`, not `<`.

**One place to change.** Every pill on the bar asks `is_warning`. Fixing this one arm fixes the battery everywhere, and free disk space later gets it for free.

## Your turn

- In `is_warning`, make the both-there arm decide by `limit.when`: `Above` warns at or over `limit.at`, `Below` warns at or under it.
- Save. Press Run to see 15 % and 80 % batteries.
