package oxide.is_a;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ZombieTest {
    @Test
    void zombieGroansInsteadOfSilence() {
        Mob bob = new Zombie("Bob");
        assertThat(bob.ambientSound()).isEqualTo("Rrrgh");
        assertThat(bob.health()).as("Zombie got health() from Mob").isEqualTo(20);
        assertThat(new Mob("Cow", 10).ambientSound()).as("Mob itself is unchanged").isEqualTo("silence");
    }

    @Test
    void parentCodeUsesTheZombiesSound() {
        assertThat(new Zombie("Bob").speak()).isEqualTo("Bob says Rrrgh");
        assertThat(new Mob("Cow", 10).speak()).isEqualTo("Cow says silence");
    }
}
