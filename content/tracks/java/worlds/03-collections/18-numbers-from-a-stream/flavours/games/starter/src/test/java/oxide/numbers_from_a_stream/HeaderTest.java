package oxide.numbers_from_a_stream;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class HeaderTest {
    private final List<Mod> pack = List.of(
            new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3"),
            new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0"),
            new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0"));

    @Test
    void totalDownloadsAddsEveryCount() {
        assertThat(Header.totalDownloads(pack)).isEqualTo(29700);
        assertThat(Header.totalDownloads(List.of())).isZero();
    }

    @Test
    void countOnCountsTheLoadersMods() {
        assertThat(Header.countOn(pack, "fabric")).isEqualTo(2L);
        assertThat(Header.countOn(pack, "quilt")).isZero();
    }
}
