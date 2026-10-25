package oxide.ask_for_a_list;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.ArrayList;
import java.util.List;
import org.junit.jupiter.api.Test;

class WordsTest {
    @Test
    void longestWordFromAnyList() {
        List<String> fixed = List.of("oak", "birch", "spruce", "acacia");
        assertThat(Words.longestWord(fixed)).isEqualTo("spruce");
        List<String> growing = new ArrayList<>();
        growing.add("ash");
        growing.add("willow");
        assertThat(Words.longestWord(growing)).isEqualTo("willow");
    }
}
