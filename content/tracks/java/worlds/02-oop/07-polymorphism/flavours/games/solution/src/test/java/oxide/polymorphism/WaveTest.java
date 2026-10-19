package oxide.polymorphism;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class WaveTest {
    /** A mob type that Wave has never heard of. */
    private static class Warden extends Mob {
        Warden() {
            super("Warden");
        }

        @Override
        public int attackDamage() {
            return 30;
        }
    }

    @Test
    void eachMobTypeDefinesItsOwnDamage() {
        assertThat(new Zombie("Bob").attackDamage()).isEqualTo(3);
        assertThat(new Skeleton("Bones", 4).attackDamage()).isEqualTo(8);
    }

    @Test
    void waveAddsUpEveryMobIncludingNewOnes() {
        List<Mob> wave = List.of(new Zombie("Bob"), new Skeleton("Bones", 4), new Warden());
        assertThat(Wave.totalDamage(wave)).isEqualTo(3 + 8 + 30);
        assertThat(Wave.totalDamage(List.of())).isZero();
    }
}
