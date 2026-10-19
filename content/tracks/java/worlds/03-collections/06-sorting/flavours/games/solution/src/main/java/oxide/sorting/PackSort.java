package oxide.sorting;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public class PackSort {
    /** New list: grouped by loader, A-Z by name inside each loader. */
    public static List<Mod> byLoaderThenName(List<Mod> mods) {
        List<Mod> copy = new ArrayList<>(mods);
        copy.sort(Comparator.comparing(Mod::loader).thenComparing(Mod::name));
        return copy;
    }

    /** New list: most downloaded mod first. */
    public static List<Mod> byDownloadsDescending(List<Mod> mods) {
        List<Mod> copy = new ArrayList<>(mods);
        copy.sort(Comparator.comparingInt(Mod::downloads).reversed());
        return copy;
    }
}
