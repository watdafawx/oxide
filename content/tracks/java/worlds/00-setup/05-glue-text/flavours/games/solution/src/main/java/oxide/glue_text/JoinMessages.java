package oxide.glue_text;

public class JoinMessages {
    /** "Ada joined the game". */
    public static String announceJoin(String player) {
        return player + " joined the game";
    }

    /** "Ada and Bo joined the game". */
    public static String announcePair(String first, String second) {
        return first + " and " + second + " joined the game";
    }
}
