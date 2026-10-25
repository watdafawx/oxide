package oxide.group_into_lists;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Piles {
    /** Word length to the words with that length, in input order. */
    public static Map<Integer, List<String>> byLength(List<String> words) {
        Map<Integer, List<String>> groups = new HashMap<>();
        for (String w : words) {
            groups.computeIfAbsent(w.length(), k -> new ArrayList<>()).add(w);
        }
        return groups;
    }
}
