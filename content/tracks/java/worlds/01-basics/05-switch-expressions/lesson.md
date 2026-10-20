When a value depends on which of many cases you are in (month, command, status code), a chain of `if`s gets noisy. A **switch expression** picks the value directly, and the compiler checks you covered every case.

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
- Tip: switch on a computed bucket (`score / 10`) to turn ranges into single cases.

## Your turn

- Open `src/main/java/oxide/switch_expressions/Rules.java`.
- `daysInMonth(month, year)`: 31/30/28/29. Use the given `isLeap` helper for February. A month outside 1-12 must throw `IllegalArgumentException`, like the `default` arm above.
- `gradeFor(score)`: 90-100 "A", 80s "B", 70s "C", 60s "D", otherwise "F".
