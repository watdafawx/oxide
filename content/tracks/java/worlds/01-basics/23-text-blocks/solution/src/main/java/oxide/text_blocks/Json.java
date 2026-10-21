package oxide.text_blocks;

public class Json {
    /**
     * Returns exactly this text (note the final newline):
     * <pre>
     * {
     *   "status": "ok",
     *   "count": 3
     * }
     * </pre>
     */
    public static String jsonBlock(String status, int count) {
        return """
                {
                  "status": "%s",
                  "count": %d
                }
                """.formatted(status, count);
    }
}
