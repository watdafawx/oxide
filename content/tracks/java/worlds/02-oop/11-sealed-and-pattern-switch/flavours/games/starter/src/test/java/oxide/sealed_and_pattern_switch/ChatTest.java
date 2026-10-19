package oxide.sealed_and_pattern_switch;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ChatTest {
    @Test
    void chatLineForEveryEventType() {
        assertThat(Chat.line(new PlayerJoined("Alex"))).isEqualTo("Alex joined the game");
        assertThat(Chat.line(new BlockBroken("Alex", "stone", new BlockPos(1, 2, 3)))).isEqualTo("Alex broke stone");
        assertThat(Chat.line(new ItemDropped("diamond", 3))).isEqualTo("3 x diamond dropped");
        assertThat(Event.class.isSealed()).isTrue();
    }

    @Test
    void emptyDropGetsItsOwnLine() {
        assertThat(Chat.line(new ItemDropped("diamond", 0))).isEqualTo("nothing dropped");
        assertThat(Chat.line(new ItemDropped("diamond", 1))).isEqualTo("1 x diamond dropped");
    }

    @Test
    void recordPatternReadsBlockCoordinates() {
        assertThat(Chat.where(new BlockBroken("Alex", "stone", new BlockPos(10, 64, -3)))).isEqualTo("10 64 -3");
        assertThat(Chat.where(new PlayerJoined("Alex"))).isEqualTo("nowhere");
        assertThat(Chat.where(new ItemDropped("diamond", 3))).isEqualTo("nowhere");
    }
}
