**Why this matters:** a traffic light is RED, GREEN or YELLOW, never "blue" or a typo like `"gren"`. When a value must come from a fixed list, a `String` allows every typo. An **enum** allows only its constants, and the compiler checks every use.

Enums are classes: they can have fields, a constructor and methods.

```java
public enum Planet {
    MERCURY(3), VENUS(12), EARTH(13);          // each constant calls the constructor

    private final int diameterThousandKm;

    Planet(int diameterThousandKm) {            // enum constructors are always private
        this.diameterThousandKm = diameterThousandKm;
    }

    public int diameter() { return diameterThousandKm; }
}

Planet.EARTH.diameter();                        // 13
Planet.valueOf("VENUS");                        // exact name; unknown name -> IllegalArgumentException
Planet.values();                                // all constants, in order
Planet.EARTH.ordinal();                         // 2: position in the list

String label = switch (planet) {                // switch over an enum: no default needed
    case MERCURY -> "hot";
    case VENUS -> "hotter";
    case EARTH -> "just right";
};
```

- Constants are written in UPPER_CASE and end with `;` before the fields.
- `valueOf` is case-sensitive: `"earth"` fails. To accept any case, clean the text first.
- A `switch` expression over an enum must cover every constant, or the compiler stops you.
- Inside an enum method you can switch over the enum itself: `switch (this) { ... }`.
- To ignore case use `text.toUpperCase(Locale.ROOT)` (`Locale` needs `import java.util.Locale;`, already in the starter).

## Your turn

- Open `src/main/java/oxide/enums/TrafficLight.java`.
- Give each constant its real seconds (RED 30, GREEN 25, YELLOW 5).
- Write `parse` and `next`.
