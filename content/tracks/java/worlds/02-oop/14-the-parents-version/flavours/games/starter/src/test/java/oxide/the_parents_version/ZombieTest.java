package oxide.the_parents_version;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ZombieTest {
    @Test
    void zombieStartsWith20Hp() {
        Zombie bob = new Zombie("Bob");
        assertThat(bob.name()).isEqualTo("Bob");
        assertThat(bob.health()).isEqualTo(20);
    }

    @Test
    void zombieDescriptionKeepsMobPrefix() {
        assertThat(new Zombie("Bob").describe()).isEqualTo("Bob (20 hp) burns in daylight");
        assertThat(new Mob("Cow", 10).describe()).as("Mob itself is unchanged").isEqualTo("Cow (10 hp)");
    }
}
