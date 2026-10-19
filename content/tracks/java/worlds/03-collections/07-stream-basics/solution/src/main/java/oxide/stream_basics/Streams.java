package oxide.stream_basics;

import java.util.List;

public class Streams {
    /** The squares of the even numbers, in input order. */
    public static List<Integer> evenSquares(List<Integer> nums) {
        return nums.stream().filter(n -> n % 2 == 0).map(n -> n * n).toList();
    }

    /** Total number of characters in all words. */
    public static int totalLength(List<String> words) {
        return words.stream().mapToInt(String::length).sum();
    }
}
