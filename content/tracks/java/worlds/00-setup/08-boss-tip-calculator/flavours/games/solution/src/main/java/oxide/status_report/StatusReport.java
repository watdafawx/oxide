package oxide.status_report;

import java.util.Locale;

public class StatusReport {
    /** "12/20" becomes {12, 20}: online, then max. Spaces are allowed. */
    public static int[] parseCounts(String raw) {
        String[] parts = raw.split("/");
        return new int[] {Integer.parseInt(parts[0].trim()), Integer.parseInt(parts[1].trim())};
    }

    /** Whole percent full: fillPercent(12, 20) is 60. A max of 0 gives 0. */
    public static int fillPercent(int online, int max) {
        if (max == 0) {
            return 0;
        }
        return online * 100 / max;
    }

    /** Servers needed for `players` at `perServer` each, rounded up: (45, 20) is 3. */
    public static int serversNeeded(int players, int perServer) {
        return (players + perServer - 1) / perServer;
    }

    /** One report line, e.g. "survival: 12/20 online (60%)". */
    public static String report(String name, String raw) {
        int[] counts = parseCounts(raw);
        int percent = fillPercent(counts[0], counts[1]);
        return String.format(Locale.ROOT, "%s: %d/%d online (%d%%)", name, counts[0], counts[1], percent);
    }
}
