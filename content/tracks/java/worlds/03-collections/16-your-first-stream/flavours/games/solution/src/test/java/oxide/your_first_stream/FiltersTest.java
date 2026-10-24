package oxide.your_first_stream;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

class FiltersTest {
    private static final Mod SODIUM = new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3");
    private static final Mod IRIS = new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0");
    private static final Mod JEI = new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0");
    private final List<Mod> pack = List.of(SODIUM, IRIS, JEI);

    @Test
    void outdatedKeepsModsBehindLatest() {
        Map<String, String> latest = Map.of("sodium", "0.5.8", "iris", "1.7.0", "jei", "15.3.0");
        assertThat(Filters.outdated(pack, latest)).containsExactly(SODIUM, JEI);
        assertThat(pack).hasSize(3);
    }

    @Test
    void onLoaderKeepsModsForThatLoader() {
        assertThat(Filters.onLoader(pack, "fabric")).containsExactly(SODIUM, IRIS);
        assertThat(Filters.onLoader(pack, "quilt")).isEmpty();
    }
}
