package oxide.printing;

import java.io.PrintStream;
import java.util.Locale;

public class StatusPrinter {
    /** Prints e.g. "play.example.net: 3/20 online" and a newline to out. */
    public static void printStatus(PrintStream out, String server, int online, int max) {
        out.printf("%s: %d/%d online%n", server, online, max);
    }

    /** 19.9766 becomes "19.98". */
    public static String formatTps(double tps) {
        return String.format(Locale.ROOT, "%.2f", tps);
    }
}
