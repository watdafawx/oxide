package oxide.the_one_line_choice;

public class StatusWords {
    /** "ONLINE" or "OFFLINE". */
    public static String statusWord(boolean online) {
        return online ? "ONLINE" : "OFFLINE";
    }

    /** "server" for exactly one, otherwise "servers". */
    public static String serverWord(int count) {
        return count == 1 ? "server" : "servers";
    }
}
