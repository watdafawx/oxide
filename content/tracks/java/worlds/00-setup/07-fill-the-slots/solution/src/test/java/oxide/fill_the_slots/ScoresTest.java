package oxide.fill_the_slots;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ScoresTest {
    @Test
    void scoreLineHasThreeSlots() {
        assertThat(Scores.scoreLine("Ada", 7, 10)).isEqualTo("Ada: 7/10 correct");
        assertThat(Scores.scoreLine("Bo", 0, 5)).isEqualTo("Bo: 0/5 correct");
    }

    @Test
    void itemLineHasTwoSlots() {
        assertThat(Scores.itemLine("apple", 3)).isEqualTo("3 x apple");
    }
}
