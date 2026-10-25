package oxide.a_switch_that_does;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class PlayerTest {
    @Test
    void healAndHurtMoveHealth() {
        Player steve = new Player();
        steve.apply(new Hurt(7));
        assertThat(steve.health()).isEqualTo(13);
        steve.apply(new Heal(4));
        assertThat(steve.health()).isEqualTo(17);
    }

    @Test
    void respawnRestoresHealthAndIsCounted() {
        Player steve = new Player();
        steve.apply(new Hurt(20));
        steve.apply(new Respawn());
        assertThat(steve.health()).isEqualTo(20);
        steve.apply(new Respawn());
        assertThat(steve.respawns()).isEqualTo(2);
    }
}
