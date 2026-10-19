package oxide.inheritance_and_super;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class DogTest {
    @Test
    void dogOverridesSound() {
        Animal rex = new Dog("Rex");
        assertThat(rex.sound()).isEqualTo("Woof");
        assertThat(new Animal("Generic").sound()).as("Animal itself is unchanged").isEqualTo("...");
    }

    @Test
    void superCallKeepsBaseBehaviour() {
        assertThat(new Dog("Rex").describe()).isEqualTo("Rex says Woof and wags its tail");
        assertThat(new Animal("Generic").describe()).isEqualTo("Generic says ...");
    }
}
