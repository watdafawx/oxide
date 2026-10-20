package oxide.status_report;

import java.util.Locale;

public class StatusReport {
    /** "12/20" becomes {12, 20}: online, then max. Spaces are allowed. */
    public static int[] parseCounts(String raw) {
        throw new UnsupportedOperationException("TODO: split on \"/\", trim, parse both parts");
    }

    /** Whole percent full: fillPercent(12, 20) is 60. A max of 0 gives 0. */
    public static int fillPercent(int online, int max) {
        throw new UnsupportedOperationException("TODO: whole percent that online is of max (multiply before dividing; max of 0 gives 0)");
    }

    /** Servers needed for `players` at `perServer` each, rounded up: (45, 20) is 3. */
    public static int serversNeeded(int players, int perServer) {
        throw new UnsupportedOperationException("TODO: divide players by perServer, rounding up");
    }

    /** One report line, e.g. "survival: 12/20 online (60%)". */
    public static String report(String name, String raw) {
        // TODO: parse the counts, work out the percent, then format the line
        throw new UnsupportedOperationException("TODO: build the report line");
    }
}
