package oxide.flatmap_reduce;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class InstancesTest {
    private static final Mod SODIUM = new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3");
    private static final Mod IRIS = new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0");
    private static final Mod JEI = new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0");

    @Test
    void flattensModsFromAllInstances() {
        assertThat(Instances.allMods(List.of(List.of(SODIUM, IRIS), List.of(), List.of(JEI))))
                .containsExactly(SODIUM, IRIS, JEI);
        assertThat(Instances.allMods(List.of())).isEmpty();
    }

    @Test
    void reducesDownloadsToTotal() {
        assertThat(Instances.totalDownloads(List.of(SODIUM, IRIS, JEI))).isEqualTo(29700);
        assertThat(Instances.totalDownloads(List.of())).isZero();
    }

    @Test
    void mostDownloadedOfNoModsIsEmpty() {
        assertThat(Instances.mostDownloaded(List.of())).isEmpty();
        assertThat(Instances.mostDownloaded(List.of(SODIUM, JEI, IRIS))).hasValue(JEI);
    }
}
