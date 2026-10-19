package oxide.sets;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.Set;
import org.junit.jupiter.api.Test;

class SetsTest {
    @Test
    void uniqueWordsDropsDuplicates() {
        Set<String> words = Sets.uniqueWords("The cat saw the other cat");
        assertThat(words).hasSize(4);
        assertThat(words).containsExactlyInAnyOrder("the", "cat", "saw", "other");
        assertThat(words.contains("cat")).isTrue();
    }
}
