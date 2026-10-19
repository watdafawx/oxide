package oxide.arrays;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class PlayerHistoryTest {
    @Test
    void peakFindsLargestSample() {
        assertThat(PlayerHistory.peak(new int[] {3, 9, 2})).isEqualTo(9);
        assertThat(PlayerHistory.peak(new int[] {-5, -2, -8})).isEqualTo(-2);
        assertThat(PlayerHistory.peak(new int[] {4})).isEqualTo(4);
    }

    @Test
    void newestFirstReversesInPlace() {
        int[] odd = {1, 2, 3, 4, 5};
        PlayerHistory.newestFirst(odd);
        assertThat(odd).containsExactly(5, 4, 3, 2, 1);

        int[] even = {1, 2, 3, 4};
        PlayerHistory.newestFirst(even);
        assertThat(even).containsExactly(4, 3, 2, 1);
    }

    @Test
    void hourlyGridSwapsRowsAndCols() {
        int[][] serversByHour = {{1, 2, 3}, {4, 5, 6}};
        assertThat(PlayerHistory.byHour(serversByHour)).isDeepEqualTo(new int[][] {{1, 4}, {2, 5}, {3, 6}});
    }
}
