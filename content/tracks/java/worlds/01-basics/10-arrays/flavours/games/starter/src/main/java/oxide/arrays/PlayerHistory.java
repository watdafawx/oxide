package oxide.arrays;

public class PlayerHistory {
    /** Highest sample. The array is never empty. */
    public static int peak(int[] samples) {
        return 0; // TODO: start from samples[0] and keep the bigger one
    }

    /** Reverses the array itself; returns nothing. */
    public static void newestFirst(int[] samples) {
        // TODO: swap the first with the last, and so on, up to the middle
    }

    /** {{1, 2, 3}, {4, 5, 6}} (servers x hours) becomes {{1, 4}, {2, 5}, {3, 6}} (hours x servers). */
    public static int[][] byHour(int[][] grid) {
        return null; // TODO: build a new grid with servers and hours swapped, then copy every cell across
    }
}
