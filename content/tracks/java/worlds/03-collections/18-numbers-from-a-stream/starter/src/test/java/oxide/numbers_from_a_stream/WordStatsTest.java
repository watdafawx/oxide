package oxide.numbers_from_a_stream;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class WordStatsTest {
    @Test
    void totalLettersAddsEveryLength() {
        assertThat(WordStats.totalLetters(List.of("ab", "cde", ""))).isEqualTo(5);
        assertThat(WordStats.totalLetters(List.of())).isZero();
    }

    @Test
    void countLongerCountsTheLongWords() {
        assertThat(WordStats.countLonger(List.of("cat", "bookshelf", "planet", "umbrella"), 6)).isEqualTo(2L);
        assertThat(WordStats.countLonger(List.of(), 6)).isZero();
    }
}
