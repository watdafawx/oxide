package oxide.build_a_table;

public class Table {
    /** One row per name, e.g. "Ada   |  95\n". */
    public static String table(String[] names, int[] scores) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < names.length; i++) {
            sb.append(String.format("%-6s|%4d\n", names[i], scores[i]));
        }
        return sb.toString();
    }
}
