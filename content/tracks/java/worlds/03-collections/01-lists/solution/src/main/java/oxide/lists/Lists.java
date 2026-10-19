package oxide.lists;

import java.util.ArrayList;
import java.util.List;

public class Lists {
    /** Copy of nums without the element at index and without the first element equal to value. */
    public static List<Integer> removeIndexAndValue(List<Integer> nums, int index, int value) {
        List<Integer> copy = new ArrayList<>(nums);
        copy.remove(index);
        copy.remove(Integer.valueOf(value));
        return copy;
    }

    /** ["item0", "item1", ...] with count entries. */
    public static List<String> labels(int count) {
        List<String> out = new ArrayList<>(2);
        for (int i = 0; i < count; i++) {
            out.add("item" + i);
        }
        return out;
    }
}
