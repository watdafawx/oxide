A score has to become a letter grade, and a calendar needs the number of days in each month. Long chains of `if` get noisy fast. A `switch` handles many cases in one tidy block.

**Pick by value.** A **switch expression** looks at one value and picks the matching **arm**. The whole `switch` has a value, so you can `return` it or store it.

**Arrows.** Each arm is a label, an arrow and a result: `case 1 -> "mon";`. Only the matching arm runs. There is no falling through to the next one.

**Sharing an arm.** Several labels can share one result, separated by commas: `case 6, 7 -> "weekend";`.

**Cover everything.** The `switch` must have an answer for every possible value. `default` is the arm for everything else. Throwing an error in `default` makes bad input loud instead of silent:

**All together.** One value in, one result out:

```java
String kind = switch (day) {
    case 6, 7 -> "weekend";
    case 1, 2, 3, 4, 5 -> "weekday";
    default -> throw new IllegalArgumentException("day " + day);
};
```

**Buckets.** To turn a range into one case, divide first. `score / 10` turns 90 to 99 into 9, 80 to 89 into 8, and so on.

## Your turn

- In `gradeFor`, add the missing arms: `10, 9` is A, `8` B, `7` C, `6` D, everything else F.
- In `daysInMonth`, add arms for the months with 31 days, with 30 days, and February (29 in a leap year, else 28). `isLeap` is already written for you.
- Save. Both objectives turn green.
