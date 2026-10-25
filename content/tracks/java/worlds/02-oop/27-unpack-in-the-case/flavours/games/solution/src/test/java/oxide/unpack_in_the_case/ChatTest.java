package oxide.unpack_in_the_case;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ChatTest {
    @Test
    void recordPatternReadsBlockCoordinates() {
        assertThat(Chat.where(new BlockBroken("Alex", "stone", new BlockPos(10, 64, -3)))).isEqualTo("10 64 -3");
    }

    @Test
    void otherEventsAreNowhere() {
        assertThat(Chat.where(new PlayerJoined("Alex"))).isEqualTo("nowhere");
        assertThat(Chat.where(new ItemDropped("diamond", 3))).isEqualTo("nowhere");
    }
}
