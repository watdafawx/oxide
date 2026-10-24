package oxide.walk_a_map;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.Map;
import org.junit.jupiter.api.Test;

class PopularityTest {
    private final Map<String, Integer> downloads = Map.of("sodium", 9500, "jei", 12000, "iris", 8200, "quark", 6000);

    @Test
    void mostDownloadedHasTheMostDownloads() {
        assertThat(Popularity.mostDownloaded(downloads)).isEqualTo("jei");
        assertThat(Popularity.mostDownloaded(Map.of("solo", 0))).isEqualTo("solo");
    }

    @Test
    void popularListsEveryIdAtOrAboveMin() {
        assertThat(Popularity.popular(downloads, 9500)).containsExactlyInAnyOrder("sodium", "jei");
        assertThat(Popularity.popular(downloads, 50000)).isEmpty();
    }
}
