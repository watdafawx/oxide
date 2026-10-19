package oxide.inheritance_and_super;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ZombieTest {
    @Test
    void zombieGroansInsteadOfSilence() {
        Mob bob = new Zombie("Bob");
        assertThat(bob.ambientSound()).isEqualTo("Rrrgh");
        assertThat(new Mob("Cow", 10).ambientSound()).as("Mob itself is unchanged").isEqualTo("silence");
    }

    @Test
    void zombieDescriptionKeepsMobPrefix() {
        assertThat(new Zombie("Bob").describe()).isEqualTo("Bob (20 hp) burns in daylight");
        assertThat(new Mob("Cow", 10).describe()).isEqualTo("Cow (10 hp)");
    }
}
