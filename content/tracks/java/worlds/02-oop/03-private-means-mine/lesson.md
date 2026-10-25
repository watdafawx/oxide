A piggy bank should never hold -3 coins. But with the field open, any code that has the bank can write `bank.coins = -3;` and nothing stops it. The class needs a way to say "only my own methods touch this".

**Lock it.** Put **`private`** in front of a field: `private int coins;`. Now only code inside the `PiggyBank` class can read or change `coins`. Anywhere else, the compiler refuses:

```text
error: coins has private access in PiggyBank
```

**Go through the door.** Outside code now has to ask, through the class's methods. Those methods are **`public`**, which means any code that has the object may call them. That is the word you have been copying in `public static` since your first quest.

**Why it works.** If `take` is the only way to remove coins, then `take` is the only place that has to check the rule. One check, in one place, and the count can never go wrong.

**Check, then change.** A method that guards a field tests first and changes the field only when the test passes. Here a door lets in at most 30 people:

```java
public class Door {
    private int inside;

    public boolean letIn(int people) {
        if (inside + people > 30) {
            return false;               // refused: inside stays as it was
        }
        inside = inside + people;
        return true;
    }
}
```

## Your turn

- Make the `coins` field `private`.
- In `take`, take the coins and hand back `true` only when there are enough. Otherwise leave `coins` alone and hand back `false`.
- Save. Both objectives turn green.
