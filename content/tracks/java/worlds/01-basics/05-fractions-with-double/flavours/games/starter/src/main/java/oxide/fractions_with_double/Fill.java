package oxide.fractions_with_double;

public class Fill {
    /** How full the server is, 0.0 to 1.0: fillRatio(17, 20) is 0.85. */
    public static double fillRatio(int online, int capacity) {
        // TODO: this is int division widened afterwards; cast one side first
        return online / capacity;
    }

    /** The average of two pings: averagePing(30, 45) is 37.5. */
    public static double averagePing(int first, int second) {
        return 0.0; // TODO: add them, then divide by 2.0
    }
}
