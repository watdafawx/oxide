package oxide.numbers_need_a_box;

import java.util.ArrayList;
import java.util.List;

public class Scores {
    /** Sum of all scores (0 for none). */
    public static int total(List<Integer> scores) {
        int sum = 0;
        for (int s : scores) {
            sum += s;
        }
        return sum;
    }

    /** A new list: every score plus bonus, same order. */
    public static List<Integer> curved(List<Integer> scores, int bonus) {
        List<Integer> out = new ArrayList<>();
        for (int s : scores) {
            out.add(s + bonus);
        }
        return out;
    }
}
