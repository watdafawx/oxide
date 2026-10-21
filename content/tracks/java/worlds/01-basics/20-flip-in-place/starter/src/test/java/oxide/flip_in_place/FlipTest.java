package oxide.flip_in_place;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class FlipTest {
    @Test
    void swapEndsSwapsFirstAndLast() {
        int[] values = {1, 2, 3, 4};
        Flip.swapEnds(values);
        assertThat(values).containsExactly(4, 2, 3, 1);
    }

    @Test
    void reverseInPlaceMutates() {
        int[] odd = {1, 2, 3, 4, 5};
        Flip.reverseInPlace(odd);
        assertThat(odd).containsExactly(5, 4, 3, 2, 1);

        int[] even = {1, 2, 3, 4};
        Flip.reverseInPlace(even);
        assertThat(even).containsExactly(4, 3, 2, 1);
    }
}
