package oxide.a_half_finished_parent;

import static org.assertj.core.api.Assertions.assertThat;

import java.lang.reflect.Modifier;
import org.junit.jupiter.api.Test;

class MobTest {
    @Test
    void zombieHitsForThree() {
        Mob zombie = new Zombie();
        assertThat(zombie.attackDamage()).isEqualTo(3);
        assertThat(Modifier.isAbstract(Mob.class.getModifiers())).as("Mob stays abstract").isTrue();
    }

    @Test
    void skeletonHitsTwoPerArrow() {
        Mob skeleton = new Skeleton(3);
        assertThat(skeleton.attackDamage()).isEqualTo(6);
        assertThat(skeleton.name()).isEqualTo("Skeleton");
    }
}
