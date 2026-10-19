package oxide.static_members;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class MobTest {
    @Test
    void spawnBuildsNamedMob() {
        Mob zombie = Mob.spawn("zombie");
        assertThat(zombie).isNotNull();
        assertThat(zombie.kind()).isEqualTo("zombie");
    }

    @Test
    void everyMobGetsItsOwnEntityId() {
        Mob a = Mob.spawn("zombie");
        Mob b = Mob.spawn("zombie");
        Mob c = Mob.spawn("creeper");
        assertThat(a.id()).isPositive();
        assertThat(b.id()).isEqualTo(a.id() + 1);
        assertThat(c.id()).isEqualTo(b.id() + 1);
    }
}
