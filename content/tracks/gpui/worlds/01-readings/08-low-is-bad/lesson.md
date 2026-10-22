A battery at 15 % should turn red, and one at 80 % should not. Your `is_warning` only knows "too high is bad", so right now 80 % is the alarming one. Battery and free disk space are the other way round.

**A direction.** Each limit now says which side is bad. `WarnWhen` has two variants, `Above` and `Below`, and a `Limit` holds the number `at` and the direction `when`. Open `limits.rs` to see them; the battery's limit is 20, `Below`.

**Ask the limit.** In the arm where both are there, decide by direction. Above means `v >= limit.at`, below means `v <= limit.at`:

```rust
match limit.when { WarnWhen::Above => v >= limit.at, WarnWhen::Below => v <= limit.at }
```

**One place to change.** Every pill on the bar asks `is_warning`. Fixing this one arm fixes the battery everywhere, and free disk space later gets it for free.

## Your turn

- In `is_warning`, replace `v >= limit.at` with the `match` on `limit.when`.
- Save. Press Run to see 15 % and 80 % batteries.
