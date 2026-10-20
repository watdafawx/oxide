package oxide.boss_wordle_scorer;

import java.util.Arrays;

public class Codeword {
    static final String GREEN = "🟩";  // green square
    static final String YELLOW = "🟨"; // yellow square
    static final String GRAY = "⬛";         // black square

    /**
     * Scores a guess against the secret (both 5 lowercase letters).
     * Returns one char per position: 'G' right place, 'Y' elsewhere in the word, '.' not available.
     */
    public static String score(String secret, String guess) {
        return null; // TODO: start with: if the guess equals the secret, return "GGGGG"
    }

    /** "GY." becomes GREEN + YELLOW + GRAY. */
    public static String render(String score) {
        return null; // TODO: loop over the chars with a StringBuilder
    }

    /** The chat line: "Ada: " followed by the emoji row for this guess. */
    public static String reply(String player, String secret, String guess) {
        return null; // TODO: call score, then render, then add the player name
    }
}
