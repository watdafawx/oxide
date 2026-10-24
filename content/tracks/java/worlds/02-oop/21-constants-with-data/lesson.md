Each light stays on for its own time: red 30 seconds, green 25, yellow 5. That number belongs to the light, so it should live in the enum, not in a separate table somewhere that can drift out of step.

**Constants with values.** Since constants are objects, an enum can have fields and a constructor. Each constant passes its values in brackets, and that calls the constructor once for it:

```java
public enum Planet {
    MERCURY(3), VENUS(12), EARTH(13);

    private final int size;

    Planet(int size) {
        this.size = size;
    }
}
```

**The shape.** The constants come first and end with `;`. Fields, constructor and methods follow. The constructor is private by nature: nobody can build a fourth light with `new`, which is the point of an enum.

**Every constant.** `TrafficLight.values()` is a static method every enum gets. It hands back an array of all constants, in the order they are written, ready for an enhanced `for`.

**For the curious.** `TrafficLight.valueOf("GREEN")` finds a constant by its exact name and throws an error for any other text, handy for reading config files. `GREEN.ordinal()` is its position, 1.

## Your turn

- Give each constant its real seconds: RED 30, GREEN 25, YELLOW 5.
- In `fullCycle`, loop over `values()` and add up every light's `seconds()`.
- Save. Both objectives turn green.
