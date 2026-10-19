package oxide.maps;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Maps {
    /** How often each word appears. */
    public static Map<String, Integer> countWords(List<String> words) {
        Map<String, Integer> counts = new HashMap<>();
        for (String w : words) {
            counts.merge(w, 1, Integer::sum);
        }
        return counts;
    }

    /** First letter to the words starting with it, in input order. */
    public static Map<Character, List<String>> groupByFirstLetter(List<String> words) {
        Map<Character, List<String>> groups = new HashMap<>();
        for (String w : words) {
            groups.computeIfAbsent(w.charAt(0), k -> new ArrayList<>()).add(w);
        }
        return groups;
    }
}
