package oxide.a_default_body;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class DamageableTest {
    /** A damageable thing the interface has never heard of. */
    static class Boat implements Damageable {
        private int health = 4;

        @Override
        public String name() {
            return "Boat";
        }

        @Override
        public int health() {
            return health;
        }

        @Override
        public void damage(int amount) {
            health = Math.max(0, health - amount);
        }
    }

    @Test
    void statusUsesNameAndHealth() {
        Zombie zombie = new Zombie();
        assertThat(zombie.status()).isEqualTo("Zombie: 20 hp");
        zombie.damage(20);
        assertThat(zombie.status()).isEqualTo("Zombie: dead");
        assertThat(new ArmorStand().status()).isEqualTo("Armor stand: 6 hp");
    }

    @Test
    void newMobsGetStatusForFree() {
        Boat boat = new Boat();
        assertThat(boat.status()).isEqualTo("Boat: 4 hp");
        boat.damage(9);
        assertThat(boat.status()).isEqualTo("Boat: dead");
    }
}
