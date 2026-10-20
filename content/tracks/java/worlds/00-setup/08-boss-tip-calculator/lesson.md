**Why this matters:** this is what real programs do all day: take messy text from the outside world, turn it into numbers, do the maths, and print something a person can read. You already have every piece.

The spec, in order:

- `parseAmount("42.50")` turns text into a `double`. It must cope with spaces.
- `withTip(40.0, 15)` returns the total with the tip added: `46.0`.
- `splitEvenly(46.0, 4)` returns what each person pays: `11.5`.
- `receipt("40.00", 15, 4)` returns this exact text, lines joined with `\n`:

```
Bill: $40.00
Tip (15%): $6.00
Total: $46.00
Each (4): $11.50
```

New tools for this boss (printing was the only one you have met):

```java
String clean = "  42.50 ".trim();                    // "42.50": drops spaces at both ends
double amount = Double.parseDouble(clean);           // 42.5: text -> double
String line = String.format(Locale.ROOT, "Bill: $%.2f", amount);   // you know this one from printing
String text = String.join("\n", "line one", "line two");           // glue lines with \n between them
```

`\n` is the newline character inside a Java string. Money as `double` is fine for a toy; real money code uses whole cents or `BigDecimal`.

## Your turn

- Open `src/main/java/oxide/tip_calculator/TipCalculator.java`.
- Fill in the four methods. Each objective turns green as you finish its method.
- If one stays red, read `expected` and `but was`.
