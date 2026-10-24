package oxide.set_once;

import static org.assertj.core.api.Assertions.assertThat;

import java.lang.reflect.Modifier;
import org.junit.jupiter.api.Test;

class MobTest {
    @Test
    void typeIsFinal() throws Exception {
        int modifiers = Mob.class.getDeclaredField("type").getModifiers();
        assertThat(Modifier.isFinal(modifiers)).as("write `private final String type;`").isTrue();
    }

    @Test
    void hurtLowersHealth() throws Exception {
        Mob zombie = new Mob("zombie", 20);
        zombie.hurt(7);
        zombie.hurt(3);
        assertThat(zombie.health()).isEqualTo(10);
        int modifiers = Mob.class.getDeclaredField("health").getModifiers();
        assertThat(Modifier.isFinal(modifiers)).as("health changes, so it is not final").isFalse();
    }
}
