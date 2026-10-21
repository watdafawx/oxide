package oxide.flip_in_place;

public class Flip {
    /** Swaps the first and the last value. */
    public static void swapEnds(int[] values) {
        int tmp = values[0];
        values[0] = values[values.length - 1];
        values[values.length - 1] = tmp;
    }

    /** Reverses the array itself; returns nothing. */
    public static void reverseInPlace(int[] values) {
        for (int i = 0; i < values.length / 2; i++) {
            int tmp = values[i];
            values[i] = values[values.length - 1 - i];
            values[values.length - 1 - i] = tmp;
        }
    }
}
