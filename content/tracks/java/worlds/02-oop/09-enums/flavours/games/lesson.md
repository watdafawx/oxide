**Why now:** a sword is COMMON, RARE or EPIC, never "legendari" or a typo in a config file. Item tiers must come from a fixed list; with a `String` every typo compiles. A `Rarity` **enum** allows only its constants, and the compiler checks every use. Name colour and loot weight can ride along as fields.

Enums are classes: they can have fields, a constructor and methods.

```java
public enum Dimension {
    OVERWORLD(1), NETHER(8), END(1);            // each constant calls the constructor

    private final int coordinateScale;

    Dimension(int coordinateScale) {             // enum constructors are always private
        this.coordinateScale = coordinateScale;
    }

    public int scale() { return coordinateScale; }
}

Dimension.NETHER.scale();                        // 8
Dimension.valueOf("END");                        // exact name; unknown name -> IllegalArgumentException
Dimension.values();                              // all constants, in order
Dimension.END.ordinal();                         // 2: position in the list

String sky = switch (dimension) {                // switch over an enum: no default needed
    case OVERWORLD -> "blue";
    case NETHER -> "red";
    case END -> "black";
};
```

- Constants are written in UPPER_CASE and end with `;` before the fields.
- `valueOf` is case-sensitive: `"end"` fails. To accept any case (a config file!), clean the text first.
- A `switch` expression over an enum must cover every constant, or the compiler stops you.
- Inside an enum method you can switch over the enum itself: `switch (this) { ... }`.
- To ignore case use `text.toUpperCase(Locale.ROOT)` (`Locale` needs `import java.util.Locale;`, already in the starter).

## Your turn

- Open `src/main/java/oxide/enums/Rarity.java`.
- Give each constant its real colour and weight (the table is in the hint, or write your own from the tests).
- Write `parse` and `upgrade`.
