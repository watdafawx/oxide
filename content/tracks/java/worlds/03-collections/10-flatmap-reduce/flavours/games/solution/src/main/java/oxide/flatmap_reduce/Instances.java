package oxide.flatmap_reduce;

import java.util.Comparator;
import java.util.List;
import java.util.Optional;

public class Instances {
    /** Every mod of every instance, in order. */
    public static List<Mod> allMods(List<List<Mod>> instances) {
        return instances.stream().flatMap(List::stream).toList();
    }

    /** Sum of every mod's downloads (0 for no mods). */
    public static int totalDownloads(List<Mod> mods) {
        return mods.stream().map(Mod::downloads).reduce(0, Integer::sum);
    }

    /** The mod with the most downloads, or empty when there are no mods. */
    public static Optional<Mod> mostDownloaded(List<Mod> mods) {
        return mods.stream().max(Comparator.comparingInt(Mod::downloads));
    }
}
