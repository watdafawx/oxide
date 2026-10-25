package oxide.lists_that_cannot_change;

import java.util.List;

public class Locking {
    /** fabric, forge, neoforge, quilt as a list that refuses changes. */
    public static List<String> supportedLoaders() {
        return List.of("fabric", "forge", "neoforge", "quilt");
    }

    /** A read-only copy that does not follow later changes to the live pack. */
    public static List<String> snapshot(List<String> pack) {
        return List.copyOf(pack);
    }
}
