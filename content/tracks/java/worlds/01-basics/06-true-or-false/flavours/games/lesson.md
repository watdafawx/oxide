The bot answers yes/no questions all day: is the server full, may this player join? A yes/no answer has its own type.

**Yes or no.** A **`boolean`** holds one of two values: `true` or `false`. A comparison makes one: `5 > 3` is `true`.

**The comparisons.** `<` less than, `>` greater than, `<=` and `>=` with "or equal", `==` equal, `!=` not equal. Careful: one `=` stores a value, two `==` ask whether two numbers are equal.

**And, or, not.** `&&` means both must be true. `||` means at least one is true. `!` flips true to false. So `!banned` is true for someone who is not banned.

**Put them together.** A method that answers a yes/no question has the return type `boolean` and returns the expression directly:

```java
boolean full = online >= max;
boolean canJoin = !full && !banned;
```

**Text is different.** `==` is for numbers. Comparing text needs another tool, which a later quest explains.

## Your turn

- In `isFull`, return whether `online` has reached `max`.
- In `canJoin`, return true only when the server is not full and the player is not banned.
- Save. Both objectives turn green.
