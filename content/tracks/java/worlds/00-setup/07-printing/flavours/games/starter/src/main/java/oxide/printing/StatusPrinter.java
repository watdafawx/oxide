package oxide.printing;

import java.io.PrintStream;
import java.util.Locale;

public class StatusPrinter {
    /** Prints e.g. "play.example.net: 3/20 online" and a newline to out. */
    public static void printStatus(PrintStream out, String server, int online, int max) {
        // TODO: use out.printf with %s, %d, %d and %n
    }

    /** 19.9766 becomes "19.98". */
    public static String formatTps(double tps) {
        return null; // TODO: String.format(Locale.ROOT, ...) with %.2f
    }
}
