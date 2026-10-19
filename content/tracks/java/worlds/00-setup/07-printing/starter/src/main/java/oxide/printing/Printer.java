package oxide.printing;

import java.io.PrintStream;
import java.util.Locale;

public class Printer {
    /** Prints "Hello, <name>!" and a newline to out. */
    public static void printGreeting(PrintStream out, String name) {
        // TODO: use out.printf with a %s slot and %n
    }

    /** 12.5 becomes "$12.50". */
    public static String formatPrice(double price) {
        return null; // TODO: String.format(Locale.ROOT, ...) with %.2f
    }
}
