package oxide.integer_division;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class BoxesTest {
    @Test
    void fullBoxesCountsWholeBoxes() {
        assertThat(Boxes.fullBoxes(17, 5)).isEqualTo(3);
        assertThat(Boxes.fullBoxes(10, 5)).isEqualTo(2);
        assertThat(Boxes.fullBoxes(4, 5)).isEqualTo(0);
    }

    @Test
    void itemsLeftAreTheRemainder() {
        assertThat(Boxes.itemsLeft(17, 5)).isEqualTo(2);
        assertThat(Boxes.itemsLeft(10, 5)).isEqualTo(0);
    }
}
