package oxide.same_name_different_inputs;

public class StatusFormat {
    /** "5 online" */
    public static String show(int playerCount) {
        return String.format("%d online", playerCount);
    }

    /** "ONLINE" or "OFFLINE" */
    public static String show(boolean online) {
        return online ? "ONLINE" : "OFFLINE";
    }

    /** "MOTD: <text>" */
    public static String show(String motd) {
        return "MOTD: " + motd;
    }
}
