**Why now:** a flaky server query can report `-1` players or `300` on a 20-slot server. Your status line must never show that, so the bot **clamps** the count into `0..maxPlayers`. It also adds a ▲ / ▬ / ▼ trend, which is a decision on the sign of "players now minus players a minute ago".

```java
if (delta < 0) {
    return "leaving";
} else if (delta == 0) {
    return "steady";
} else {
    return "joining";
}

String label = playerCount == 1 ? "player" : "players";   // ternary: an if that has a value
```

- Conditions are `boolean`: `<`, `<=`, `==`, `!=`, and `&&` (and), `||` (or), `!` (not).
- `&&` and `||` **short-circuit**: in `motd != null && motd.length() > 0` the right side never runs when the left is false. That is how a missing MOTD does not crash the bot.
- Ternary `cond ? a : b` is for one small choice; use `if` for anything longer.

## Your turn

- Open `src/main/java/oxide/if_and_ternary/PlayerCount.java`.
- `clampPlayerCount(playerCount, min, max)`: force it into `min..max`.
- `trend(delta)`: return `-1`, `0` or `1`.
