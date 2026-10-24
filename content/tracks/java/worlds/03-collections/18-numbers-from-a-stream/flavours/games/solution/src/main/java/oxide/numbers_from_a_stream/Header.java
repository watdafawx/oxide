package oxide.numbers_from_a_stream;

import java.util.List;

public class Header {
    /** Sum of every mod's downloads (0 for none). */
    public static int totalDownloads(List<Mod> mods) {
        return mods.stream().mapToInt(Mod::downloads).sum();
    }

    /** How many mods use this loader. */
    public static long countOn(List<Mod> mods, String loader) {
        return mods.stream().filter(m -> m.loader().equals(loader)).count();
    }
}
