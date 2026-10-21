A program that counts things, like orders or bytes, fails in an ugly way when the number gets too big. Java does not crash: it carries on with a wrong answer.

**The ceiling.** An `int` has room for numbers up to 2147483647. Java calls that `Integer.MAX_VALUE`. The lowest an `int` can go is `Integer.MIN_VALUE`, which is -2147483648.

**Past the top.** Add 1 to the highest `int` and you do not get a bigger number. The count **wraps** around to the very bottom of the range. There is no error and no warning.

**See it.** Two lines are enough:

```java
int top = Integer.MAX_VALUE;
int next = top + 1;   // -2147483648
```

**How you spot it.** A total that suddenly goes negative is the classic sign. If a sum that should only grow comes back negative, an `int` ran out of room.

**What the checker wants.** It adds the highest `int` and 1 and expects the wrapped answer. So today you just add, and see for yourself that the wrap is real.

## Your turn

- In `addInts`, return the two numbers added together.
- Save. The objective turns green because the sum wraps, exactly as the checker expects.
