You want a single name, `area`, for the area of a square, a rectangle and a circle. The three need different inputs, and Java lets one name cover all of them.

**One name, many jobs.** Java lets several methods share a name, as long as their parameters are different. That is called **overloading**.

**Java picks by input.** When you call the name, Java looks at how many arguments you passed and what types they have, and runs the method that fits. `label(3)` and `label(true)` call different methods:

**Two methods, one name.** Same name, different parameter type, so Java can tell them apart:

```java
public static String label(int n)      { return String.format("%d items", n); }
public static String label(boolean on) { return on ? "on" : "off"; }
```

**What must differ.** The parameter list: a different number of parameters, or different types. The return type alone cannot tell two methods apart, because the call does not say what you will do with the result.

**You already use it.** `System.out.println` works for text, numbers and booleans because it is overloaded in exactly this way.

## Your turn

- In `area(int side)`, return the area of a square.
- In `area(int width, int height)`, return the area of a rectangle.
- In `area(double radius)`, return the area of a circle. Save and the objective turns green.
