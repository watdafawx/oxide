package oxide.lists;

import java.util.ArrayList;
import java.util.List;

public class ModList {
    /** Copy of the ids without the mod in slot and without the mod with projectId. */
    public static List<Integer> dropMods(List<Integer> projectIds, int slot, int projectId) {
        List<Integer> copy = new ArrayList<>(projectIds);
        copy.remove(slot);
        copy.remove(Integer.valueOf(projectId));
        return copy;
    }

    /** ["mod0", "mod1", ...] with count entries. */
    public static List<String> placeholderNames(int count) {
        List<String> out = new ArrayList<>(2);
        for (int i = 0; i < count; i++) {
            out.add("mod" + i);
        }
        return out;
    }
}
