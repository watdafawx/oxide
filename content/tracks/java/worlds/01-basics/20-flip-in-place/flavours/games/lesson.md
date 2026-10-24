The samples are stored oldest first, but the dashboard wants the newest first. Instead of building a second array, the bot can flip the one it was given.

**Arrays are shared.** When you pass an array to a method, the method gets the very same array, not a copy. If it changes a value, the caller sees the change.

**Nothing to hand back.** A method that changes things and returns nothing has the return type **`void`**. It has no `return value;` line at all.

**A swap needs a third hand.** To swap two values you need a spare variable, because writing the first over the second would lose the second.

**Swap two positions.** Save one value in `tmp`, overwrite it, then use `tmp`:

```java
int tmp = samples[0];
samples[0] = samples[1];
samples[1] = tmp;
```

**Meet in the middle.** To reverse, swap position `i` with position `length - 1 - i`, for each `i` in the first half. Go all the way and you would swap everything back again.

**If this is new.** Programming 101 explains two names for one thing from zero: [Two names, one table](/quest/basics/07-how-the-computer-holds-things/02-two-names-one-table).

## Your turn

- In `swapEnds`, swap the first and the last value of `samples`.
- In `newestFirst`, reverse the whole array: swap the first with the last, the second with the second to last, and so on up to the middle.
- Save. Both objectives turn green.
