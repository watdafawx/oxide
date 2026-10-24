package oxide.split_in_two;

import static java.util.stream.Collectors.partitioningBy;

import java.util.List;
import java.util.Map;

public class Exam {
    /** true: scores of 50 or more, false: the rest. Both keys always present. */
    public static Map<Boolean, List<Integer>> passFail(List<Integer> scores) {
        return scores.stream().collect(partitioningBy(s -> s >= 50));
    }
}
