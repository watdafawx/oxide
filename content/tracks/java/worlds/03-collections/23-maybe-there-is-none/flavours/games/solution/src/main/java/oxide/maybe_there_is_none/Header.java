package oxide.maybe_there_is_none;

import java.util.Comparator;
import java.util.List;
import java.util.Optional;

public class Header {
    /** The mod with the most downloads, or empty when there are none. No ties. */
    public static Optional<Mod> mostDownloaded(List<Mod> mods) {
        return mods.stream().max(Comparator.comparingInt(Mod::downloads));
    }

    /** The most downloaded mod's name, or "none". */
    public static String headline(List<Mod> mods) {
        return mostDownloaded(mods).map(Mod::name).orElse("none");
    }
}
