package oxide.private_means_mine;

import static org.assertj.core.api.Assertions.assertThat;

import java.lang.reflect.Modifier;
import org.junit.jupiter.api.Test;

class ItemStackTest {
    @Test
    void countFieldIsPrivate() throws Exception {
        int modifiers = ItemStack.class.getDeclaredField("count").getModifiers();
        assertThat(Modifier.isPrivate(modifiers)).as("write `private int count;`").isTrue();
    }

    @Test
    void shrinkNeverGoesBelowZero() {
        ItemStack logs = new ItemStack();
        logs.grow(17);
        assertThat(logs.shrink(4)).isTrue();
        assertThat(logs.count()).isEqualTo(13);
        assertThat(logs.shrink(14)).as("only 13 in the stack").isFalse();
        assertThat(logs.count()).as("a refused shrink changes nothing").isEqualTo(13);
        assertThat(logs.shrink(13)).as("removing exactly what is there is fine").isTrue();
        assertThat(logs.count()).isZero();
    }
}
