package oxide.lambdas_and_method_refs;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.Map;
import java.util.function.Predicate;
import org.junit.jupiter.api.Test;

class UpdatesTest {
    private static final Mod SODIUM = new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3");
    private static final Mod IRIS = new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0");
    private static final Mod JEI = new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0");

    @Test
    void appliesOutdatedPredicate() {
        Map<String, String> latest = Map.of("sodium", "0.5.8", "iris", "1.7.0", "jei", "15.3.0");
        Predicate<Mod> outdated = Updates.isOutdated(latest);
        assertThat(outdated.test(SODIUM)).isTrue();
        assertThat(outdated.test(IRIS)).isFalse();
        assertThat(outdated.test(JEI)).isTrue();
    }

    @Test
    void methodRefEqualsLambdaForModNames() {
        for (Mod mod : List.of(SODIUM, IRIS, JEI)) {
            assertThat(Updates.nameByLambda().apply(mod)).isEqualTo(mod.name());
            assertThat(Updates.nameByMethodRef().apply(mod)).isEqualTo(Updates.nameByLambda().apply(mod));
        }
    }
}
