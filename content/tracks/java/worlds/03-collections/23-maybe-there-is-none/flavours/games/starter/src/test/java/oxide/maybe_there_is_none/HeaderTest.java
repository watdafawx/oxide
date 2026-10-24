package oxide.maybe_there_is_none;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class HeaderTest {
    private static final Mod SODIUM = new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3");
    private static final Mod JEI = new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0");

    @Test
    void mostDownloadedIsEmptyForNoMods() {
        assertThat(Header.mostDownloaded(List.of(SODIUM, JEI))).hasValue(JEI);
        assertThat(Header.mostDownloaded(List.of())).isEmpty();
    }

    @Test
    void headlineFallsBackToNone() {
        assertThat(Header.headline(List.of(JEI, SODIUM))).isEqualTo("Just Enough Items");
        assertThat(Header.headline(List.of())).isEqualTo("none");
    }
}
