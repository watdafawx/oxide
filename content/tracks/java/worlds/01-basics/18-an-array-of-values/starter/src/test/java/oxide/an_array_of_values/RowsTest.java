package oxide.an_array_of_values;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class RowsTest {
    @Test
    void firstIsIndexZero() {
        assertThat(Rows.first(new int[] {7, 3, 9})).isEqualTo(7);
        assertThat(Rows.first(new int[] {4})).isEqualTo(4);
    }

    @Test
    void lastIsTheEnd() {
        assertThat(Rows.last(new int[] {7, 3, 9})).isEqualTo(9);
        assertThat(Rows.last(new int[] {4})).isEqualTo(4);
    }

    @Test
    void zerosHasTheGivenSize() {
        assertThat(Rows.zeros(5)).hasSize(5).containsOnly(0);
        assertThat(Rows.zeros(0)).isEmpty();
    }
}
