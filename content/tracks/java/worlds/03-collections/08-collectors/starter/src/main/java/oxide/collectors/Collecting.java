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
        throw new UnsupportedOperationException("TODO: collect(groupingBy(String::length))");
    }

    /** Length to how many words have it. */
    public static Map<Integer, Long> countByLength(List<String> words) {
        throw new UnsupportedOperationException("TODO: groupingBy with a counting() downstream collector");
    }

    /** "[a, b, c]", or "[]" for no words. */
    public static String joinWithCommas(List<String> words) {
        throw new UnsupportedOperationException("TODO: collect(joining(separator, prefix, suffix))");
    }

    /** true: scores of 50 or more, false: the rest. */
    public static Map<Boolean, List<Integer>> partitionPassFail(List<Integer> scores) {
        throw new UnsupportedOperationException("TODO: collect(partitioningBy(...))");
    }
}
