package oxide.interfaces;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class DamageableTest {
    @Test
    void zombieLosesHealthWhenDamaged() {
        Damageable zombie = new Zombie();
        assertThat(zombie.health()).isEqualTo(20);
        zombie.damage(7);
        assertThat(zombie.health()).isEqualTo(13);
        zombie.damage(100);
        assertThat(zombie.health()).as("health never goes below 0").isZero();
    }

    @Test
    void armorStandTakesHalfDamage() {
        Damageable stand = new ArmorStand();
        assertThat(stand.health()).isEqualTo(6);
        stand.damage(4);
        assertThat(stand.health()).isEqualTo(4);
        stand.damage(100);
        assertThat(stand.health()).isZero();
    }

    @Test
    void defaultStatusUsesNameAndHealth() {
        Damageable zombie = new Zombie();
        assertThat(zombie.status()).isEqualTo("Zombie: 20 hp");
        zombie.damage(20);
        assertThat(zombie.status()).isEqualTo("Zombie: dead");
        assertThat(new ArmorStand().status()).isEqualTo("Armor stand: 6 hp");
    }
}
