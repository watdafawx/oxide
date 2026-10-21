The bot should total the players of however many servers you give it: one, three, eight. Writing a different method for every count would be silly.

**Any number of values.** Three dots after a type, like `int... nums`, mean "zero or more ints". That is a **varargs** parameter.

**One header, many calls.** The caller just lists the values, however many:

```java
static void list(String... names) { }

list();                  // no names
list("Ada");             // one
list("Ada", "Bo");       // two
```

**Inside, it is an array.** In the method, `perServer` is an ordinary `int[]`. So `perServer.length` and the enhanced `for` work exactly as they did in the last quests.

**One rule.** A varargs parameter must be the last one in the list, and a method can have only one. If someone already has an array, they can pass the array itself.

## Your turn

- In `serverCount`, return how many numbers were passed in.
- In `totalPlayers`, return the sum of all of them. Save and both objectives turn green.
