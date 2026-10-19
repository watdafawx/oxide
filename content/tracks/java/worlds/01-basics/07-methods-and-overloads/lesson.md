Methods are how you give a piece of logic a name and reuse it. Java also lets several methods share one name when their parameters differ: that is **overloading**, and you already use it every time you call `println`.

```java
static int area(int side)          { return side * side; }
static int area(int w, int h)      { return w * h; }
static double area(double radius)  { return Math.PI * radius * radius; }

area(3);        // 9      -> area(int)
area(3, 4);     // 12     -> area(int, int)
area(2.0);      // 12.56  -> area(double)
```

- Java picks the overload from the number and **types** of arguments. The return type alone cannot tell two overloads apart.
- **Varargs** (`int... nums`) means "zero or more ints". Inside the method `nums` is just an `int[]`. The caller writes `sum(1, 2, 3)` or `sum()`.
- Varargs must be the last parameter, and there can be only one.

## Your turn

- Open `src/main/java/oxide/methods_and_overloads/Shapes.java`.
- Write the three `area` overloads.
- Write `sum(int... nums)`.
