package oxide.numbers_from_a_stream;

import java.util.List;

public class WordStats {
    /** Total number of letters in all words (0 for none). */
    public static int totalLetters(List<String> words) {
        return words.stream().mapToInt(String::length).sum();
    }

    /** How many words are longer than n letters. */
    public static long countLonger(List<String> words, int n) {
        return words.stream().filter(w -> w.length() > n).count();
    }
}
