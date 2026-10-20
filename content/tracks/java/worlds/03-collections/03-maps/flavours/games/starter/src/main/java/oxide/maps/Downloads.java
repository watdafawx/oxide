package oxide.maps;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Downloads {
    /** How many downloads each mod id got. */
    public static Map<String, Integer> countDownloads(List<String> log) {
        throw new UnsupportedOperationException("TODO: count every id in the log: 1 the first time, old count + 1 after that");
    }

    /** First letter to the mod names starting with it, in input order. */
    public static Map<Character, List<String>> indexByLetter(List<String> modNames) {
        throw new UnsupportedOperationException("TODO: for each name, find the list for its first letter (create it when missing) and add the name");
    }
}
