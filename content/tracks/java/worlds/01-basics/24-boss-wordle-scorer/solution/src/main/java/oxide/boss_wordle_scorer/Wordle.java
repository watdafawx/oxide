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
        if (guess.equals(target)) {
            return "GGGGG";
        }
        char[] result = new char[guess.length()];
        Arrays.fill(result, '.');
        int[] remaining = new int[26];
        for (int i = 0; i < guess.length(); i++) {
            if (guess.charAt(i) == target.charAt(i)) {
                result[i] = 'G';
            } else {
                remaining[target.charAt(i) - 'a']++;
            }
        }
        for (int i = 0; i < guess.length(); i++) {
            int slot = guess.charAt(i) - 'a';
            if (result[i] != 'G' && remaining[slot] > 0) {
                result[i] = 'Y';
                remaining[slot]--;
            }
        }
        return new String(result);
    }

    /** "GY." becomes GREEN + YELLOW + GRAY. */
    public static String render(String score) {
        StringBuilder sb = new StringBuilder();
        for (char c : score.toCharArray()) {
            sb.append(switch (c) {
                case 'G' -> GREEN;
                case 'Y' -> YELLOW;
                default -> GRAY;
            });
        }
        return sb.toString();
    }
}
