package oxide.largest_so_far;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class PeakFinderTest {
    @Test
    void peakFindsLargestSample() {
        assertThat(PeakFinder.peak(new int[] {3, 9, 2})).isEqualTo(9);
        assertThat(PeakFinder.peak(new int[] {-5, -2, -8})).isEqualTo(-2);
        assertThat(PeakFinder.peak(new int[] {4})).isEqualTo(4);
    }

    @Test
    void lowFindsSmallestSample() {
        assertThat(PeakFinder.low(new int[] {3, 9, 2})).isEqualTo(2);
        assertThat(PeakFinder.low(new int[] {5, 2, 8})).isEqualTo(2);
        assertThat(PeakFinder.low(new int[] {4})).isEqualTo(4);
    }
}
