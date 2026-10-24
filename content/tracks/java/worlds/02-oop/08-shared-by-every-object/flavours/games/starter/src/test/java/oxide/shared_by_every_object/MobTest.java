package oxide.shared_by_every_object;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class MobTest {
    @Test
    void mobTakesTheCounter() {
        Mob zombie = new Mob("zombie");
        assertThat(zombie.kind()).isEqualTo("zombie");
        assertThat(zombie.id()).as("entity ids start at 1").isPositive();
    }

    @Test
    void everyMobGetsItsOwnEntityId() {
        Mob a = new Mob("zombie");
        Mob b = new Mob("zombie");
        Mob c = new Mob("skeleton");
        assertThat(a.id()).isPositive();
        assertThat(b.id()).as("the counter is shared, so b comes right after a").isEqualTo(a.id() + 1);
        assertThat(c.id()).isEqualTo(b.id() + 1);
    }
}
