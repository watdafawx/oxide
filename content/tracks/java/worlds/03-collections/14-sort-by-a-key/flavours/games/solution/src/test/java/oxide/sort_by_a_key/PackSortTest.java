package oxide.sort_by_a_key;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class PackSortTest {
    private final List<Mod> pack = List.of(
            new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3"),
            new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0"),
            new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0"),
            new Mod("create", "Create", "forge", 15000, "0.5.1"));

    @Test
    void byNameSortsModsAtoZ() {
        assertThat(PackSort.byName(pack)).extracting(Mod::name)
                .containsExactly("Create", "Iris Shaders", "Just Enough Items", "Sodium");
        assertThat(pack.get(0).name()).isEqualTo("Sodium");
    }

    @Test
    void byDownloadsDescendingPutsTheMostPopularFirst() {
        assertThat(PackSort.byDownloadsDescending(pack)).extracting(Mod::name)
                .containsExactly("Create", "Just Enough Items", "Sodium", "Iris Shaders");
    }
}
