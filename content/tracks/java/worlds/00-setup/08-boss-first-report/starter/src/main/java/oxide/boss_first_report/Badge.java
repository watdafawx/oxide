package oxide.boss_first_report;

public class Badge {
    /** "Welcome to DevDay" */
    public static String title(String event) {
        throw new UnsupportedOperationException("TODO: glue the text and the event together with +");
    }

    /** "Ada: seat 3 of 40" */
    public static String seatLine(String name, int seat, int total) {
        throw new UnsupportedOperationException("TODO: String.format with %s, %d and %d");
    }

    /** The first word typed after the program name. */
    public static String eventOf(String[] args) {
        throw new UnsupportedOperationException("TODO: take args[0]");
    }

    /** The two-line badge: title, a new line, then seatLine. */
    public static String badge(String[] args, String name, int seat, int total) {
        // TODO: call eventOf, title and seatLine, and join the two lines with "\n"
        throw new UnsupportedOperationException("TODO: build the two-line badge");
    }
}
