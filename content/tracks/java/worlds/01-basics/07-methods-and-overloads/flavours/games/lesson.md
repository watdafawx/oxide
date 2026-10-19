**Why now:** the status line is built by calling **smaller methods**: one for the player count, one for the online flag, one for the MOTD. Giving them all the name `show` keeps the call sites readable, and a network of several servers needs a total over *however many* servers there are.

```java
static String show(int playerCount) { return playerCount + " online"; }
static String show(boolean online)  { return online ? "ONLINE" : "OFFLINE"; }
static String show(String motd)     { return "MOTD: " + motd; }

show(5);        // "5 online"  -> show(int)
show(true);     // "ONLINE"    -> show(boolean)
show("Hi");     // "MOTD: Hi"  -> show(String)
```

- This is **overloading**: Java picks the method from the number and **types** of the arguments. The return type alone cannot tell two overloads apart.
- **Varargs** (`int... perServer`) means "zero or more ints". Inside the method it is just an `int[]`. The caller writes `totalPlayers(3, 8, 0)` or `totalPlayers()`.
- Varargs must be the last parameter, and there can be only one.

## Your turn

- Open `src/main/java/oxide/methods_and_overloads/StatusFormat.java`.
- Write the three `show` overloads.
- Write `totalPlayers(int... perServer)`.
