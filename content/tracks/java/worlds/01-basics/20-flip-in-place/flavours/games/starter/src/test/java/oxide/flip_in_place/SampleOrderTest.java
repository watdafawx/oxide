package oxide.flip_in_place;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class SampleOrderTest {
    @Test
    void swapEndsSwapsFirstAndLast() {
        int[] values = {1, 2, 3, 4};
        SampleOrder.swapEnds(values);
        assertThat(values).containsExactly(4, 2, 3, 1);
    }

    @Test
    void newestFirstReversesInPlace() {
        int[] odd = {1, 2, 3, 4, 5};
        SampleOrder.newestFirst(odd);
        assertThat(odd).containsExactly(5, 4, 3, 2, 1);

        int[] even = {1, 2, 3, 4};
        SampleOrder.newestFirst(even);
        assertThat(even).containsExactly(4, 3, 2, 1);
    }
}
