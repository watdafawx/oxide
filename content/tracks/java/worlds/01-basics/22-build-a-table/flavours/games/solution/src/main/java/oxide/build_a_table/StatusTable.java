package oxide.build_a_table;

public class StatusTable {
    /** One row per server, e.g. "survival| 12\n". */
    public static String table(String[] servers, int[] players) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < servers.length; i++) {
            sb.append(String.format("%-8s|%3d\n", servers[i], players[i]));
        }
        return sb.toString();
    }
}
