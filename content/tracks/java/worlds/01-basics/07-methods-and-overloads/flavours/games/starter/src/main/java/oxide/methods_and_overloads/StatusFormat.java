package oxide.methods_and_overloads;

public class StatusFormat {
    /** "5 online" */
    public static String show(int playerCount) {
        return null; // TODO: playerCount + " online"
    }

    /** "ONLINE" or "OFFLINE" */
    public static String show(boolean online) {
        return null; // TODO: use a ternary
    }

    /** "MOTD: <text>" */
    public static String show(String motd) {
        return null; // TODO: "MOTD: " + motd
    }

    /** Adds the player counts of any number of servers. totalPlayers() is 0. */
    public static int totalPlayers(int... perServer) {
        return 0; // TODO: loop over perServer
    }
}
