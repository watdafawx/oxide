package oxide.classes_and_fields;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ItemStackTest {
    @Test
    void newStackIsEmpty() {
        assertThat(new ItemStack().count()).isZero();
    }

    @Test
    void growAddsOne() {
        ItemStack logs = new ItemStack();
        logs.grow();
        logs.grow();
        assertThat(logs.count()).isEqualTo(2);
        assertThat(new ItemStack().count()).as("other stacks are unaffected").isZero();
    }
}
