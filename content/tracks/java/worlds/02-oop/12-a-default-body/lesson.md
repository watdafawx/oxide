The planner wants a one-line label for every shape: `Rectangle with area 12`. You could write that method in `Circle`, again in `Rectangle`, and again in every shape to come. It is the same line each time, and it only needs things every shape already promises.

**A body in the promise.** A method in an interface marked **`default`** has a body. Every class that implements the interface gets it for free, without writing a line.

**It can use the promise.** Inside a default method you may call the interface's other methods, like `name()` and `area()`. You do not know which class you are in, but you know it kept the promise, so those calls always work:

```java
public interface Priced {
    int cents();

    default String label() {
        return cents() / 100 + " euro";
    }
}
```

**Rounding.** `Math.round(12.57)` rounds a decimal to the nearest whole number, 13. It is the helper the `// TODO` names.

**Still yours to change.** A class may write its own `describe()` with `@Override`, and then its version wins. Most do not need to, and that is the point.

## Your turn

- In `Shape.describe()`, hand back the name, then `" with area "`, then the area rounded with `Math.round`.
- Save. Both objectives turn green.
