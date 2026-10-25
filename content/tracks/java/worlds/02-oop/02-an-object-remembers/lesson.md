A shop door counts visitors: every time it opens, one more. A method alone cannot do that, because every method you have written forgot everything the moment it returned. Today's code keeps a number alive between calls.

**A variable that stays.** You want a counter that is still 2 after it was pressed twice. That needs a **field**: a variable written inside the class but outside any method. It lives as long as the thing it belongs to, not just for one call.

**The plan and the thing.** A **class** is the plan: "a counter has a number called `count` and can `increment` it". An **object** is one real counter built from that plan. `new Counter()` builds one, and a number field starts at `0`.

**Each one is separate.** Build two counters, one for the front door and one for the back door. Each object has its own `count`, so pressing one leaves the other at 0.

**No static today.** `increment()` changes one particular counter, so it is written without `static` and called on an object, with a dot: `front.increment()`. Why every earlier method had `static` gets its own quest later in this world.

**Fields in code.** Here is a lap counter. The field sits at the top, and the method changes it:

```java
public class LapCounter {
    int laps;                  // a field: every LapCounter has its own

    public void lap() {        // no static: works on one LapCounter
        laps++;
    }
}
```

**The TODO line.** From here on, an unfinished method in the starter holds `throw new UnsupportedOperationException("TODO: ...")`. It is a placeholder that stops the method with an error, so the checker shows red. Replace the whole line with your code.

## Your turn

- In `value`, replace the `throw` line so it hands back the `count` field.
- In `increment`, add one to `count`.
- Save. Both objectives turn green.
