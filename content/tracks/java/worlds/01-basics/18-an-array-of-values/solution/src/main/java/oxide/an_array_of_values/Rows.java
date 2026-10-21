package oxide.an_array_of_values;

public class Rows {
    /** The first value. The array is never empty. */
    public static int first(int[] values) {
        return values[0];
    }

    /** The last value. The array is never empty. */
    public static int last(int[] values) {
        return values[values.length - 1];
    }

    /** A new array of the given size, all zeros. */
    public static int[] zeros(int count) {
        return new int[count];
    }
}
