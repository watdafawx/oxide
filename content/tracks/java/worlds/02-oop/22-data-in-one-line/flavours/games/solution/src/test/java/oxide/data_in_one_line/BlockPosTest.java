package oxide.data_in_one_line;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class BlockPosTest {
    @Test
    void blockPosIsARecord() {
        assertThat(BlockPos.class.isRecord()).as("declare BlockPos with `record`").isTrue();
        BlockPos pos = new BlockPos(10, 64, -3);
        assertThat(pos.x()).isEqualTo(10);
        assertThat(pos.y()).isEqualTo(64);
        assertThat(pos.z()).isEqualTo(-3);
    }

    @Test
    void blockPosEqualsByValue() {
        BlockPos a = new BlockPos(10, 64, -3);
        assertThat(a.equals(new BlockPos(10, 64, -3))).isTrue();
        assertThat(a.equals(new BlockPos(10, 64, -4))).isFalse();
        assertThat(a.hashCode()).isEqualTo(new BlockPos(10, 64, -3).hashCode());
        assertThat(a.toString()).isEqualTo("BlockPos[x=10, y=64, z=-3]");
    }
}
