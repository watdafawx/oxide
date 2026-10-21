package oxide.boss_first_report;

public class Report {
    /** "Status bot online: play.example.net" */
    public static String announce(String server) {
        return "Status bot online: " + server;
    }

    /** "play.example.net has 3 of 20 players" */
    public static String occupancy(String server, int online, int max) {
        return String.format("%s has %d of %d players", server, online, max);
    }

    /** The first word typed after the program name. */
    public static String serverOf(String[] args) {
        return args[0];
    }

    /** The two-line report: announce, a new line, then occupancy. */
    public static String report(String[] args, int online, int max) {
        return announce(serverOf(args)) + "\n" + occupancy(serverOf(args), online, max);
    }
}
