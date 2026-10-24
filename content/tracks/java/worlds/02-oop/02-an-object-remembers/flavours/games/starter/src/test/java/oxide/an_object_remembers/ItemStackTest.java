package oxide.an_object_remembers;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ItemStackTest {
    @Test
    void newStackIsEmpty() {
        assertThat(new ItemStack().count()).isZero();
    }

    @Test
    void growAddsOne() {
        ItemStack slot1 = new ItemStack();
        ItemStack slot2 = new ItemStack();
        slot1.grow();
        slot1.grow();
        assertThat(slot1.count()).isEqualTo(2);
        assertThat(slot2.count()).as("the other stack keeps its own count").isZero();
    }
}
