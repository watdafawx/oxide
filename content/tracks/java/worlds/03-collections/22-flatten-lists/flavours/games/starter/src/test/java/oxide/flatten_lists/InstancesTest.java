package oxide.flatten_lists;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class InstancesTest {
    private static final Mod SODIUM = new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3");
    private static final Mod IRIS = new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0");
    private static final Mod JEI = new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0");

    @Test
    void allModsFlattensEveryInstance() {
        assertThat(Instances.allMods(List.of(List.of(SODIUM, IRIS), List.of(), List.of(JEI))))
                .containsExactly(SODIUM, IRIS, JEI);
        assertThat(Instances.allMods(List.of())).isEmpty();
    }
}
