Windows only gives CPU temperatures to apps running as administrator. Without that, Kinetik cannot read the number at all, and the bar shows `--°C` instead of making one up. Your model needs a way to say "this stat, and maybe a value".

**A reading.** `Reading` pairs a `kind` with a `value`. The value is an `Option<f64>`: `Some(47.0)` when Kinetik read 47 °C, `None` when it could not.

**Why not zero.** Storing 0.0 for "unknown" would put `0°C` on the bar, which looks like a real, very cold CPU. `None` keeps "unknown" and "zero" apart, and the compiler makes every caller handle both.

**What the bar shows.** `text` turns a reading into what the bar draws. The `Some` arm already writes the number. The `None` arm gives back an empty `String` for now, so the pill would be blank. Kinetik writes two dashes:

```rust
None => "--".to_string(),
```

**Callers stay honest.** Every place that shows a reading goes through `text`, so none of them can forget the missing case. The bar, the hover card and the overlay all print `--` the same way.

## Your turn

- In `text`, change the `None` arm so it gives back `"--"`.
- Save. Press Run to see a reading with a value and one without.
