package oxide.numbers_need_a_box;

import java.util.ArrayList;
import java.util.List;

public class Downloads {
    /** Sum of all download counts (0 for none). */
    public static int totalDownloads(List<Integer> downloads) {
        int sum = 0;
        for (int d : downloads) {
            sum += d;
        }
        return sum;
    }

    /** A new list: every count plus bonus, same order. */
    public static List<Integer> withBonus(List<Integer> downloads, int bonus) {
        List<Integer> out = new ArrayList<>();
        for (int d : downloads) {
            out.add(d + bonus);
        }
        return out;
    }
}
