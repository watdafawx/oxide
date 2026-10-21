You keep halving a number until it is small enough. You do not know in advance how many rounds that takes, so a counted `for` loop is the wrong tool.

**Loop until.** `while (condition) { ... }` repeats the block for as long as the condition is true. Java checks the condition before every round, including the first.

**It can run zero times.** If the condition is already false on the first check, the block never runs. That is often exactly what you want.

**The pattern.** Change a value inside the loop, and let the condition watch that value:

```java
int ping = 800;
while (ping > 100) {
    ping = ping / 2;   // 400, 200, 100
}
```

**Change something.** If the block never changes what the condition looks at, the loop never ends: an **infinite loop**. When a run hangs, a `while` that cannot finish is the first thing to check.

**Which loop to pick.** Use `for` when you know how many rounds you want. Use `while` when you only know when to stop. Keep a counter variable if you also need to know how many rounds it took.

## Your turn

- In `halvings`, count how many times `start` can be halved (use `/ 2`) before it is no longer above `limit`.
- Save. Both objectives turn green, including the case that needs zero rounds.
