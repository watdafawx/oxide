package oxide.printing;

import java.io.PrintStream;
import java.util.Locale;

public class Printer {
    /** Prints "Hello, <name>!" and a newline to out. */
    public static void printGreeting(PrintStream out, String name) {
        out.printf("Hello, %s!%n", name);
    }

    /** 12.5 becomes "$12.50". */
    public static String formatPrice(double price) {
        return String.format(Locale.ROOT, "$%.2f", price);
    }
}
