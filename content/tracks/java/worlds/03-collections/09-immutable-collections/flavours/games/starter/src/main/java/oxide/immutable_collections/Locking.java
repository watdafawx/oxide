package oxide.immutable_collections;

import java.util.ArrayList;
import java.util.List;

public class Locking {
    /** fabric, forge, neoforge, quilt as a read-only list. */
    public static List<String> supportedLoaders() {
        return new ArrayList<>(List.of("fabric", "forge", "neoforge", "quilt")); // TODO: return a read-only list instead
    }

    /** A read-only copy that does not follow later changes to the live list. */
    public static List<Mod> snapshot(List<Mod> live) {
        return live; // TODO: return a read-only copy instead
    }
}
