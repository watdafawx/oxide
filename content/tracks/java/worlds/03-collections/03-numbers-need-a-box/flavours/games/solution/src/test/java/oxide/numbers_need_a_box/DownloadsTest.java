package oxide.numbers_need_a_box;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class DownloadsTest {
    @Test
    void totalAddsEveryDownloadCount() {
        assertThat(Downloads.totalDownloads(List.of(9500, 8200, 12000))).isEqualTo(29700);
        assertThat(Downloads.totalDownloads(List.of(7100))).isEqualTo(7100);
    }

    @Test
    void withBonusAddsToEachCount() {
        assertThat(Downloads.withBonus(List.of(9500, 8200), 100)).containsExactly(9600, 8300);
        assertThat(Downloads.withBonus(List.of(), 100)).isEmpty();
    }
}
