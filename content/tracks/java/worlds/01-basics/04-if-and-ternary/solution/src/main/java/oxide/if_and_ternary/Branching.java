package oxide.if_and_ternary;

public class Branching {
    /** Forces v into the range lo..hi. */
    public static int clamp(int v, int lo, int hi) {
        if (v < lo) {
            return lo;
        }
        return v > hi ? hi : v;
    }

    /** -1 for negative, 0 for zero, 1 for positive. */
    public static int signOf(int n) {
        if (n < 0) {
            return -1;
        } else if (n > 0) {
            return 1;
        }
        return 0;
    }
}
