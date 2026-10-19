package oxide.text_blocks_and_format;

public class Report {
    /** One row per name, e.g. "Ada   |  95\n". */
    public static String table(String[] names, int[] scores) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < names.length; i++) {
            sb.append(String.format("%-6s|%4d\n", names[i], scores[i]));
        }
        return sb.toString();
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
        return """
            {
              "status": "%s",
              "count": %d
            }
            """.formatted(status, count);
    }
}
