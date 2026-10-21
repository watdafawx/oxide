You have 17 items and every box holds 5. How many full boxes is that, and how many items are left over? Division answers both, but not the way a calculator would.

**Think in groups.** 17 split into groups of 5 is 3 full boxes with 2 left over. Java gives you one operator for each half of that answer.

**Slash counts the groups.** `/` on two `int`s gives a whole number: the fraction is thrown away, not rounded. `17 / 5` is `3`, and so is `19 / 5`.

**Percent counts the rest.** `%` is the **remainder**: what is left after the full groups are taken out. `17 % 5` is `2`. When it divides exactly, the remainder is `0`.

**Both at once.** Each answer is its own whole number:

```java
int groups = 17 / 5;   // 3
int left = 17 % 5;     // 2
```

**One thing to avoid.** Dividing an `int` by `0` stops the program with an error. Today's numbers are never zero.

## Your turn

- In `fullBoxes`, return how many full boxes `items` make at `perBox` each.
- In `itemsLeft`, return how many items are left over.
- Save. Both objectives turn green.
