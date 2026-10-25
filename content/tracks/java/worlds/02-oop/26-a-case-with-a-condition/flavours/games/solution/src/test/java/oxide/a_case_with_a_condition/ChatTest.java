package oxide.a_case_with_a_condition;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ChatTest {
    @Test
    void chatLineForEveryEventType() {
        assertThat(Chat.line(new PlayerJoined("Alex"))).isEqualTo("Alex joined the game");
        assertThat(Chat.line(new BlockBroken("Alex", "stone", new BlockPos(1, 64, 2)))).isEqualTo("Alex broke stone");
        assertThat(Chat.line(new ItemDropped("diamond", 3))).isEqualTo("3 x diamond dropped");
    }

    @Test
    void emptyDropGetsItsOwnLine() {
        assertThat(Chat.line(new ItemDropped("diamond", 0))).isEqualTo("nothing dropped");
    }
}
