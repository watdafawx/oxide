package oxide.sorting;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class PackSortTest {
    private static Mod mod(String name, String loader, int downloads) {
        return new Mod(name.toLowerCase(), name, loader, downloads, "1.0");
    }

    private final List<Mod> pack = List.of(
            mod("Sodium", "fabric", 9500),
            mod("JEI", "forge", 12000),
            mod("Iris", "fabric", 8200),
            mod("Create", "forge", 15000));

    @Test
    void sortsModsByLoaderThenName() {
        assertThat(PackSort.byLoaderThenName(pack)).extracting(Mod::name)
                .containsExactly("Iris", "Sodium", "Create", "JEI");
        assertThat(pack.get(0).name()).isEqualTo("Sodium");
    }

    @Test
    void sortsModsByDownloadsDescending() {
        assertThat(PackSort.byDownloadsDescending(pack)).extracting(Mod::name)
                .containsExactly("Create", "JEI", "Sodium", "Iris");
    }
}
