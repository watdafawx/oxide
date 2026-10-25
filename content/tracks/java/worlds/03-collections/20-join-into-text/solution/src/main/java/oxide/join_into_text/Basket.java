package oxide.join_into_text;

import static java.util.stream.Collectors.joining;

import java.util.List;

public class Basket {
    /** "[a, b, c]", or "[]" for no words. */
    public static String bracketed(List<String> words) {
        return words.stream().collect(joining(", ", "[", "]"));
    }
}
