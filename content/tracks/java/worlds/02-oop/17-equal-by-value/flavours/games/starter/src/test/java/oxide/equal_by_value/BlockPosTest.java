package oxide.equal_by_value;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class BlockPosTest {
    @Test
    void samePositionIsEqual() {
        BlockPos chest = new BlockPos(10, 64, -3);
        assertThat(chest.equals(new BlockPos(10, 64, -3))).as("same x, y, z").isTrue();
        assertThat(chest.equals(new BlockPos(11, 64, -3))).isFalse();
        assertThat(chest.equals(new BlockPos(10, 65, -3))).isFalse();
        assertThat(chest.equals(new BlockPos(10, 64, -4))).isFalse();
    }

    @Test
    void notEqualToOtherThings() {
        BlockPos chest = new BlockPos(10, 64, -3);
        assertThat(chest.equals(null)).isFalse();
        assertThat(chest.equals("10 64 -3")).as("a String is never a BlockPos").isFalse();
    }
}
