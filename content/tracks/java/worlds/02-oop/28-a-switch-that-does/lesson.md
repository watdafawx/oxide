A thermostat takes three commands: warmer, cooler, and reset to the factory 20 degrees. Each command is a record of a sealed `Adjust`, so a switch over the kind fits. But `apply` changes the thermostat and hands nothing back, and every switch so far handed back a value.

**A switch statement.** Leave out `return` and the `;` after the switch, and it becomes a **switch statement**: each arm does something instead of producing a value. `case Warmer w -> target += w.degrees();` raises the target when the command is a `Warmer`.

**A block for more steps.** An arm may hold a block in braces, `case Reset r -> { ... }`, with as many lines as it needs. Reset does two things, set the target and count the reset, so its arm is a block:

```java
switch (order) {
    case Refill r -> cups += r.cups();
    case Close c -> {
        open = false;
        closings++;
    }
}
```

**Take away.** `target -= 5` is short for `target = target - 5`, the partner of the `+=` you know.

**Still counted.** A switch statement over a sealed interface must cover every kind too. Leave out `Reset` and the compiler stops you, exactly like before.

## Your turn

- In `apply`, write a switch statement over `adjust`.
- `Warmer` adds its degrees to `target`, `Cooler` takes its degrees away.
- `Reset` sets `target` to 20 and adds one to `resets`. Save. Both objectives turn green.
