package oxide.reject_bad_values;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import org.junit.jupiter.api.Test;

class BlockPosTest {
    @Test
    void yOutsideTheWorldIsRejected() {
        assertThatThrownBy(() -> new BlockPos(0, 320, 0)).isInstanceOf(IllegalArgumentException.class);
        assertThatThrownBy(() -> new BlockPos(0, -65, 0)).isInstanceOf(IllegalArgumentException.class);
        assertThat(new BlockPos(0, -64, 0).y()).as("bedrock floor is fine").isEqualTo(-64);
        assertThat(new BlockPos(0, 319, 0).y()).as("build limit is fine").isEqualTo(319);
    }

    @Test
    void beyondTheBorderIsRejected() {
        assertThatThrownBy(() -> new BlockPos(30_000_001, 64, 0)).isInstanceOf(IllegalArgumentException.class);
        assertThatThrownBy(() -> new BlockPos(0, 64, -30_000_001)).isInstanceOf(IllegalArgumentException.class);
        assertThat(new BlockPos(30_000_000, 64, -30_000_000).x()).isEqualTo(30_000_000);
    }
}
