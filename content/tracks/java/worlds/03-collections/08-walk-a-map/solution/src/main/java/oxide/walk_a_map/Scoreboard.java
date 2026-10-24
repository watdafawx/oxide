package oxide.walk_a_map;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

public class Scoreboard {
    /** The name with the highest score. The map is never empty and has no ties. */
    public static String topScorer(Map<String, Integer> scores) {
        String best = null;
        int bestScore = Integer.MIN_VALUE;
        for (Map.Entry<String, Integer> e : scores.entrySet()) {
            if (e.getValue() > bestScore) {
                best = e.getKey();
                bestScore = e.getValue();
            }
        }
        return best;
    }

    /** Every name with a score of at least min, in any order. */
    public static List<String> passing(Map<String, Integer> scores, int min) {
        List<String> out = new ArrayList<>();
        for (Map.Entry<String, Integer> e : scores.entrySet()) {
            if (e.getValue() >= min) {
                out.add(e.getKey());
            }
        }
        return out;
    }
}
