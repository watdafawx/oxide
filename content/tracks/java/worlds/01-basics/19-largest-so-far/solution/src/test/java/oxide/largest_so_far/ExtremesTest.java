package oxide.largest_so_far;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ExtremesTest {
    @Test
    void maxFindsLargest() {
        assertThat(Extremes.max(new int[] {3, 9, 2})).isEqualTo(9);
        assertThat(Extremes.max(new int[] {-5, -2, -8})).isEqualTo(-2);
        assertThat(Extremes.max(new int[] {4})).isEqualTo(4);
    }

    @Test
    void minFindsSmallest() {
        assertThat(Extremes.min(new int[] {3, 9, 2})).isEqualTo(2);
        assertThat(Extremes.min(new int[] {5, 2, 8})).isEqualTo(2);
        assertThat(Extremes.min(new int[] {4})).isEqualTo(4);
    }
}
