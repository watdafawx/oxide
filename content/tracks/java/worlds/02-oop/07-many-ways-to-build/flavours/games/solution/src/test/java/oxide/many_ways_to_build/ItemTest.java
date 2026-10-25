package oxide.many_ways_to_build;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ItemTest {
    @Test
    void airIsTheDefaultItem() {
        Item air = new Item();
        assertThat(air.id()).isEqualTo("air");
        assertThat(air.maxStack()).isEqualTo(64);
    }

    @Test
    void shortConstructorReusesFullOne() {
        Item stick = new Item("  Stick ");
        assertThat(stick.id()).as("id is cleaned by the full constructor").isEqualTo("stick");
        assertThat(stick.maxStack()).isEqualTo(64);
        assertThat(new Item("ender_pearl", 16).maxStack()).isEqualTo(16);
        assertThat(new Item("bucket", 0).maxStack()).as("at least 1").isEqualTo(1);
    }
}
