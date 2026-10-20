package oxide.flatmap_reduce;

import java.util.Comparator;
import java.util.List;
import java.util.Optional;
import java.util.stream.IntStream;

public class Flat {
    /** All the numbers of all inner lists, in order. */
    public static List<Integer> flatten(List<List<Integer>> lists) {
        throw new UnsupportedOperationException("TODO: turn the list of lists into one flat list");
    }

    /** 1 * 2 * ... * n (1 when n is 0). */
    public static int product(int n) {
        throw new UnsupportedOperationException("TODO: multiply 1 * 2 * ... * n together, starting from 1");
    }

    /** The biggest number, or empty when there are none. */
    public static Optional<Integer> largest(List<Integer> nums) {
        throw new UnsupportedOperationException("TODO: return the biggest number; max gives an Optional, empty when there are none");
    }
}
