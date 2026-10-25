package oxide.method_references;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class NamesTest {
    private final List<Mod> pack = List.of(
            new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3"),
            new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0"),
            new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0"));

    @Test
    void nameByLambdaGivesTheName() {
        for (Mod mod : pack) {
            assertThat(Names.nameByLambda().apply(mod)).isEqualTo(mod.name());
        }
    }

    @Test
    void nameByReferenceMatchesTheLambda() {
        for (Mod mod : pack) {
            assertThat(Names.nameByReference().apply(mod)).isEqualTo(mod.name());
        }
    }
}
