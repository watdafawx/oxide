package oxide.maybe_there_is_none;

import java.util.Comparator;
import java.util.List;
import java.util.Optional;

public class Header {
    /** The mod with the most downloads, or empty when there are none. No ties. */
    public static Optional<Mod> mostDownloaded(List<Mod> mods) {
        // TODO: stream the mods and take the max by downloads
        return Optional.empty();
    }

    /** The most downloaded mod's name, or "none". */
    public static String headline(List<Mod> mods) {
        // TODO: mostDownloaded(mods), map to the name, orElse "none"
        return "";
    }
}
