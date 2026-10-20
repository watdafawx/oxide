**Why now:** the status bot reports "uptime this month: 99.2%", and that needs the length of the month in seconds, leap February included. It also colours the ping: green, yellow, red. Both are "which of many cases is this?", which a **switch expression** answers without a pile of `if`s.

```java
String name = switch (day) {
    case 1, 7 -> "weekend";          // several labels, one arm
    case 2, 3, 4, 5, 6 -> "weekday";
    default -> throw new IllegalArgumentException("day " + day);
};
```

- `->` arms do **not** fall through. No `break`, none of the old `case X:` bugs.
- The whole `switch` has a value, so you can `return` it or assign it.
- It must be **exhaustive**: `default` handles everything else. Throwing in `default` makes bad input loud.
- Tip: switch on a computed bucket (`pingMs / 50`) to turn ranges into single cases.

## Your turn

- Open `src/main/java/oxide/switch_expressions/Uptime.java`.
- `daysInMonth(month, year)`: 31/30/28/29. Use the given `isLeap` helper for February. A month outside 1-12 must throw `IllegalArgumentException`, like the `default` arm above.
- `pingGrade(pingMs)`: under 50 ms "great", 50-99 "good", 100-199 "ok", 200 and over "poor".
