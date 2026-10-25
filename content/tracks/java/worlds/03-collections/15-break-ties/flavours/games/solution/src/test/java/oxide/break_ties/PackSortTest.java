package oxide.break_ties;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class PackSortTest {
    @Test
    void byLoaderThenNameBreaksTiesAtoZ() {
        List<Mod> pack = List.of(
                new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3"),
                new Mod("jei", "JEI", "forge", 12000, "15.2.0"),
                new Mod("iris", "Iris", "fabric", 8200, "1.7.0"),
                new Mod("create", "Create", "forge", 15000, "0.5.1"));
        assertThat(PackSort.byLoaderThenName(pack)).extracting(Mod::name)
                .containsExactly("Iris", "Sodium", "Create", "JEI");
        assertThat(pack.get(0).name()).isEqualTo("Sodium");
    }
}
