RAM used is 8589934592 bytes; nobody reads that. Kinetik shows `8.0 GB`, a download of 3565158 bytes per second shows `3.4 MB`, and a tiny 512 stays `512 B`. The rule: pick the biggest unit that keeps the number at 1 or more.

**One step each.** A kilobyte is 1024 bytes, a megabyte 1024 kilobytes, a gigabyte 1024 megabytes. So divide by 1024 and move one unit up, again and again, while the number is still 1024 or more.

**Count the steps.** 3565158 divided by 1024 is 3481.6, still too big. Divide again: 3.4. Two steps, so the unit is `units[2]`, `MB`. The starter keeps that count in `unit` and the number in `value`.

**The loop.** "Repeat while a condition holds" is a `while` loop, and both numbers change inside it, so both need `mut`. Here is the same idea for a time in seconds, turned into minutes and then hours, with a second condition that stops at the last unit:

```rust
let mut time = 7200.0;
let mut step = 0;
while time >= 60.0 && step < 2 {
    time /= 60.0;
    step += 1;
}
// time is 2.0 and step is 2: hours
```

**Then write it.** The `if` under the TODO is already written: no steps means plain bytes with no decimals, otherwise one decimal and the unit's name. Your loop's last unit is `TB`, the end of `units`.

**For the curious.** Strictly, 1024 bytes is a kibibyte, `KiB`, and a kilobyte is 1000. Windows and Kinetik divide by 1024 and still write `KB`, so the bar matches Task Manager.

## Your turn

- In `format_bytes`, replace the TODO with a `while` loop that divides `value` by 1024 and counts the steps in `unit`, stopping before it runs past `TB`.
- Save. Press Run: RAM now reads in GB.
