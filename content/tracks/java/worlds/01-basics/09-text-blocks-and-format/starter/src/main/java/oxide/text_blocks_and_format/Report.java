package oxide.text_blocks_and_format;

public class Report {
    /** One row per name, e.g. "Ada   |  95\n". */
    public static String table(String[] names, int[] scores) {
        return null; // TODO: StringBuilder + String.format("%-6s|%4d\n", ...)
    }

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
        return null; // TODO: a text block with %s and %d, then .formatted(status, count)
    }
}
