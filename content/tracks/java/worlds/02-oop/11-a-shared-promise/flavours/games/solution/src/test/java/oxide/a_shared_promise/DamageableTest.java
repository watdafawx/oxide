package oxide.a_shared_promise;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class DamageableTest {
    @Test
    void zombieLosesHealthWhenDamaged() {
        Damageable zombie = new Zombie();
        assertThat(zombie.name()).isEqualTo("Zombie");
        assertThat(zombie.health()).isEqualTo(20);
        zombie.damage(7);
        assertThat(zombie.health()).isEqualTo(13);
        zombie.damage(50);
        assertThat(zombie.health()).as("never below 0").isZero();
    }

    @Test
    void armorStandTakesHalfDamage() {
        Damageable stand = new ArmorStand();
        assertThat(stand.name()).isEqualTo("Armor stand");
        assertThat(stand.health()).isEqualTo(6);
        stand.damage(5);
        assertThat(stand.health()).as("half of 5, rounded down, is 2").isEqualTo(4);
        stand.damage(100);
        assertThat(stand.health()).isZero();
    }
}
