A stack with no item is nonsense, and so is a stack of 100 when the game caps stacks at 64. With only fields, `new ItemStack()` gives you exactly that: an item of `null` and whatever count someone remembers to set later. You want every stack to be complete the moment it exists.

**Steps inside new.** A **constructor** is a block of steps that runs once, inside `new`, while the object is being built. It is where the object gets its first values, so nobody can forget them.

**How to spot one.** A constructor looks like a method with two differences: its name is the class's name, and it has no return type, not even `void`. Its parameters are what `new` must be given: `new ItemStack("oak_log", 17)`.

**Build it valid.** Because every stack passes through the constructor, the constructor is the one place to enforce a rule like "never more than 64". After it finishes, the object is already in a good state.

**You had one already.** When a class has no constructor, Java quietly adds an empty one with no parameters. That is why `new ItemStack()` worked two quests ago. Write your own, and that free one goes away.

**A constructor in code.** This potion takes its effect and length from `new`, and refuses a negative length:

```java
public class Potion {
    private String effect;
    private int seconds;

    public Potion(String name, int length) {   // runs inside new Potion(...)
        effect = name;
        if (length < 0) {
            seconds = 0;
        } else {
            seconds = length;
        }
    }
}
```

## Your turn

- In the `ItemStack` constructor, store `name` in `item`.
- Store `amount` in `count`, but cap it at 64 when `amount` is above 64.
- Save. Both objectives turn green.
