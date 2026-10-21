package oxide.an_array_of_values;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class HourlySamplesTest {
    @Test
    void firstSampleIsIndexZero() {
        assertThat(HourlySamples.firstSample(new int[] {7, 3, 9})).isEqualTo(7);
        assertThat(HourlySamples.firstSample(new int[] {4})).isEqualTo(4);
    }

    @Test
    void latestSampleIsTheLast() {
        assertThat(HourlySamples.latestSample(new int[] {7, 3, 9})).isEqualTo(9);
        assertThat(HourlySamples.latestSample(new int[] {4})).isEqualTo(4);
    }

    @Test
    void emptyDayHas24Zeros() {
        assertThat(HourlySamples.emptyDay()).hasSize(24).containsOnly(0);
    }
}
