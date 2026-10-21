package oxide.text_blocks;

public class StatusJson {
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
