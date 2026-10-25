package oxide.change_each_item;

import java.util.List;

public class Popular {
    /** Every mod's name, in order. */
    public static List<String> names(List<Mod> mods) {
        return mods.stream().map(Mod::name).toList();
    }

    /** The names of the first three mods (fewer if there are fewer). */
    public static List<String> topThree(List<Mod> ranked) {
        return ranked.stream().limit(3).map(Mod::name).toList();
    }
}
