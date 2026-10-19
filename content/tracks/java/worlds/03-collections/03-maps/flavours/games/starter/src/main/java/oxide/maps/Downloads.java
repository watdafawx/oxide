package oxide.maps;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Downloads {
    /** How many downloads each mod id got. */
    public static Map<String, Integer> countDownloads(List<String> log) {
        throw new UnsupportedOperationException("TODO: loop over the log and merge(id, 1, Integer::sum)");
    }

    /** First letter to the mod names starting with it, in input order. */
    public static Map<Character, List<String>> indexByLetter(List<String> modNames) {
        throw new UnsupportedOperationException("TODO: computeIfAbsent(first letter, k -> new ArrayList<>()).add(name)");
    }
}
