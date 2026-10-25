A stack of logs should never hold -3 items: that is a dupe bug waiting to happen. But with the field open, any mod code that has the stack can write `stack.count = -3;` and nothing stops it. The class needs a way to say "only my own methods touch this".

**Lock it.** Put **`private`** in front of a field: `private int count;`. Now only code inside the `ItemStack` class can read or change `count`. Anywhere else, the compiler refuses:

```text
error: count has private access in ItemStack
```

**Go through the door.** Outside code now has to ask, through the class's methods. Those methods are **`public`**, which means any code that has the object may call them. That is the word you have been copying in `public static` since your first quest.

**Why it works.** If `shrink` is the only way to remove items, then `shrink` is the only place that has to check the rule. One check, in one place, and the count can never go wrong.

**Check, then change.** A method that guards a field tests first and changes the field only when the test passes. Here a furnace burns fuel only when it has enough:

```java
public class Furnace {
    private int fuel;

    public boolean burn(int ticks) {
        if (ticks > fuel) {
            return false;               // refused: fuel stays as it was
        }
        fuel = fuel - ticks;
        return true;
    }
}
```

## Your turn

- Make the `count` field `private`.
- In `shrink`, remove the items and hand back `true` only when the stack has enough. Otherwise leave `count` alone and hand back `false`.
- Save. Both objectives turn green.
