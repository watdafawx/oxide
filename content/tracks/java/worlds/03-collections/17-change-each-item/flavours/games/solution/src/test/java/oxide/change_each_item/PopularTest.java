package oxide.change_each_item;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class PopularTest {
    private final List<Mod> ranked = List.of(
            new Mod("create", "Create", "forge", 15000, "0.5.1"),
            new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0"),
            new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3"),
            new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0"));

    @Test
    void namesTurnsModsIntoNames() {
        assertThat(Popular.names(ranked)).containsExactly("Create", "Just Enough Items", "Sodium", "Iris Shaders");
    }

    @Test
    void topThreeKeepsTheFirstThreeNames() {
        assertThat(Popular.topThree(ranked)).containsExactly("Create", "Just Enough Items", "Sodium");
        assertThat(Popular.topThree(ranked.subList(0, 2))).containsExactly("Create", "Just Enough Items");
    }
}
