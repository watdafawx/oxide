package oxide.the_parents_version;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class DogTest {
    @Test
    void dogPassesItsNameUp() {
        assertThat(new Dog("Rex").name()).isEqualTo("Rex");
        assertThat(new Dog("Fido").name()).isEqualTo("Fido");
    }

    @Test
    void describeExtendsTheParentText() {
        assertThat(new Dog("Rex").describe()).isEqualTo("Rex says Woof and wags its tail");
        assertThat(new Animal("Generic").describe()).as("Animal itself is unchanged").isEqualTo("Generic says ...");
    }
}
