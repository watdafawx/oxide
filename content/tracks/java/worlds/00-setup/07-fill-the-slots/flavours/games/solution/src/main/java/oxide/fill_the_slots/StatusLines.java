package oxide.fill_the_slots;

public class StatusLines {
    /** "play.example.net: 3/20 online" */
    public static String statusLine(String server, int online, int max) {
        return String.format("%s: %d/%d online", server, online, max);
    }

    /** "survival: 12 online" */
    public static String onlineLine(String server, int online) {
        return String.format("%s: %d online", server, online);
    }
}
