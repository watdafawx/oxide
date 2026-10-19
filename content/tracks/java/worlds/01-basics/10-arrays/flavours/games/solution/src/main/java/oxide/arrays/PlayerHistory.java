package oxide.arrays;

public class PlayerHistory {
    /** Highest sample. The array is never empty. */
    public static int peak(int[] samples) {
        int best = samples[0];
        for (int s : samples) {
            if (s > best) {
                best = s;
            }
        }
        return best;
    }

    /** Reverses the array itself; returns nothing. */
    public static void newestFirst(int[] samples) {
        for (int i = 0; i < samples.length / 2; i++) {
            int j = samples.length - 1 - i;
            int tmp = samples[i];
            samples[i] = samples[j];
            samples[j] = tmp;
        }
    }

    /** {{1, 2, 3}, {4, 5, 6}} (servers x hours) becomes {{1, 4}, {2, 5}, {3, 6}} (hours x servers). */
    public static int[][] byHour(int[][] grid) {
        int servers = grid.length;
        int hours = grid[0].length;
        int[][] result = new int[hours][servers];
        for (int s = 0; s < servers; s++) {
            for (int h = 0; h < hours; h++) {
                result[h][s] = grid[s][h];
            }
        }
        return result;
    }
}
