Kinetik's System group has an uptime stat: how long since Windows started. `sysinfo::System::uptime()` gives seconds, like 277 200. Nobody reads that; Kinetik writes `3d 5h`.

**Whole units with /.** A day is 86 400 seconds, an hour 3600, a minute 60. On whole numbers, `/` drops the remainder: 277 200 / 86 400 is 3, three whole days.

**What is left with %.** `%` gives the remainder instead: 277 200 % 86 400 is 18 000, the seconds that did not fill a day. Divide that by 3600 for the whole hours. Minutes work the same way, from what is left after whole hours. Here is a 125-second download split into minutes and seconds:

```rust
let minutes = seconds / 60;
let rest = seconds % 60;
```

**Two units are enough.** `3d 5h 12m 40s` is noise on a taskbar. Show the two biggest units that matter: days and hours, else hours and minutes, else minutes.

**Pick by size.** An `if` / `else if` / `else` on `days > 0` and `hours > 0` picks the format. Each branch is one `format!`. Uptime is not on the default bar; the widget's System group and the dashboard show it in later worlds.

## Your turn

- In `format_uptime`, work out `days`, `hours` and `minutes`.
- Give back `"{days}d {hours}h"`, `"{hours}h {minutes}m"` or `"{minutes}m"`, by size.
- Save.
