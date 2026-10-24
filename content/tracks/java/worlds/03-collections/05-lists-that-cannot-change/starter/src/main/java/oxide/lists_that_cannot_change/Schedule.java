package oxide.lists_that_cannot_change;

import java.util.ArrayList;
import java.util.List;

public class Schedule {
    /** Mon, Tue, Wed as a list that refuses changes. */
    public static List<String> openingDays() {
        // TODO: return a read-only list instead of this changeable one
        return new ArrayList<>(List.of("Mon", "Tue", "Wed"));
    }

    /** A read-only copy that does not follow later changes to source. */
    public static List<String> snapshot(List<String> source) {
        // TODO: return a read-only copy instead of the same list
        return source;
    }
}
