package oxide.boss_first_report;

public class Badge {
    /** "Welcome to DevDay" */
    public static String title(String event) {
        return "Welcome to " + event;
    }

    /** "Ada: seat 3 of 40" */
    public static String seatLine(String name, int seat, int total) {
        return String.format("%s: seat %d of %d", name, seat, total);
    }

    /** The first word typed after the program name. */
    public static String eventOf(String[] args) {
        return args[0];
    }

    /** The two-line badge: title, a new line, then seatLine. */
    public static String badge(String[] args, String name, int seat, int total) {
        return title(eventOf(args)) + "\n" + seatLine(name, seat, total);
    }
}
