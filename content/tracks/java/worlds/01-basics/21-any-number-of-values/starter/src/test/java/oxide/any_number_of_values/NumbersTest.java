package oxide.any_number_of_values;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class NumbersTest {
    @Test
    void howManyCountsTheNumbers() {
        assertThat(Numbers.howMany()).isEqualTo(0);
        assertThat(Numbers.howMany(7)).isEqualTo(1);
        assertThat(Numbers.howMany(3, 8, 0)).isEqualTo(3);
    }

    @Test
    void sumAddsAnyNumber() {
        assertThat(Numbers.sum()).isEqualTo(0);
        assertThat(Numbers.sum(5)).isEqualTo(5);
        assertThat(Numbers.sum(1, 2, 3)).isEqualTo(6);
        assertThat(Numbers.sum(new int[] {4, 4})).isEqualTo(8);
    }
}
