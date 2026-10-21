package oxide.a_name_for_a_count;

public class Tally {
    /** Players online on survival (12) and creative (3), added together. */
    public static int totalOnline() {
        int survival = 12;
        int creative = 3;
        return survival + creative;
    }

    /** Free slots on a server with capacity 20 and 12 players online. */
    public static int slotsLeft() {
        int capacity = 20;
        int online = 12;
        return capacity - online;
    }
}
