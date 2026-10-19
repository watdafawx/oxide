package oxide.immutable_collections;

import java.util.ArrayList;
import java.util.List;

public class Immutables {
    /** Mon, Tue, Wed as a read-only list. */
    public static List<String> readOnlyDays() {
        return List.of("Mon", "Tue", "Wed");
    }

    /** A read-only copy that does not follow later changes to source. */
    public static List<String> snapshot(List<String> source) {
        return List.copyOf(source);
    }
}
