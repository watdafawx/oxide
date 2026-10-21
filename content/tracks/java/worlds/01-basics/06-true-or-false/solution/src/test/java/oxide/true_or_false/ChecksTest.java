package oxide.true_or_false;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ChecksTest {
    @Test
    void fullWhenCountReachesCapacity() {
        assertThat(Checks.isFull(20, 20)).isTrue();
        assertThat(Checks.isFull(25, 20)).isTrue();
        assertThat(Checks.isFull(19, 20)).isFalse();
    }

    @Test
    void entryNeedsRoomAndNoBan() {
        assertThat(Checks.canEnter(3, 20, false)).isTrue();
        assertThat(Checks.canEnter(3, 20, true)).isFalse();
        assertThat(Checks.canEnter(20, 20, false)).isFalse();
    }
}
