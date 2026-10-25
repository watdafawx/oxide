package oxide.sets_hold_each_once;

import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Enchantments {
    /** Every enchantment id once. */
    public static Set<String> distinct(List<String> ids) {
        return new HashSet<>(ids);
    }

    /** The first id that shows up a second time, or null when none does. */
    public static String firstRepeat(List<String> ids) {
        Set<String> seen = new HashSet<>();
        for (String id : ids) {
            if (!seen.add(id)) {
                return id;
            }
        }
        return null;
    }
}
