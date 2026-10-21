package oxide.largest_so_far;

public class Extremes {
    /** Largest element. The array is never empty. */
    public static int max(int[] values) {
        int best = values[0];
        for (int v : values) {
            if (v > best) {
                best = v;
            }
        }
        return best;
    }

    /** Smallest element. The array is never empty. */
    public static int min(int[] values) {
        int best = values[0];
        for (int v : values) {
            if (v < best) {
                best = v;
            }
        }
        return best;
    }
}
