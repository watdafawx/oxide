package oxide.immutable_collections;

import java.util.ArrayList;
import java.util.List;

public class Locking {
    /** fabric, forge, neoforge, quilt as a read-only list. */
    public static List<String> supportedLoaders() {
        return List.of("fabric", "forge", "neoforge", "quilt");
    }

    /** A read-only copy that does not follow later changes to the live list. */
    public static List<Mod> snapshot(List<Mod> live) {
        return List.copyOf(live);
    }
}
