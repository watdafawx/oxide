The bot adds up player counts all day. An `int` has a ceiling, and when a sum goes past it Java does not warn you: it quietly gives a wrong answer.

**The ceiling.** An `int` has room for numbers up to 2147483647. Java calls that `Integer.MAX_VALUE`. The lowest an `int` can go is `Integer.MIN_VALUE`, which is -2147483648.

**Past the top.** Add 1 to the highest `int` and you do not get a bigger number. The count **wraps** around to the very bottom of the range. There is no error and no warning.

**See it.** Two lines are enough:

```java
int top = Integer.MAX_VALUE;
int next = top + 1;   // -2147483648
```

**How you spot it.** A player count that suddenly goes negative is the classic sign. If a sum that should only grow comes back negative, an `int` ran out of room.

**What the checker wants.** It adds the highest `int` and 1 and expects the wrapped answer. So today you just add, and see for yourself that the wrap is real.

## Your turn

- In `addPlayers`, return the two counts added together.
- Save. The objective turns green because the sum wraps, exactly as the checker expects.
