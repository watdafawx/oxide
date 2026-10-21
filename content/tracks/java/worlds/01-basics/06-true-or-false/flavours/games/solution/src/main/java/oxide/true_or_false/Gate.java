package oxide.true_or_false;

public class Gate {
    /** True when the server has no free slot left. */
    public static boolean isFull(int online, int max) {
        return online >= max;
    }

    /** True when there is a free slot and the player is not banned. */
    public static boolean canJoin(int online, int max, boolean banned) {
        return online < max && !banned;
    }
}
