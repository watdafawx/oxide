The server sends a player three commands: heal, hurt, and respawn at full health. Each command is a record of a sealed `Command`, so a switch over the kind fits. But `apply` changes the player and hands nothing back, and every switch so far handed back a value.

**A switch statement.** Leave out `return` and the `;` after the switch, and it becomes a **switch statement**: each arm does something instead of producing a value. `case Heal h -> health += h.amount();` raises health when the command is a `Heal`.

**A block for more steps.** An arm may hold a block in braces, `case Respawn r -> { ... }`, with as many lines as it needs. Respawn does two things, restore health and count the respawn, so its arm is a block:

```java
switch (action) {
    case Refuel r -> fuel += r.ticks();
    case Extinguish e -> {
        lit = false;
        timesPutOut++;
    }
}
```

**Take away.** `health -= 7` is short for `health = health - 7`, the partner of the `+=` you know.

**Still counted.** A switch statement over a sealed interface must cover every kind too. Leave out `Respawn` and the compiler stops you, exactly like before.

## Your turn

- In `apply`, write a switch statement over `command`.
- `Heal` adds its amount to `health`, `Hurt` takes its amount away.
- `Respawn` sets `health` to 20 and adds one to `respawns`. Save. Both objectives turn green.
