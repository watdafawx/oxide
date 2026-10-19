**Why this matters:** a drawing app has circles, rectangles, later stars. Code that draws or measures them should not care which one it holds, only that it can ask for `area()`. An **interface** is that promise: "anything that is a `Shape` can do these things".

```java
public interface Pet {
    String name();
    int legs();

    default String describe() {                 // has a body: implementations inherit it
        return name() + " has " + legs() + " legs";
    }
}

public class Bird implements Pet {
    public String name() { return "Bird"; }
    public int legs() { return 2; }
}

Pet p = new Bird();
p.describe();      // "Bird has 2 legs"
```

- `interface` lists methods without bodies. Classes `implements` it and fill every one in.
- A **default method** has a body, so you write shared behaviour once. It can call the interface's other methods.
- A variable can have the interface as its type: `Pet p = new Bird();`.

## Your turn

- `Shape.java` has `name()`, `area()`, `perimeter()` and an empty `describe()`. Write the default body of `describe()`.
- Fill in `Circle` and `Rectangle` (name, area, perimeter).
