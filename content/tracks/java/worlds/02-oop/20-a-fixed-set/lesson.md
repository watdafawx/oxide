A traffic light is RED, GREEN or YELLOW, never "blue" and never a typo like `"gren"`. Stored as a `String`, every typo compiles and turns up as a bug on the road. You want a type with exactly three values, and nothing else.

**A fixed set.** An **enum** lists every value a type can have: `public enum TrafficLight { RED, GREEN, YELLOW; }`. Those are its **constants**, written in capitals. `TrafficLight.GREEN` is one of them, and `TrafficLight.BLUE` does not compile.

**One of each.** Each constant exists exactly once, so `==` is the right way to compare them, unlike text. Inside the enum's own methods, `this` is the current constant, and `this == RED` asks "am I red?".

**Switch over it.** A switch expression from world 01 works on an enum, one case per constant:

```java
String label = switch (this) {
    case MERCURY -> "hot";
    case VENUS -> "hotter";
    case EARTH -> "just right";
};
```

**No default needed.** The compiler knows every constant, so a switch that covers them all needs no `default`. Add a fourth light one day and every switch that forgets it stops compiling, which is exactly where you want to find out.

**Enums are classes.** The constants are objects, so an enum can have methods like `next()`, and the next quest gives them fields too.

## Your turn

- In `mayDrive`, hand back whether this light is `GREEN`.
- In `next`, switch over `this`: RED goes to GREEN, GREEN to YELLOW, YELLOW to RED.
- Save. Both objectives turn green.
