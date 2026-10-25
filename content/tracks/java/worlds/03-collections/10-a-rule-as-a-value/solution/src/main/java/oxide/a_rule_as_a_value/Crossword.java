package oxide.a_rule_as_a_value;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;

public class Crossword {
    /** True for words longer than n letters. */
    public static Predicate<String> longerThan(int n) {
        return w -> w.length() > n;
    }

    /** A copy of words without the ones longer than n. */
    public static List<String> dropLong(List<String> words, int n) {
        List<String> copy = new ArrayList<>(words);
        copy.removeIf(longerThan(n));
        return copy;
    }
}
