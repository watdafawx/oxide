package oxide.lists_that_cannot_change;

import java.util.List;

public class Schedule {
    /** Mon, Tue, Wed as a list that refuses changes. */
    public static List<String> openingDays() {
        return List.of("Mon", "Tue", "Wed");
    }

    /** A read-only copy that does not follow later changes to source. */
    public static List<String> snapshot(List<String> source) {
        return List.copyOf(source);
    }
}
