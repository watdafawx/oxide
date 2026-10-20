**Why now:** the status bot has two "for every one of them" jobs. After a restart the server fills up one more player each minute, and the bot totals the **player-minutes**. And MOTDs arrive with Minecraft color codes (`§a`, `§l`); before cleaning one the bot counts how many there are. Both are loops.

```java
int total = 0;
for (int m = 1; m <= 5; m++) {      // counted loop: start; keep going while; step
    total += m;                     // total = total + m
}

int ticks = 100;
while (ticks > 1) {                 // loop while the condition holds
    ticks = ticks / 2;
}

for (char c : "§aHi".toCharArray()) {   // enhanced for: each element, no index
    System.out.println(c);
}
```

- Off-by-one is the classic bug: `m < minutes` stops before the last minute, `m <= minutes` includes it.
- A `char` is one character in **single quotes**: `'a'`. Compare it with `==`: `c == 'a'` is true when `c` is the letter a.
- Use enhanced `for` when you do not need the index; counted `for` when you do.
- `motd.toCharArray()` gives the characters. The section sign is written `'\u00a7'` in source code so it survives any file encoding.

## Your turn

- Open `src/main/java/oxide/loops/StatusLoops.java`.
- `playerMinutes(minutes)`: add `1 + 2 + ... + minutes`.
- `countColorCodes(motd)`: count how many `'\u00a7'` characters the MOTD has.
