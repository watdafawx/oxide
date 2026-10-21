package oxide.text_never_changes;

public class MotdTools {
    /** "  Welcome " becomes "Welcome". */
    public static String tidy(String raw) {
        return raw.trim();
    }

    /** "Welcome" becomes "WELCOME". */
    public static String shout(String motd) {
        return motd.toUpperCase();
    }

    /** "Welcome" has 7 characters. */
    public static int motdLength(String motd) {
        return motd.length();
    }
}
