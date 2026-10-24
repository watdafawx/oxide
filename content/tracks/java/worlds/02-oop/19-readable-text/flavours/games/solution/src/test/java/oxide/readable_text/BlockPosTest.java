package oxide.readable_text;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class BlockPosTest {
    @Test
    void toStringReadsLikeACoordinate() {
        assertThat(new BlockPos(10, 64, -3).toString()).isEqualTo("(10, 64, -3)");
        assertThat(new BlockPos(0, -60, 0).toString()).isEqualTo("(0, -60, 0)");
    }

    @Test
    void joiningTextUsesToString() {
        BlockPos chest = new BlockPos(1, 70, 2);
        assertThat("Chest at " + chest).isEqualTo("Chest at (1, 70, 2)");
    }
}
