package oxide.boss_first_report;

public class Report {
    /** "Status bot online: play.example.net" */
    public static String announce(String server) {
        throw new UnsupportedOperationException("TODO: glue the text and the server together with +");
    }

    /** "play.example.net has 3 of 20 players" */
    public static String occupancy(String server, int online, int max) {
        throw new UnsupportedOperationException("TODO: String.format with %s, %d and %d");
    }

    /** The first word typed after the program name. */
    public static String serverOf(String[] args) {
        throw new UnsupportedOperationException("TODO: take args[0]");
    }

    /** The two-line report: announce, a new line, then occupancy. */
    public static String report(String[] args, int online, int max) {
        // TODO: call serverOf, announce and occupancy, and join the two lines with "\n"
        throw new UnsupportedOperationException("TODO: build the two-line report");
    }
}
