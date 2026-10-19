package oxide.maps;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

class MapsTest {
    @Test
    void countsWords() {
        Map<String, Integer> counts = Maps.countWords(List.of("to", "be", "or", "not", "to", "be"));
        assertThat(counts).hasSize(4);
        assertThat(counts).containsEntry("to", 2).containsEntry("be", 2).containsEntry("or", 1).containsEntry("not", 1);
    }

    @Test
    void computeIfAbsentGroupsByFirstLetter() {
        Map<Character, List<String>> groups = Maps.groupByFirstLetter(List.of("apple", "avocado", "banana", "apricot"));
        assertThat(groups).hasSize(2);
        assertThat(groups.get('a')).containsExactly("apple", "avocado", "apricot");
        assertThat(groups.get('b')).containsExactly("banana");
    }
}
