package oxide.primitives;

public class Primitives {
    /** Adds two ints. Overflow wraps around silently. */
    public static int addInts(int a, int b) {
        return 0; // TODO: return a + b
    }

    /** Adds two ints and returns the true sum, even past Integer.MAX_VALUE. */
    public static long addAsLong(int a, int b) {
        return a + b; // TODO: this adds as int and then widens, so it still wraps
    }
}
