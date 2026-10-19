package oxide.stream_basics;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

class OutdatedTest {
    private final List<Mod> pack = List.of(
            new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3"),
            new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0"),
            new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0"));

    @Test
    void filtersOutdatedModNames() {
        Map<String, String> latest = Map.of("sodium", "0.5.8", "iris", "1.7.0", "jei", "15.3.0");
        assertThat(Outdated.outdatedNames(pack, latest)).containsExactly("Sodium", "Just Enough Items");
        assertThat(Outdated.outdatedNames(pack, Map.of("sodium", "0.5.3", "iris", "1.7.0", "jei", "15.2.0"))).isEmpty();
    }

    @Test
    void sumsDownloadsWithMapToInt() {
        assertThat(Outdated.totalDownloads(pack)).isEqualTo(29700);
        assertThat(Outdated.totalDownloads(List.of())).isZero();
    }
}
