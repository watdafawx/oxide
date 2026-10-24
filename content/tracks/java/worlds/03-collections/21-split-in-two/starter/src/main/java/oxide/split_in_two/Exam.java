package oxide.split_in_two;

import static java.util.stream.Collectors.partitioningBy;

import java.util.List;
import java.util.Map;

public class Exam {
    /** true: scores of 50 or more, false: the rest. Both keys always present. */
    public static Map<Boolean, List<Integer>> passFail(List<Integer> scores) {
        // TODO: collect with partitioningBy and a rule for "50 or more"
        return Map.of();
    }
}
