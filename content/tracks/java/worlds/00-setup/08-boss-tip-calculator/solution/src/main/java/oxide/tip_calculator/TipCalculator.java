package oxide.tip_calculator;

import java.util.Locale;

public class TipCalculator {
    /** "42.50" becomes 42.5. Spaces around the number are allowed. */
    public static double parseAmount(String raw) {
        return Double.parseDouble(raw.trim());
    }

    /** The amount plus tip: withTip(40.0, 15) is 46.0. */
    public static double withTip(double amount, int percent) {
        return amount + amount * percent / 100.0;
    }

    /** What each of `people` pays: splitEvenly(46.0, 4) is 11.5. */
    public static double splitEvenly(double total, int people) {
        return total / people;
    }

    /** The four-line receipt, lines joined with "\n" (no trailing newline). */
    public static String receipt(String rawAmount, int percent, int people) {
        double amount = parseAmount(rawAmount);
        double total = withTip(amount, percent);
        double tip = total - amount;
        return String.join("\n",
                String.format(Locale.ROOT, "Bill: $%.2f", amount),
                String.format(Locale.ROOT, "Tip (%d%%): $%.2f", percent, tip),
                String.format(Locale.ROOT, "Total: $%.2f", total),
                String.format(Locale.ROOT, "Each (%d): $%.2f", people, splitEvenly(total, people)));
    }
}
