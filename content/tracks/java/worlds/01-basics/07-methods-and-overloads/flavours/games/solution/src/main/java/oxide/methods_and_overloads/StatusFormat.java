package oxide.methods_and_overloads;

public class StatusFormat {
    /** "5 online" */
    public static String show(int playerCount) {
        return playerCount + " online";
    }

    /** "ONLINE" or "OFFLINE" */
    public static String show(boolean online) {
        return online ? "ONLINE" : "OFFLINE";
    }

    /** "MOTD: <text>" */
    public static String show(String motd) {
        return "MOTD: " + motd;
    }

    /** Adds the player counts of any number of servers. totalPlayers() is 0. */
    public static int totalPlayers(int... perServer) {
        int total = 0;
        for (int n : perServer) {
            total += n;
        }
        return total;
    }
}
