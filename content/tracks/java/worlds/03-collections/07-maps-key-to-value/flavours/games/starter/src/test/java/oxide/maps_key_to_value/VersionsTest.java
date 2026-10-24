package oxide.maps_key_to_value;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

class VersionsTest {
    @Test
    void latestVersionFallsBackToUnknown() {
        Map<String, String> latest = Map.of("sodium", "0.5.8", "iris", "1.7.0");
        assertThat(Versions.latestVersion(latest, "sodium")).isEqualTo("0.5.8");
        assertThat(Versions.latestVersion(latest, "quark")).isEqualTo("unknown");
    }

    @Test
    void countPerLoaderCountsEachLoader() {
        Map<String, Integer> counts = Versions.countPerLoader(List.of("fabric", "forge", "fabric", "neoforge", "fabric"));
        assertThat(counts).hasSize(3)
                .containsEntry("fabric", 3).containsEntry("forge", 1).containsEntry("neoforge", 1);
    }
}
