package oxide.true_or_false;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class GateTest {
    @Test
    void fullWhenOnlineReachesMax() {
        assertThat(Gate.isFull(20, 20)).isTrue();
        assertThat(Gate.isFull(25, 20)).isTrue();
        assertThat(Gate.isFull(19, 20)).isFalse();
    }

    @Test
    void joinNeedsRoomAndNoBan() {
        assertThat(Gate.canJoin(3, 20, false)).isTrue();
        assertThat(Gate.canJoin(3, 20, true)).isFalse();
        assertThat(Gate.canJoin(20, 20, false)).isFalse();
    }
}
