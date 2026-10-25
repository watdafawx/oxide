Most reminders are typed in a hurry: just a text, or nothing at all. So `Reminder` gets three constructors: `new Reminder("Call mum", 5)`, `new Reminder("Call mum")` and `new Reminder()`. The full one already trims the text and keeps the time at 1 minute or more. The short ones must not skip those rules.

**Same name, different inputs.** Several constructors with different parameters is **overloading**, the same idea as `show(int)` and `show(boolean)` in world 01. Java picks the one whose parameters match what you pass to `new`.

**Hand it over.** Copying the rules into every constructor is how the copies drift apart: one day the short one forgets to trim. Instead, a short constructor calls another one of the same class with **`this(...)`**, filling in a default:

```java
public Timer(String name, int seconds) {   // the one with the rules
    this.name = name.toUpperCase();
    this.seconds = Math.max(0, seconds);  // the larger of 0 and seconds
}

public Timer(String name) {
    this(name, 60);                        // same rules, 60 by default
}
```

**First line only.** `this(...)` has to be the very first line of the constructor, because the object must be set up by the full constructor before anything else touches it. So a short constructor is usually that one line and nothing more.

**Chains are fine.** `Timer()` can say `this("TIMER");`, which calls `Timer(String)`, which calls the full one. Every path ends in the one place with the rules.

## Your turn

- Replace the body of `Reminder(String text)` with one `this(...)` line: the full constructor, 10 minutes.
- Replace the body of `Reminder()` the same way, with the text `"Untitled"`.
- Save. Both objectives turn green.
