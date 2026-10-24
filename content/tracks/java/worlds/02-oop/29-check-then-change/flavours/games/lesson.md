Alex has 10 XP levels and tries to give Steve 50. The gift must be refused, and afterwards both players must have exactly what they had before. A method that changes something first and checks second has already done damage by the time it says no, and in a mod that damage is a dupe.

**An exception of your own.** The starter has `NotEnoughLevelsException`. An exception is a class like any other: this one extends **`RuntimeException`**, Java's parent for errors a method may throw, and its constructor hands the message up with `super(message)`. Its own name tells the caller exactly what went wrong.

**Throw before you touch.** `throw` stops the method on the spot, so every line after it is skipped. Put the check and the `throw` first, and the change after it: a refused enchant never reaches the line that changes `levels`:

```java
public void use(int charges) {
    if (charges > left) {
        throw new OutOfChargesException("only " + left + " left");
    }
    left -= charges;
}
```

**Two objects, same rule.** A gift changes two players. The starter's `giveLevels` gives Steve the levels first, then calls `spendLevels`. When that throws, Steve already has 50 levels that never left Alex: levels from nowhere.

**Order is the fix.** Do the step that can fail first. `spendLevels(amount)` either throws, and nothing has changed anywhere, or succeeds, and then giving the levels to `to` cannot fail.

**Another player's field.** `to.levels` is private, yet `giveLevels` may change it: private means "only code in the Player class", and `giveLevels` is in it.

## Your turn

- In `spendLevels`, throw a `NotEnoughLevelsException` when `amount` is more than `levels`, before anything changes.
- In `giveLevels`, put the two lines in the order that keeps a refused gift from changing either player.
- Save. Both objectives turn green.
