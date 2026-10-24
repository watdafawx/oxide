package oxide.ask_for_a_list;

import java.util.List;

public class Pack {
    /** The longest mod id; the first one wins a tie. The list is never empty. */
    public static String longestId(List<String> ids) {
        String best = ids.get(0);
        for (String id : ids) {
            if (id.length() > best.length()) {
                best = id;
            }
        }
        return best;
    }
}
