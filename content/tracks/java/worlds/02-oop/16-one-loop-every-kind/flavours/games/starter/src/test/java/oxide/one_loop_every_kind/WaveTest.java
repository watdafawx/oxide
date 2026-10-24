package oxide.one_loop_every_kind;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class WaveTest {
    /** A mob that Wave has never heard of. */
    static class Creeper extends Mob {
        Creeper() {
            super("Creeper");
        }

        @Override
        public int attackDamage() {
            return 10;
        }
    }

    @Test
    void waveSumsAMixedNight() {
        Mob[] night = {new Zombie(), new Skeleton(3), new Zombie()};
        assertThat(Wave.totalDamage(night)).isEqualTo(3 + 6 + 3);
        assertThat(Wave.totalDamage(new Mob[0])).as("a quiet night").isZero();
    }

    @Test
    void waveCountsAMobItHasNeverSeen() {
        Mob[] night = {new Zombie(), new Creeper()};
        assertThat(Wave.totalDamage(night)).isEqualTo(3 + 10);
    }
}
