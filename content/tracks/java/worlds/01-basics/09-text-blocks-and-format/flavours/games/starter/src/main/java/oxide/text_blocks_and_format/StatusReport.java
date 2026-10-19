package oxide.text_blocks_and_format;

public class StatusReport {
    /** One row per server, e.g. "survival| 12\n". */
    public static String table(String[] servers, int[] players) {
        return null; // TODO: StringBuilder + String.format("%-8s|%3d\n", ...)
    }

    /**
     * Returns exactly this text (note the final newline):
     * <pre>
     * {
     *   "status": "online",
     *   "players": 12
     * }
     * </pre>
     */
    public static String statusJson(String status, int playerCount) {
        return null; // TODO: a text block with %s and %d, then .formatted(status, playerCount)
    }
}
