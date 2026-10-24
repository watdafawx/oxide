A bank account without an owner is nonsense, and so is one that opens at -200. With only fields, `new Account()` gives you exactly that: an owner of `null` and whatever balance someone remembers to set later. You want every account to be complete the moment it exists.

**Steps inside new.** A **constructor** is a block of steps that runs once, inside `new`, while the object is being built. It is where the object gets its first values, so nobody can forget them.

**How to spot one.** A constructor looks like a method with two differences: its name is the class's name, and it has no return type, not even `void`. Its parameters are what `new` must be given: `new Account("Ann", 500)`.

**Build it valid.** Because every account passes through the constructor, the constructor is the one place to enforce a rule like "never open below 0". After it finishes, the object is already in a good state.

**You had one already.** When a class has no constructor, Java quietly adds an empty one with no parameters. That is why `new Counter()` worked two quests ago. Write your own, and that free one goes away.

**A constructor in code.** This timer takes its label and length from `new`, and refuses a negative length:

```java
public class Timer {
    private String label;
    private int seconds;

    public Timer(String name, int length) {   // runs inside new Timer(...)
        label = name;
        if (length < 0) {
            seconds = 0;
        } else {
            seconds = length;
        }
    }
}
```

## Your turn

- In the `Account` constructor, store `name` in `owner`.
- Store `opening` in `balance`, but start at 0 when `opening` is below 0.
- Save. Both objectives turn green.
