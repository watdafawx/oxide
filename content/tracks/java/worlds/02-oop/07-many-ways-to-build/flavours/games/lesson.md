Your item registry builds items the short way, `new Item("Stick")`, and the long way, `new Item("ender_pearl", 16)`. The full constructor already lower-cases the id and keeps the stack size between 1 and 64. If a short constructor skipped those rules, you would end up with two kinds of stick.

**Same name, different inputs.** Several constructors with different parameters is **overloading**, the same idea as `show(int)` and `show(boolean)` in world 01. Java picks the one whose parameters match what you pass to `new`.

**Hand it over.** Copying the rules into every constructor is how the copies drift apart: one day the short one forgets to lower-case. Instead, a short constructor calls another one of the same class with **`this(...)`**, filling in a default:

```java
public Potion(String effect, int seconds) {   // the one with the rules
    this.effect = effect.toUpperCase();
    this.seconds = Math.max(0, seconds);  // the larger of 0 and seconds
}

public Potion(String effect) {
    this(effect, 180);                        // same rules, 180 by default
}
```

**First line only.** `this(...)` has to be the very first line of the constructor, because the object must be set up by the full constructor before anything else touches it. So a short constructor is usually that one line and nothing more.

**Chains are fine.** `Potion()` can say `this("WATER");`, which calls `Potion(String)`, which calls the full one. Every path ends in the one place with the rules.

## Your turn

- Replace the body of `Item(String id)` with one `this(...)` line: the full constructor, stack size 64.
- Replace the body of `Item()` the same way, with the id `"air"`.
- Save. Both objectives turn green.
