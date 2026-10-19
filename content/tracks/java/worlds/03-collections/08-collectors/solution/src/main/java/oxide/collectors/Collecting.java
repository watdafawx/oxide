package oxide.collectors;

import static java.util.stream.Collectors.counting;
import static java.util.stream.Collectors.groupingBy;
import static java.util.stream.Collectors.joining;
import static java.util.stream.Collectors.partitioningBy;

import java.util.List;
import java.util.Map;

public class Collecting {
    /** Length to the words with that length, in input order. */
    public static Map<Integer, List<String>> groupByLength(List<String> words) {
        return words.stream().collect(groupingBy(String::length));
    }

    /** Length to how many words have it. */
    public static Map<Integer, Long> countByLength(List<String> words) {
        return words.stream().collect(groupingBy(String::length, counting()));
    }

    /** "[a, b, c]", or "[]" for no words. */
    public static String joinWithCommas(List<String> words) {
        return words.stream().collect(joining(", ", "[", "]"));
    }

    /** true: scores of 50 or more, false: the rest. */
    public static Map<Boolean, List<Integer>> partitionPassFail(List<Integer> scores) {
        return scores.stream().collect(partitioningBy(s -> s >= 50));
    }
}
