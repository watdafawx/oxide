package oxide.collectors;

import static java.util.stream.Collectors.counting;
import static java.util.stream.Collectors.groupingBy;
import static java.util.stream.Collectors.joining;
import static java.util.stream.Collectors.partitioningBy;

import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

public class Collecting {
    /** Length to the words with that length, in input order. */
    public static Map<Integer, List<String>> groupByLength(List<String> words) {
        throw new UnsupportedOperationException("TODO: group the words by their length");
    }

    /** Length to how many words have it. */
    public static Map<Integer, Long> countByLength(List<String> words) {
        throw new UnsupportedOperationException("TODO: group by length again, but make each group a count instead of a list");
    }

    /** "[a, b, c]", or "[]" for no words. */
    public static String joinWithCommas(List<String> words) {
        throw new UnsupportedOperationException("TODO: join the words with ', ' between them, '[' in front and ']' at the end");
    }

    /** true: scores of 50 or more, false: the rest. */
    public static Map<Boolean, List<Integer>> partitionPassFail(List<Integer> scores) {
        throw new UnsupportedOperationException("TODO: split the scores into two groups: 50 or more (true) and the rest (false)");
    }
}
