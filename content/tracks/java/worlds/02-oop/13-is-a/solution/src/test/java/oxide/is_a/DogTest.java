package oxide.is_a;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class DogTest {
    @Test
    void dogOverridesSound() {
        Animal rex = new Dog("Rex");
        assertThat(rex.sound()).isEqualTo("Woof");
        assertThat(rex.name()).as("Dog got name() from Animal").isEqualTo("Rex");
        assertThat(new Animal("Generic").sound()).as("Animal itself is unchanged").isEqualTo("...");
    }

    @Test
    void parentCodeUsesTheDogsSound() {
        assertThat(new Dog("Rex").describe()).isEqualTo("Rex says Woof");
        assertThat(new Animal("Generic").describe()).isEqualTo("Generic says ...");
    }
}
