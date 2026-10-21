package oxide.fractions_with_double;

public class Fill {
    /** How full the server is, 0.0 to 1.0: fillRatio(17, 20) is 0.85. */
    public static double fillRatio(int online, int capacity) {
        return (double) online / capacity;
    }

    /** The average of two pings: averagePing(30, 45) is 37.5. */
    public static double averagePing(int first, int second) {
        return (first + second) / 2.0;
    }
}
