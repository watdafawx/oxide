**Why this matters:** every program remembers things: a score, a click count, a battery level. In Java that memory lives in a **class**: a blueprint with **fields** (the data) and **methods** (what you can do with it).

A `Counter` that remembers how many times a button was pressed:

```java
public class Clicker {
    private int clicks;              // field: starts at 0 by default

    public int total() {             // method that reads the field
        return clicks;
    }

    public void press() {            // method that changes the field
        clicks++;
    }
}

Clicker a = new Clicker();
Clicker b = new Clicker();
a.press();
a.press();
// a.total() == 2, b.total() == 0: every object has its own `clicks`
```

- `new Clicker()` builds an object. Fields of numbers start at `0`.
- `private` keeps the field inside the class: other code must go through methods.
- `void` means the method gives nothing back.

## Your turn

- Open `src/main/java/oxide/classes_and_fields/Counter.java`.
- Make `value()` return the `count` field.
- Make `increment()` add one to `count`.
