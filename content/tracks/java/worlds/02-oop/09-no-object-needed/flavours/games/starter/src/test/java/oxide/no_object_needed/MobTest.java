package oxide.no_object_needed;

import static org.assertj.core.api.Assertions.assertThat;

import java.lang.reflect.Modifier;
import org.junit.jupiter.api.Test;

class MobTest {
    @Test
    void spawnBuildsNamedMob() {
        Mob mob = Mob.spawn(" Zombie ");
        assertThat(mob).isNotNull();
        assertThat(mob.kind()).isEqualTo("zombie");
    }

    @Test
    void constructorIsPrivate() throws Exception {
        int modifiers = Mob.class.getDeclaredConstructor(String.class).getModifiers();
        assertThat(Modifier.isPrivate(modifiers)).as("write `private Mob(String kind)`").isTrue();
    }
}
