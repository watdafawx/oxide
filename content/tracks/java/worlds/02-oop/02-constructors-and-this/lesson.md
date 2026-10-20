**Why this matters:** objects are easier to use when you can build them the short way (`new Reminder("Call mum")`) and the long way. But if each constructor repeats the rules ("trim the text, at least one minute"), the copies drift apart.

A class can have several constructors with different parameters (**overloading**). `this(...)` as the first line calls another constructor of the same class:

```java
public class Timer {
    private final String name;
    private final int seconds;

    public Timer(String name, int seconds) {     // the one real constructor
        this.name = name.toUpperCase();
        this.seconds = Math.max(0, seconds);
    }

    public Timer(String name) {
        this(name, 60);                           // reuse it: name is upper-cased too
    }

    public Timer() {
        this("TIMER");                            // chains again
    }
}
```

- `this.name` is the field; plain `name` is the parameter. `this.` tells them apart.
- `this(...)` must be the **first** statement of the constructor.
- `final` fields are set once, in a constructor.
- `Math.max(a, b)` is the larger of two numbers, `Math.min(a, b)` the smaller. Nest them to keep a value inside a range.

## Your turn

- Open `src/main/java/oxide/constructors_and_this/Reminder.java`.
- In the two-argument constructor: trim the text and make `minutes` at least 1.
- Make `Reminder(String)` and `Reminder()` call another constructor with `this(...)`: default text `"Untitled"`, default `10` minutes.
