package oxide.flip_in_place;

public class SampleOrder {
    /** Swaps the first and the last value. */
    public static void swapEnds(int[] samples) {
        int tmp = samples[0];
        samples[0] = samples[samples.length - 1];
        samples[samples.length - 1] = tmp;
    }

    /** Reverses the array itself; returns nothing. */
    public static void newestFirst(int[] samples) {
        for (int i = 0; i < samples.length / 2; i++) {
            int tmp = samples[i];
            samples[i] = samples[samples.length - 1 - i];
            samples[samples.length - 1 - i] = tmp;
        }
    }
}
