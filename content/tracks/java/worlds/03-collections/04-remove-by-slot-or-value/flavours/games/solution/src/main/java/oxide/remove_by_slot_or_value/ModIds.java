package oxide.remove_by_slot_or_value;

import java.util.ArrayList;
import java.util.List;

public class ModIds {
    /** A copy of the project ids without the one at position slot. */
    public static List<Integer> dropSlot(List<Integer> projectIds, int slot) {
        List<Integer> copy = new ArrayList<>(projectIds);
        copy.remove(slot);
        return copy;
    }

    /** A copy of the project ids without projectId. */
    public static List<Integer> dropProject(List<Integer> projectIds, int projectId) {
        List<Integer> copy = new ArrayList<>(projectIds);
        copy.remove(Integer.valueOf(projectId));
        return copy;
    }
}
