package oxide.group_a_stream;

import static java.util.stream.Collectors.counting;
import static java.util.stream.Collectors.groupingBy;

import java.util.List;
import java.util.Map;

public class Buckets {
    /** Length to the words with that length, in input order. */
    public static Map<Integer, List<String>> groupByLength(List<String> words) {
        // TODO: collect with groupingBy on the length
        return Map.of();
    }

    /** Length to how many words have it. */
    public static Map<Integer, Long> countByLength(List<String> words) {
        // TODO: the same groupingBy, with counting() as the second argument
        return Map.of();
    }
}
