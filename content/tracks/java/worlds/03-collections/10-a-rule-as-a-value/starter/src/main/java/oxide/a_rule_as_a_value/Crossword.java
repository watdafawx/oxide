package oxide.a_rule_as_a_value;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;

public class Crossword {
    /** True for words longer than n letters. */
    public static Predicate<String> longerThan(int n) {
        // TODO: return a lambda w -> ... that compares the word's length with n
        return null;
    }

    /** A copy of words without the ones longer than n. */
    public static List<String> dropLong(List<String> words, int n) {
        List<String> copy = new ArrayList<>(words);
        // TODO: remove every word longer than n from the copy with removeIf
        return copy;
    }
}
