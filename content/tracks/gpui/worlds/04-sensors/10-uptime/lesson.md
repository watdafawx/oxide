Kinetik's System group has an uptime stat: how long since Windows started. `sysinfo::System::uptime()` gives seconds, like 277 200. Nobody reads that; Kinetik writes `3d 5h`.

**Split the seconds.** A day is 86 400 seconds, an hour 3600, a minute 60. Whole days are `seconds / 86_400`. The leftover after whole days is `seconds % 86_400`, and whole hours of that are `/ 3600`. Minutes work the same way:

```rust
let hours = seconds % 86_400 / 3600;
```

**Two units are enough.** `3d 5h 12m 40s` is noise on a taskbar. Show the two biggest units that matter: days and hours, else hours and minutes, else minutes.

**Pick by size.** An `if` / `else if` / `else` on `days > 0` and `hours > 0` picks the format. Each branch is one `format!`.

**Where it shows.** Uptime is not on the default bar. The widget's System group and the dashboard show it in later worlds.

## Your turn

- In `format_uptime`, work out `days`, `hours` and `minutes`.
- Give back `"{days}d {hours}h"`, `"{hours}h {minutes}m"` or `"{minutes}m"`, by size.
- Save.
