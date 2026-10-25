package oxide.split_in_two;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

class UpdateScreenTest {
    private static final Mod SODIUM = new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3");
    private static final Mod IRIS = new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0");
    private static final Mod JEI = new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0");
    private static final Map<String, String> LATEST = Map.of("sodium", "0.5.8", "iris", "1.7.0", "jei", "15.3.0");

    @Test
    void byOutdatedSplitsThePack() {
        Map<Boolean, List<Mod>> piles = UpdateScreen.byOutdated(List.of(SODIUM, JEI, IRIS), LATEST);
        assertThat(piles.get(true)).containsExactly(SODIUM, JEI);
        assertThat(piles.get(false)).containsExactly(IRIS);
        assertThat(UpdateScreen.byOutdated(List.of(IRIS), LATEST).get(true)).isEmpty();
    }
}
