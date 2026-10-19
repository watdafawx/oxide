package oxide.constructors_and_this;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ItemTest {
    @Test
    void airIsTheDefaultItem() {
        Item air = new Item();
        assertThat(air.name()).isEqualTo("air");
        assertThat(air.maxStack()).isEqualTo(64);
    }

    @Test
    void shortConstructorReusesFullOne() {
        Item stick = new Item("  Stick ");
        assertThat(stick.name()).as("name is cleaned by the full constructor").isEqualTo("stick");
        assertThat(stick.maxStack()).isEqualTo(64);
        assertThat(new Item("Ender_Pearl", 16).maxStack()).isEqualTo(16);
        assertThat(new Item("sword", 0).maxStack()).as("never below 1").isEqualTo(1);
        assertThat(new Item("dirt", 99).maxStack()).as("never above 64").isEqualTo(64);
    }
}
