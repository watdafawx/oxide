package oxide.maps;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Maps {
    /** How often each word appears. */
    public static Map<String, Integer> countWords(List<String> words) {
        throw new UnsupportedOperationException("TODO: loop over the words and merge(w, 1, Integer::sum)");
    }

    /** First letter to the words starting with it, in input order. */
    public static Map<Character, List<String>> groupByFirstLetter(List<String> words) {
        throw new UnsupportedOperationException("TODO: computeIfAbsent(first letter, k -> new ArrayList<>()).add(word)");
    }
}
