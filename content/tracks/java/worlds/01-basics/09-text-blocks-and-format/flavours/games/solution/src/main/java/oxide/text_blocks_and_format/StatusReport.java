package oxide.text_blocks_and_format;

public class StatusReport {
    /** One row per server, e.g. "survival| 12\n". */
    public static String table(String[] servers, int[] players) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < servers.length; i++) {
            sb.append(String.format("%-8s|%3d\n", servers[i], players[i]));
        }
        return sb.toString();
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
        return """
            {
              "status": "%s",
              "players": %d
            }
            """.formatted(status, playerCount);
    }
}
