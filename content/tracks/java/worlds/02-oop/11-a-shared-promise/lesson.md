A floor planner has circles and rectangles, and later stars. The code that adds up floor space should not care which shape it holds, only that it can ask for `area()`. You want a way to say "anything that is a shape can do this", and let the compiler check it.

**A promise with no steps.** An **interface** is a list of method lines without bodies: what a thing can do, not how. `public interface Shape { String name(); double area(); }` says every shape has a name and an area.

**Keep the promise.** A class that writes `implements Shape` promises to give every one of those methods a body. Forget one, and the compiler names it:

```text
error: Circle is not abstract and does not override abstract method area() in Shape
```

**One type, many classes.** A variable can have the interface as its type: `Shape s = new Circle(2);`. Code that takes a `Shape` works with a circle, a rectangle, or a star written next year, because all it uses is the promise.

**In code.** Here a coffee keeps the `Priced` promise. `@Override` above a method says "this fills in a promised method"; the compiler checks the spelling, the way `@Test` marks a test:

```java
public interface Priced {
    int cents();
}

public class Coffee implements Priced {
    @Override
    public int cents() {
        return 320;
    }
}
```

## Your turn

- In `Circle`, fill in `name` (`"Circle"`) and `area` (pi times radius times radius).
- In `Rectangle`, fill in `name` (`"Rectangle"`) and `area` (width times height).
- Save. Both objectives turn green.
