package oxide.a_rule_as_a_value;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.function.Predicate;
import org.junit.jupiter.api.Test;

class CrosswordTest {
    @Test
    void longerThanBuildsTheRule() {
        Predicate<String> overSix = Crossword.longerThan(6);
        assertThat(overSix.test("bookshelf")).isTrue();
        assertThat(overSix.test("planet")).isFalse();
        assertThat(Crossword.longerThan(0).test("")).isFalse();
    }

    @Test
    void dropLongRemovesMatchingWords() {
        List<String> words = List.of("cat", "bookshelf", "planet", "umbrella");
        assertThat(Crossword.dropLong(words, 6)).containsExactly("cat", "planet");
        assertThat(Crossword.dropLong(words, 2)).isEmpty();
    }
}
