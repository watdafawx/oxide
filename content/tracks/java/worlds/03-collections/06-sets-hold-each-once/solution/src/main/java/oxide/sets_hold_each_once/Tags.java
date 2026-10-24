package oxide.sets_hold_each_once;

import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Tags {
    /** Every tag once. */
    public static Set<String> distinct(List<String> tags) {
        return new HashSet<>(tags);
    }

    /** The first tag that shows up a second time, or null when none does. */
    public static String firstRepeat(List<String> tags) {
        Set<String> seen = new HashSet<>();
        for (String t : tags) {
            if (!seen.add(t)) {
                return t;
            }
        }
        return null;
    }
}
