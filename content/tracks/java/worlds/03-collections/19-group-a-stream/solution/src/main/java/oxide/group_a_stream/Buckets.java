package oxide.group_a_stream;

import static java.util.stream.Collectors.counting;
import static java.util.stream.Collectors.groupingBy;

import java.util.List;
import java.util.Map;

public class Buckets {
    /** Length to the words with that length, in input order. */
    public static Map<Integer, List<String>> groupByLength(List<String> words) {
        return words.stream().collect(groupingBy(String::length));
    }

    /** Length to how many words have it. */
    public static Map<Integer, Long> countByLength(List<String> words) {
        return words.stream().collect(groupingBy(String::length, counting()));
    }
}
