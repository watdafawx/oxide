package oxide.arrays;

public class ArrayWork {
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

    /** Reverses the array itself; returns nothing. */
    public static void reverseInPlace(int[] values) {
        for (int i = 0; i < values.length / 2; i++) {
            int j = values.length - 1 - i;
            int tmp = values[i];
            values[i] = values[j];
            values[j] = tmp;
        }
    }

    /** {{1, 2, 3}, {4, 5, 6}} becomes {{1, 4}, {2, 5}, {3, 6}}. */
    public static int[][] transpose(int[][] grid) {
        int rows = grid.length;
        int cols = grid[0].length;
        int[][] result = new int[cols][rows];
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                result[c][r] = grid[r][c];
            }
        }
        return result;
    }
}
