package oxide.an_array_of_values;

public class HourlySamples {
    /** The first sample. The array is never empty. */
    public static int firstSample(int[] samples) {
        return samples[0];
    }

    /** The latest (last) sample. The array is never empty. */
    public static int latestSample(int[] samples) {
        return samples[samples.length - 1];
    }

    /** A fresh day: 24 hours, all 0. */
    public static int[] emptyDay() {
        return new int[24];
    }
}
