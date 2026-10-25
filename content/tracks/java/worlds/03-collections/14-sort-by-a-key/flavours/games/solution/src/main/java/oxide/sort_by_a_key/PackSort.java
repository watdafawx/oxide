package oxide.sort_by_a_key;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public class PackSort {
    /** A new list: A to Z by name. */
    public static List<Mod> byName(List<Mod> mods) {
        List<Mod> copy = new ArrayList<>(mods);
        copy.sort(Comparator.comparing(Mod::name));
        return copy;
    }

    /** A new list: most downloaded first. */
    public static List<Mod> byDownloadsDescending(List<Mod> mods) {
        List<Mod> copy = new ArrayList<>(mods);
        copy.sort(Comparator.comparingInt(Mod::downloads).reversed());
        return copy;
    }
}
