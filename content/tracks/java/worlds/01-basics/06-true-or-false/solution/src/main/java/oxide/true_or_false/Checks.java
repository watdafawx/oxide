package oxide.true_or_false;

public class Checks {
    /** True when there is no space left. */
    public static boolean isFull(int count, int capacity) {
        return count >= capacity;
    }

    /** True when there is space and the visitor is not banned. */
    public static boolean canEnter(int count, int capacity, boolean banned) {
        return count < capacity && !banned;
    }
}
