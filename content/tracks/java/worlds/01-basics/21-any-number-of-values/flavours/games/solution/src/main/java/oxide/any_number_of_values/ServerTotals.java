package oxide.any_number_of_values;

public class ServerTotals {
    /** How many counts were passed in. serverCount() is 0. */
    public static int serverCount(int... perServer) {
        return perServer.length;
    }

    /** Adds the player counts of any number of servers. totalPlayers() is 0. */
    public static int totalPlayers(int... perServer) {
        int total = 0;
        for (int count : perServer) {
            total += count;
        }
        return total;
    }
}
