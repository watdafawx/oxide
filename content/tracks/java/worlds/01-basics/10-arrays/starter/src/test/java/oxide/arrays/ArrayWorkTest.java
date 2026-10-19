package oxide.arrays;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ArrayWorkTest {
    @Test
    void maxFindsLargest() {
        assertThat(ArrayWork.max(new int[] {3, 9, 2})).isEqualTo(9);
        assertThat(ArrayWork.max(new int[] {-5, -2, -8})).isEqualTo(-2);
        assertThat(ArrayWork.max(new int[] {4})).isEqualTo(4);
    }

    @Test
    void reverseInPlaceMutates() {
        int[] odd = {1, 2, 3, 4, 5};
        ArrayWork.reverseInPlace(odd);
        assertThat(odd).containsExactly(5, 4, 3, 2, 1);

        int[] even = {1, 2, 3, 4};
        ArrayWork.reverseInPlace(even);
        assertThat(even).containsExactly(4, 3, 2, 1);
    }

    @Test
    void transposeSwapsRowsAndCols() {
        int[][] grid = {{1, 2, 3}, {4, 5, 6}};
        assertThat(ArrayWork.transpose(grid)).isDeepEqualTo(new int[][] {{1, 4}, {2, 5}, {3, 6}});
    }
}
