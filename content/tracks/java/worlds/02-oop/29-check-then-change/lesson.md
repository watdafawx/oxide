Ann has 10 euros and tries to pay Bo 50. The payment must be refused, and afterwards both wallets must hold exactly what they held before. A method that changes something first and checks second has already done damage by the time it says no.

**An exception of your own.** The starter has `NotEnoughMoneyException`. An exception is a class like any other: this one extends **`RuntimeException`**, Java's parent for errors a method may throw, and its constructor hands the message up with `super(message)`. Its own name tells the caller exactly what went wrong.

**Throw before you touch.** `throw` stops the method on the spot, so every line after it is skipped. Put the check and the `throw` first, and the change after it: a refused spend never reaches the line that changes `cents`:

```java
public void use(int charges) {
    if (charges > left) {
        throw new OutOfChargesException("only " + left + " left");
    }
    left -= charges;
}
```

**Two objects, same rule.** A payment changes two wallets. The starter's `pay` gives Bo the money first, then calls `spend`. When `spend` throws, Bo already has 50 euros that never left Ann: money from nowhere.

**Order is the fix.** Do the step that can fail first. `spend(amount)` either throws, and nothing has changed anywhere, or succeeds, and then giving the money to `to` cannot fail.

**Another wallet's field.** `to.cents` is private, yet `pay` may change it: private means "only code in the Wallet class", and `pay` is in it.

## Your turn

- In `spend`, throw a `NotEnoughMoneyException` when `amount` is more than `cents`, before anything changes.
- In `pay`, put the two lines in the order that keeps a refused payment from changing either wallet.
- Save. Both objectives turn green.
