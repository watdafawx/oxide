package oxide.count_with_merge;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class LoaderPageTest {
    private final List<Mod> pack = List.of(
            new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3"),
            new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0"),
            new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0"),
            new Mod("lithium", "Lithium", "fabric", 7100, "0.12.0"));

    @Test
    void modsPerLoaderCountsEachMod() {
        assertThat(LoaderPage.modsPerLoader(pack)).hasSize(2).containsEntry("fabric", 3).containsEntry("forge", 1);
        assertThat(LoaderPage.modsPerLoader(List.of())).isEmpty();
    }

    @Test
    void downloadsPerLoaderAddsTheCounts() {
        assertThat(LoaderPage.downloadsPerLoader(pack)).hasSize(2)
                .containsEntry("fabric", 24800).containsEntry("forge", 12000);
    }
}
