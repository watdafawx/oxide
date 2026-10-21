package oxide.largest_so_far;

public class PeakFinder {
    /** Highest sample. The array is never empty. */
    public static int peak(int[] samples) {
        int best = samples[0];
        for (int v : samples) {
            if (v > best) {
                best = v;
            }
        }
        return best;
    }

    /** Lowest sample. The array is never empty. */
    public static int low(int[] samples) {
        int best = samples[0];
        for (int v : samples) {
            if (v < best) {
                best = v;
            }
        }
        return best;
    }
}
