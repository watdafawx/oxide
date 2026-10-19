package oxide.tip_calculator;

import java.util.Locale;

public class TipCalculator {
    /** "42.50" becomes 42.5. Spaces around the number are allowed. */
    public static double parseAmount(String raw) {
        throw new UnsupportedOperationException("TODO: parse the text into a double");
    }

    /** The amount plus tip: withTip(40.0, 15) is 46.0. */
    public static double withTip(double amount, int percent) {
        throw new UnsupportedOperationException("TODO: add amount * percent / 100.0 to the amount");
    }

    /** What each of `people` pays: splitEvenly(46.0, 4) is 11.5. */
    public static double splitEvenly(double total, int people) {
        throw new UnsupportedOperationException("TODO: divide the total by people");
    }

    /** The four-line receipt, lines joined with "\n" (no trailing newline). */
    public static String receipt(String rawAmount, int percent, int people) {
        // TODO: use the three methods above, then build:
        //   Bill: $40.00
        //   Tip (15%): $6.00
        //   Total: $46.00
        //   Each (4): $11.50
        throw new UnsupportedOperationException("TODO: build the receipt text");
    }
}
