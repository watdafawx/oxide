package oxide.a_list_that_grows;

import java.util.ArrayList;

public class ModList {
    /** sodium, iris, lithium, in that order. */
    public static ArrayList<String> starterPack() {
        ArrayList<String> mods = new ArrayList<>();
        mods.add("sodium");
        mods.add("iris");
        mods.add("lithium");
        return mods;
    }

    /** mod0, mod1, ... with count entries. */
    public static ArrayList<String> placeholderNames(int count) {
        ArrayList<String> mods = new ArrayList<>();
        for (int i = 0; i < count; i++) {
            mods.add("mod" + i);
        }
        return mods;
    }
}
