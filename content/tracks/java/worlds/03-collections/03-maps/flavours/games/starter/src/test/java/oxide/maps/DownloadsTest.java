package oxide.maps;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

class DownloadsTest {
    @Test
    void countsDownloadsPerModId() {
        Map<String, Integer> counts = Downloads.countDownloads(List.of("sodium", "iris", "lithium", "sodium", "iris", "sodium"));
        assertThat(counts).hasSize(3);
        assertThat(counts).containsEntry("sodium", 3).containsEntry("iris", 2).containsEntry("lithium", 1);
    }

    @Test
    void indexesModNamesByFirstLetter() {
        Map<Character, List<String>> index = Downloads.indexByLetter(List.of("sodium", "spark", "iris", "starlight"));
        assertThat(index).hasSize(2);
        assertThat(index.get('s')).containsExactly("sodium", "spark", "starlight");
        assertThat(index.get('i')).containsExactly("iris");
    }
}
