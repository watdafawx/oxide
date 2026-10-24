An inventory slot holds a stack: 17 oak logs. Pick one more up and it is 18. A method alone cannot do that, because every method you have written forgot everything the moment it returned. Today's code keeps a number alive between calls.

**A variable that stays.** You want a stack that still says 17 after you looked away. That needs a **field**: a variable written inside the class but outside any method. It lives as long as the thing it belongs to, not just for one call.

**The plan and the thing.** A **class** is the plan: "an item stack has a number called `count` and can `grow`". An **object** is one real stack built from that plan. `new ItemStack()` builds one, and a number field starts at `0`.

**Each one is separate.** Build two stacks, one in slot 1 and one in slot 2. Each object has its own `count`, so growing one leaves the other at 0.

**No static today.** `grow()` changes one particular stack, so it is written without `static` and called on an object, with a dot: `logs.grow()`. Why every earlier method had `static` gets its own quest later in this world.

**Fields in code.** Here is a quiver of arrows. The field sits at the top, and the method changes it:

```java
public class Quiver {
    int arrows;                // a field: every Quiver has its own

    public void refill() {     // no static: works on one Quiver
        arrows++;
    }
}
```

**The TODO line.** From here on, an unfinished method in the starter holds `throw new UnsupportedOperationException("TODO: ...")`. It is a placeholder that stops the method with an error, so the checker shows red. Replace the whole line with your code.

## Your turn

- In `count`, replace the `throw` line so it hands back the `count` field.
- In `grow`, add one to `count`.
- Save. Both objectives turn green.
