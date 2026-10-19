package oxide.flatmap_reduce;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class FlatTest {
    @Test
    void flattensNestedLists() {
        assertThat(Flat.flatten(List.of(List.of(1, 2), List.of(3), List.of(), List.of(4, 5)))).containsExactly(1, 2, 3, 4, 5);
        assertThat(Flat.flatten(List.of())).isEmpty();
    }

    @Test
    void reducesProduct() {
        assertThat(Flat.product(5)).isEqualTo(120);
        assertThat(Flat.product(1)).isEqualTo(1);
        assertThat(Flat.product(0)).isEqualTo(1);
    }

    @Test
    void maxOfEmptyIsEmpty() {
        assertThat(Flat.largest(List.of())).isEmpty();
        assertThat(Flat.largest(List.of(3, 9, 4))).hasValue(9);
    }
}
