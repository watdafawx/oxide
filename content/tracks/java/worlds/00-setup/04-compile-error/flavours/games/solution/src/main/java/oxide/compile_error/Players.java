package oxide.compile_error;

public class Players {
    /** Total players online across the survival and creative servers. */
    public static int totalOnline(int survival, int creative) {
        int total = survival + creative;
        return total;
    }
}
