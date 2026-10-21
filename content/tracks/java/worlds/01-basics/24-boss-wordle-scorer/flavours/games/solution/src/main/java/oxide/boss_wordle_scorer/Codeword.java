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
        if (guess.equals(secret)) {
            return "GGGGG";
        }
        char[] result = new char[guess.length()];
        Arrays.fill(result, '.');
        int[] unclaimed = new int[26];
        for (int i = 0; i < guess.length(); i++) {
            if (guess.charAt(i) == secret.charAt(i)) {
                result[i] = 'G';
            } else {
                unclaimed[secret.charAt(i) - 'a']++;
            }
        }
        for (int i = 0; i < guess.length(); i++) {
            int slot = guess.charAt(i) - 'a';
            if (result[i] != 'G' && unclaimed[slot] > 0) {
                result[i] = 'Y';
                unclaimed[slot]--;
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

    /** The chat line: "Ada: " followed by the emoji row for this guess. */
    public static String reply(String player, String secret, String guess) {
        return player + ": " + render(score(secret, guess));
    }
}
