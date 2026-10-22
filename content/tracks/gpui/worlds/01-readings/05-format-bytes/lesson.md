RAM used is 8589934592 bytes; nobody reads that. Kinetik shows `8.0 GB`, a download of 3565158 bytes per second shows `3.4 MB`, and a tiny 512 stays `512 B`. The rule: pick the biggest unit that keeps the number at 1 or more.

**One step at a time.** A kilobyte is 1024 bytes, a megabyte 1024 kilobytes, a gigabyte 1024 megabytes. So divide by 1024 and move one unit up, again and again, while the number is still 1024 or more.

**Count the steps.** 3565158 divided by 1024 is 3481.6, still too big. Divide again: 3.4. Two steps, so the unit is `units[2]`, `MB`. The starter keeps that count in `unit` and the number in `value`.

**The loop.** "Repeat while a condition holds" is a `while` loop. The second half of the condition stops at `TB`, the last unit in the list:

```rust
while value >= 1024.0 && unit < units.len() - 1 {
```

**Then write it.** The `if` under the TODO is already written: no steps means plain bytes with no decimals, otherwise one decimal and the unit's name.

## Your turn

- In `format_bytes`, replace the TODO with the `while` loop: inside it, divide `value` by `1024.0` and add 1 to `unit`.
- Save. Press Run: RAM now reads in GB.
