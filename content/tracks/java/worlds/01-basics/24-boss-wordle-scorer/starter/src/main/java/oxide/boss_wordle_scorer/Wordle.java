package oxide.boss_wordle_scorer;

import java.util.Arrays;

public class Wordle {
    static final String GREEN = "🟩";  // green square
    static final String YELLOW = "🟨"; // yellow square
    static final String GRAY = "⬛";         // black square

    /**
     * Scores a guess against the target (both 5 lowercase letters).
     * Returns one char per position: 'G' right place, 'Y' elsewhere in the word, '.' not available.
     */
    public static String score(String target, String guess) {
        return null; // TODO: start with: if the guess equals the target, return "GGGGG"
    }

    /** "GY." becomes GREEN + YELLOW + GRAY. */
    public static String render(String score) {
        return null; // TODO: loop over the chars with a StringBuilder
    }
}
