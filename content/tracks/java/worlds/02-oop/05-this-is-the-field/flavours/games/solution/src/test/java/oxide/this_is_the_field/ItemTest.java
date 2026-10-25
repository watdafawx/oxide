package oxide.this_is_the_field;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ItemTest {
    @Test
    void idIsStored() {
        assertThat(new Item("stick", 64).id()).as("the field, not the parameter, must get the value").isEqualTo("stick");
    }

    @Test
    void maxStackIsStored() {
        assertThat(new Item("ender_pearl", 16).maxStack()).as("the field, not the parameter, must get the value").isEqualTo(16);
    }
}
