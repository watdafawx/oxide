package oxide.lists_that_cannot_change;

import java.util.ArrayList;
import java.util.List;

public class Locking {
    /** fabric, forge, neoforge, quilt as a list that refuses changes. */
    public static List<String> supportedLoaders() {
        // TODO: return a read-only list instead of this changeable one
        return new ArrayList<>(List.of("fabric", "forge", "neoforge", "quilt"));
    }

    /** A read-only copy that does not follow later changes to the live pack. */
    public static List<String> snapshot(List<String> pack) {
        // TODO: return a read-only copy instead of the same list
        return pack;
    }
}
