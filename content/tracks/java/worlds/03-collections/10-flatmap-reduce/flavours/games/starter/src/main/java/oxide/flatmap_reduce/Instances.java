package oxide.flatmap_reduce;

import java.util.Comparator;
import java.util.List;
import java.util.Optional;

public class Instances {
    /** Every mod of every instance, in order. */
    public static List<Mod> allMods(List<List<Mod>> instances) {
        throw new UnsupportedOperationException("TODO: stream().flatMap(List::stream).toList()");
    }

    /** Sum of every mod's downloads (0 for no mods). */
    public static int totalDownloads(List<Mod> mods) {
        throw new UnsupportedOperationException("TODO: map to downloads, then reduce(0, Integer::sum)");
    }

    /** The mod with the most downloads, or empty when there are no mods. */
    public static Optional<Mod> mostDownloaded(List<Mod> mods) {
        throw new UnsupportedOperationException("TODO: stream().max(...) returns an Optional");
    }
}
