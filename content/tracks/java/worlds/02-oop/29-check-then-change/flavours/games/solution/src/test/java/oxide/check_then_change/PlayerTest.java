package oxide.check_then_change;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import org.junit.jupiter.api.Test;

class PlayerTest {
    @Test
    void overspendingThrowsAndChangesNothing() {
        Player alex = new Player("Alex", 30);
        assertThatThrownBy(() -> alex.spendLevels(31)).isInstanceOf(NotEnoughLevelsException.class);
        assertThat(alex.levels()).as("a refused enchant changes nothing").isEqualTo(30);
        alex.spendLevels(30);
        assertThat(alex.levels()).as("spending exactly what is there is fine").isZero();
    }

    @Test
    void refusedGiftChangesNeitherPlayer() {
        Player alex = new Player("Alex", 10);
        Player steve = new Player("Steve", 0);
        assertThatThrownBy(() -> alex.giveLevels(steve, 50)).isInstanceOf(NotEnoughLevelsException.class);
        assertThat(alex.levels()).isEqualTo(10);
        assertThat(steve.levels()).as("no levels appear from nowhere").isZero();
        alex.giveLevels(steve, 4);
        assertThat(alex.levels()).isEqualTo(6);
        assertThat(steve.levels()).isEqualTo(4);
    }
}
