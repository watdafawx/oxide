package oxide.hash_matches_equals;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.HashSet;
import java.util.Set;
import org.junit.jupiter.api.Test;

class BlockPosTest {
    @Test
    void equalPositionsHaveEqualHashCodes() {
        BlockPos a = new BlockPos(10, 64, -3);
        BlockPos b = new BlockPos(10, 64, -3);
        assertThat(a.equals(b)).isTrue();
        assertThat(a.hashCode()).as("equal objects must have equal hash codes").isEqualTo(b.hashCode());
    }

    @Test
    void hashSetFindsEqualPosition() {
        Set<BlockPos> torches = new HashSet<>();
        torches.add(new BlockPos(10, 64, -3));
        assertThat(torches.contains(new BlockPos(10, 64, -3))).as("the set finds the same spot").isTrue();
        torches.add(new BlockPos(10, 64, -3));
        assertThat(torches).as("a set holds a position once").hasSize(1);
    }
}
