package oxide.a_rule_as_a_value;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.Map;
import java.util.function.Predicate;
import org.junit.jupiter.api.Test;

class UpdatesTest {
    private static final Mod SODIUM = new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3");
    private static final Mod IRIS = new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0");
    private static final Mod JEI = new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0");
    private static final Map<String, String> LATEST = Map.of("sodium", "0.5.8", "iris", "1.7.0", "jei", "15.3.0");

    @Test
    void isOutdatedBuildsTheRule() {
        Predicate<Mod> outdated = Updates.isOutdated(LATEST);
        assertThat(outdated.test(SODIUM)).isTrue();
        assertThat(outdated.test(IRIS)).isFalse();
        assertThat(outdated.test(JEI)).isTrue();
    }

    @Test
    void upToDateRemovesOutdatedMods() {
        assertThat(Updates.upToDate(List.of(SODIUM, IRIS, JEI), LATEST)).containsExactly(IRIS);
        assertThat(Updates.upToDate(List.of(IRIS), LATEST)).containsExactly(IRIS);
    }
}
